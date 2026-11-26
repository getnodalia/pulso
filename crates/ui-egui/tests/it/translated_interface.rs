// SPDX-License-Identifier: MIT OR Apache-2.0
//! Every screen draws its text in the interface language. Each surface (a panel, a Settings page,
//! a dialog) is drawn in English and in Spanish; a text that reads the same in both although the
//! Spanish catalog has a row for it was drawn without going through the catalog.

use std::collections::{BTreeMap, BTreeSet};

use egui::epaint::Shape;
use egui_kittest::Harness;
use egui_kittest::kittest::NodeT as _;
use pulso_engine::Session;
use pulso_engine::menus::{MenuNode, TREE, parse};
use pulso_ui_egui::PulsoApp;
use pulso_ui_egui::dock::PanelKind;
use pulso_ui_egui::i18n::spanish_row;
use serde_json::{Value, json};

fn session() -> Session {
    let mut s = Session::default();
    s.execute("comp.new", json!({"name": "Main", "width": 320, "height": 180, "duration": 4})).unwrap();
    s.execute("layer.newSolid", json!({"name": "Plate A", "color": "#c04040"})).unwrap();
    s.execute("layer.newSolid", json!({"name": "Plate B", "color": "#4040c0"})).unwrap();
    s.execute("effect.apply", json!({"effect": "Gaussian Blur"})).ok();
    s
}

fn harness(language: &str) -> Harness<'static, PulsoApp> {
    let language = language.to_owned();
    let mut h = Harness::builder().with_size(egui::vec2(1600.0, 1000.0)).with_accessibility_check(false).build_eframe(move |_| {
        let mut app = PulsoApp::new(session());
        app.session.prefs.general.language = language;
        // A file dialog the command asks for is never answered here.
        app.hooks.file_dialog_async = Some(Box::new(|_, _| {}));
        app
    });
    h.run_steps(4);
    h
}

fn collect_shape(shape: &Shape, out: &mut BTreeMap<String, String>) {
    match shape {
        Shape::Text(t) => {
            for line in t.galley.text().split('\n') {
                let line = line.trim();
                if !line.is_empty() {
                    out.entry(line.to_owned()).or_insert_with(|| "painted".to_owned());
                }
            }
        }
        Shape::Vec(v) => v.iter().for_each(|s| collect_shape(s, out)),
        _ => {}
    }
}

/// The names the person (or a command that made something for them) gave: items of the project and
/// layers. They are data, drawn as they are.
fn own_names(h: &Harness<'_, PulsoApp>) -> BTreeSet<String> {
    let session = &h.state().session;
    let mut names: BTreeSet<String> = session.project.items.values().map(|i| i.name.clone()).collect();
    for item in session.project.items.values() {
        if let Some(comp) = item.as_comp() {
            names.extend(comp.layers.iter().map(|l| l.name.clone()));
        }
    }
    // The names of the labels are preferences the person can rewrite (Settings ▸ Labels).
    names.extend(pulso_engine::color::Label::ALL.iter().map(|l| l.name().to_string()));
    names
}

/// Everything the frame names for assistive technology or paints as text, with how it is shown.
fn shown(h: &Harness<'_, PulsoApp>) -> BTreeMap<String, String> {
    let own = own_names(h);
    let mut out = BTreeMap::new();
    for node in h.root().children_recursive() {
        let n = node.accesskit_node();
        // What a person typed into a field is theirs, not the interface's.
        let is_field = |role| matches!(role, egui::accesskit::Role::TextInput | egui::accesskit::Role::MultilineTextInput);
        if n.parent().is_some_and(|p| is_field(p.role())) {
            continue;
        }
        let value = if is_field(n.role()) { None } else { n.value() };
        for text in [n.label(), value].into_iter().flatten() {
            let text = text.trim();
            if !text.is_empty() {
                out.entry(text.to_owned()).or_insert_with(|| {
                    // Where it comes from, for the report: the role, and the node it sits in.
                    let parent = n.parent().map(|p| format!(" in {:?} {:?}", p.role(), p.label().unwrap_or_default())).unwrap_or_default();
                    format!("{:?}{parent}", n.role())
                });
            }
        }
    }
    for clipped in &h.output().shapes {
        collect_shape(&clipped.shape, &mut out);
    }
    out.retain(|text, _| !own.contains(text));
    out
}

fn invoke(h: &mut Harness<'_, PulsoApp>, id: &str, params: Value) {
    let ctx = h.ctx.clone();
    let _ = pulso_ui_egui::menus::invoke(h.state_mut(), &ctx, id, params);
    h.run_steps(3);
}

