//! A fully wired [`Session`]: footage decoding through `pulso-media` (Toma's codecs),
//! the media importer, the expression engine, JavaScript scripting (`pulso-script`) and
//! Render Queue export through `pulso-export` (Toma's encoders). Frontends (desktop,
//! CLI, MCP, web) start here.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use std::sync::Arc;

use pulso_engine::project::render_queue::OutputFormat;
use pulso_engine::{ExportJob, ExportResult, Exporter, Importer, Session};
use pulso_project::Footage;
pub use pulso_script as script;

pub mod app_dirs;

struct MediaImporter;

impl Importer for MediaImporter {
    fn probe(&self, path: &str) -> Result<Footage, String> {
        pulso_media::probe(path).map_err(|e| e.to_string())
    }
}

/// Render Queue export via `pulso-export`: to the file system, or to `sink` (the web app
/// turns written files into downloads).
#[derive(Default)]
pub struct FileExporter {
    pub sink: Option<Arc<pulso_export::Sink>>,
}

impl Exporter for FileExporter {
    fn formats(&self) -> Vec<OutputFormat> {
        pulso_export::available_formats()
    }
    fn supports_skip_existing(&self, settings: &pulso_project::render_queue::RenderSettings) -> bool {
        // The exporter's existence check precedes storage-overflow redirection.
        self.sink.is_none() && !cfg!(target_arch = "wasm32") && !settings.storage_overflow
    }
    fn export(&self, job: &ExportJob, progress: &mut dyn FnMut(u64, u64) -> bool) -> Result<ExportResult, String> {
        pulso_engine::render::passes::block_on(self.export_async(job, progress))
    }

    fn export_async<'a>(
        &'a self,
        job: &'a ExportJob<'a>,
        progress: &'a mut dyn FnMut(u64, u64) -> bool,
    ) -> pulso_engine::render_queue::LocalFuture<'a, Result<ExportResult, String>> {
        Box::pin(async move {
            let j = pulso_export::Job {
                project: job.project,
                footage: job.footage,
                expr: job.expr,
                accel: job.accel,
                comp: job.item.comp,
                settings: &job.item.settings,
                output: &job.item.output,
                path: job.path,
                sink: self.sink.as_deref(),
                options: pulso_export::JobOptions {
                    log: job.item.log,
                    label: job.label.clone(),
                    storage: job.storage,
                    overflow: job.project.render_prefs.overflow_folders.clone(),
                    caches: job.caches.map(|c| pulso_export::JobCaches { gpu: &c.gpu, cpu: &c.cpu }),
                    renderer: job.renderer,
                    gpu_frames: job.gpu_frames,
                },
                nested_switches: job.nested_switches,
            };
            match pulso_export::export_async(&j, &mut |p| progress(p.done, p.total)).await {
                Ok(r) => Ok(ExportResult {
                    path: r.path,
                    frames: r.frames,
                    width: r.width,
                    height: r.height,
                    bytes: r.bytes,
                    seconds: r.seconds,
                    audio: r.audio,
                    log: r.log,
                    overflow: r.overflow,
                }),
                Err(pulso_export::ExportError::Cancelled) => Err(pulso_engine::render_queue::CANCELLED.into()),
                Err(e) => Err(e.to_string()),
            }
        })
    }
}

/// A new session with media, import, expressions, scripting and export enabled.
pub fn session() -> Session {
    let mut s = Session {
        // The desktop app, the CLI and the MCP server share installed Roto Brush models.
        models_dir: config_dir().map(|d| d.join("models")),
        exporter: Some(Arc::new(FileExporter::default())),
        footage: Arc::new(pulso_media::MediaPool::new()),
        importer: Some(Arc::new(MediaImporter)),
        expr: Some(Arc::new(pulso_expr::Expressions)),
        expr_check: Some(pulso_expr::check_syntax),
        plugin_loader: pulso_plugin::wasm_available().then_some(pulso_plugin::loader as pulso_engine::PluginLoader),
        helpers: helper_hooks(),
        plugin_previewer: pulso_plugin::wasm_available().then_some(pulso_plugin::preview as pulso_engine::PluginPreviewer),
        plugins_dir: config_dir().map(|d| d.join("Plug-ins")),
        ..Default::default()
    };
    pulso_script::install(&mut s);
    s
}

/// External effect helpers run as child processes: native builds only.
#[cfg(not(target_arch = "wasm32"))]
fn helper_hooks() -> Option<pulso_engine::HelperHooks> {
    use pulso_helper_host as hh;
    // Effects that keep stopping their helper are remembered in the settings folder.
    if let Some(dir) = config_dir() {
        hh::blocklist::set_state_dir(dir);
    }
    fn scan(retry: bool) -> serde_json::Value {
        hh::scan_paths(&hh::default_search_paths(config_dir().as_deref()), retry)
    }
    Some(pulso_engine::HelperHooks { scan, load: hh::load_path, list: hh::list })
}

#[cfg(target_arch = "wasm32")]
fn helper_hooks() -> Option<pulso_engine::HelperHooks> {
    None
}

/// Stop the external effect helpers (when the app quits).
pub fn stop_helpers() {
    #[cfg(not(target_arch = "wasm32"))]
    pulso_helper_host::stop_all();
}

/// The platform config directory for Pulso (`PULSO_CONFIG_DIR` overrides):
/// `~/Library/Application Support/Pulso` (macOS), `%APPDATA%\Pulso` (Windows),
/// `$XDG_CONFIG_HOME/pulso` or `~/.config/pulso` (Linux and others). A `portable.txt`
/// beside the executable puts it in `PulsoData` there instead ([`app_dirs`]).
pub fn config_dir() -> Option<std::path::PathBuf> {
    app_dirs::current().dir.clone()
}

#[cfg(test)]
mod tests;