/// A surface: a name and what puts the app on it.
type Setup = Box<dyn Fn(&mut Harness<'static, PulsoApp>)>;

fn surfaces() -> Vec<(String, Setup)> {
    let mut v: Vec<(String, Setup)> = Vec::new();
    v.push(("main window".into(), Box::new(|_| {})));
    for p in PanelKind::ALL {
        v.push((
            format!("panel {}", p.id()),
            Box::new(move |h| {
                h.state_mut().show_panel(p);
                h.run_steps(3);
            }),
        ));
    }
    for page in pulso_engine::prefs::pages() {
        let id = page.id;
        v.push((format!("settings {id}"), Box::new(move |h| invoke(h, "app.settings", json!({"page": id})))));
    }
    v
}

/// The commands behind the menu bar, each with the parameters its entry binds.
fn menu_commands() -> Vec<(String, Value)> {
    fn walk(nodes: &[MenuNode], out: &mut Vec<(String, Value)>) {
        for n in nodes {
            match n {
                MenuNode::Item(e) => {
                    if !out.iter().any(|(c, p)| *c == e.command && *p == e.params) {
                        out.push((e.command.clone(), e.params.clone()));
                    }
                }
                MenuNode::Submenu { children, .. } => walk(children, out),
                _ => {}
            }
        }
    }
    let mut out = Vec::new();
    walk(&parse(TREE, false).unwrap(), &mut out);
    out
}

/// Commands that end the session or leave the window to something else.
fn skipped(command: &str) -> bool {
    command.contains("quit") || command.contains("exit") || command.starts_with("file.close") || command.starts_with("app.close")
}

/// Texts that read the same in both languages on purpose or that are still to be found.
///
/// - Defaults a person types over or later renames: the name a new solid or a custom RGB profile
///   starts with, the name of an action in the History panel.
/// - Names for assistive technology only (nothing on screen is English): the label some buttons and
///   combo boxes announce is still the English source. Each of these is a bug; take a row out of
///   this list when its draw site is found, never add one.
const KNOWN: &[&str] = &[
    "Analyze backward",
    "Analyze forward",
    "Cube",
    "Current Settings",
    "Delete",
    "Home",
    "My Custom RGB",
    "New Composition",
    "Project",
    "Render",
    "Revert",
    "Save",
    "Solid",
    "Timecode",
    "Undocked Panels",
];

/// Texts drawn in English in a Spanish interface although the Spanish catalog translates them.
fn untranslated(en: &BTreeMap<String, String>, es: &BTreeMap<String, String>) -> Vec<String> {
    en.iter()
        .filter(|(t, _)| es.contains_key(*t) && !KNOWN.contains(&t.as_str()) && spanish_row(t).is_some())
        .map(|(t, kind)| format!("{t:?} ({kind})"))
        .collect()
}

fn walk(report: bool) -> Vec<String> {
    let mut failures = Vec::new();
    let mut note = |name: &str, en: &BTreeMap<String, String>, es: &BTreeMap<String, String>| {
        if report {
            println!("== {name}");
            for (t, kind) in en.iter().filter(|(t, _)| es.contains_key(*t)) {
                println!("   {t:?} ({kind}){}", if spanish_row(t).is_some() { "   <- has a Spanish row" } else { "" });
            }
        } else {
            let bad = untranslated(en, es);
            if !bad.is_empty() {
                failures.push(format!("{name}: {bad:?}"));
            }
        }
    };
    for (name, setup) in surfaces() {
        let mut sets = Vec::new();
        for language in ["en", "es"] {
            let mut h = harness(language);
            setup(&mut h);
            sets.push(shown(&h));
        }
        note(&name, &sets[0], &sets[1]);
    }
    // Every command of the menu bar, in one window per language; what a command opens is read
    // while it is open.
    let commands = menu_commands();
    let mut per_language: Vec<Vec<Option<BTreeMap<String, String>>>> = Vec::new();
    for language in ["en", "es"] {
        let mut h = harness(language);
        let base = shown(&h);
        let mut drawn = Vec::new();
        for (command, params) in &commands {
            if skipped(command) {
                drawn.push(None);
                continue;
            }
            invoke(&mut h, command, params.clone());
            // What the window already shows is the main window's business.
            drawn.push(h.state().dialog.is_some().then(|| shown(&h).into_iter().filter(|(t, _)| !base.contains_key(t)).collect()));
            h.state_mut().dialog = None;
            h.run_steps(2);
        }
        per_language.push(drawn);
    }
    for (i, (command, _)) in commands.iter().enumerate() {
        if let (Some(en), Some(es)) = (&per_language[0][i], &per_language[1][i]) {
            note(&format!("command {command}"), en, es);
        }
    }
    failures
}

/// A screen that draws a text the catalog translates, without going through the catalog, shows
/// English to everyone who runs Pulso in Spanish.
#[test]
fn spanish_interface_has_no_untranslated_text() {
    let failures = walk(false);
    assert!(failures.is_empty(), "texts drawn in English although Spanish has them:\n{}", failures.join("\n"));
}

/// Prints every surface's texts that read the same in both languages:
/// `cargo test -p pulso-ui-egui --test it -- --ignored --nocapture untranslated_report`.
#[test]
#[ignore = "a report, not a check"]
fn untranslated_report() {
    walk(true);
}
