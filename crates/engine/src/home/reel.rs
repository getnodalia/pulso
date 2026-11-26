//! The demo project's "Earthrise" comp: a short title sequence over two public-domain NASA
//! photographs (Apollo 17's "Blue Marble" and Apollo 8's "Earthrise"). Slow push-ins, a
//! cross-dissolve, a letterbox, a vignette and a per-character title reveal, all built from
//! the same layers and properties a user would use. The pictures are embedded in the program
//! and written next to the settings when the demo opens, then imported like any footage
//! (credits in `ATTRIBUTION.md`).

use pulso_color::Label;
use pulso_keyframe::{Gradient, Justify, Keyframe, TextDoc, Value};
use pulso_project::build::{self, Ids};
use pulso_project::{Comp, Footage, ItemId, ItemKind, LayerSource, Project};
use pulso_time::{FrameRate, Tick};

use super::demo::{anim, contents, effect, hex, keys, keys_out, set, t, text_layer};
use crate::Session;

/// The comp's name.
pub const EARTHRISE: &str = "Earthrise";

/// How long the sequence runs, in seconds.
const SECONDS: f64 = 14.0;

/// The folder (inside the settings folder's footage area) the pictures are written to.
const FOLDER: &str = "Pulso Demo";

/// One embedded picture.
struct Art {
    file: &'static str,
    name: &'static str,
    bytes: &'static [u8],
}

const ART: [Art; 2] = [
    Art { file: "blue-marble.jpg", name: "Blue Marble (NASA, Apollo 17)", bytes: include_bytes!("../../assets/demo/blue-marble.jpg") },
    Art { file: "earthrise.jpg", name: "Earthrise (NASA, Apollo 8)", bytes: include_bytes!("../../assets/demo/earthrise.jpg") },
];

/// Write the embedded pictures where the media layer can read them and probe each one. `None`
/// when a host can't (no media layer, no writable storage): the demo then has no photo comp.
pub(crate) fn install(s: &Session) -> Option<Vec<Footage>> {
    let importer = s.importer.as_ref()?;
    let base = s
        .config
        .as_ref()
        .and_then(|c| c.dir())
        .map(|d| d.join(super::templates::FOOTAGE_DIR).join(FOLDER).to_string_lossy().into_owned())
        .unwrap_or_else(|| format!("/{}/{FOLDER}", super::templates::FOOTAGE_DIR));
    let mut out = Vec::with_capacity(ART.len());
    for a in &ART {
        let path = format!("{base}/{}", a.file);
        s.services.store_file(&path, a.bytes).ok()?;
        importer.register(&path, a.bytes);
        let f = importer.probe(&path).ok()?;
        if f.width == 0 || f.height == 0 {
            return None;
        }
        out.push(f);
    }
    Some(out)
}

/// Add the photos and the "Earthrise" comp to `p`; returns the comp.
pub(crate) fn add(p: &mut Project, footage: Vec<Footage>) -> Option<ItemId> {
    if footage.len() != ART.len() {
        return None;
    }
    let folder = p.add_item("Photographs", Label::Yellow, None, ItemKind::Folder);
    let mut ids = vec![];
    let mut sizes = vec![];
    for (a, f) in ART.iter().zip(footage) {
        sizes.push((f.width, f.height));
        ids.push(p.add_item(a.name, Label::Aqua, Some(folder), ItemKind::Footage(f)));
    }
    let comp = build_comp(p, (ids[0], sizes[0]), (ids[1], sizes[1]));
    let id = p.add_item(EARTHRISE, Label::Sandstone, None, ItemKind::Comp(comp.into()));
    p.fix_next_id();
    Some(id)
}

fn build_comp(p: &mut Project, marble: (ItemId, (u32, u32)), rise: (ItemId, (u32, u32))) -> Comp {
    let mut comp = Comp::new(1920, 1080, FrameRate::FPS_29_97, t(SECONDS));
    comp.background = [0.0, 0.0, 0.0];
    comp.work_area = (Tick::ZERO, t(SECONDS));

    // Fade from and to black.
    let sid =
        p.add_item("Black", Label::Red, None, ItemKind::Solid(pulso_project::Solid { color: [0.0, 0.0, 0.0], width: 1920, height: 1080, pixel_aspect: 1.0 }));
    let mut black = build::layer(p, &comp, "Fade", LayerSource::Solid { item: sid }, (1920, 1080), None);
    anim(
        &mut black,
        "transform/opacity",
        keys(&[(0.0, Value::Scalar(100.0)), (1.6, Value::Scalar(0.0)), (12.6, Value::Scalar(0.0)), (14.0, Value::Scalar(100.0))]),
    );

    // Letterbox: 2.39:1 bars, sliding in with the first seconds.
    let bar_h = 138.0;
    let mut bars = build::layer(p, &comp, "Letterbox", LayerSource::Shape, (1920, 1080), None);
    let mut next = p.next_id;
    {
        let mut ids = Ids(&mut next);
        let top = build::shape_rect(&mut ids, [1920.0, bar_h], [0.0, -(540.0 - bar_h / 2.0)], 0.0);
        let top_fill = build::shape_fill(&mut ids, hex("#000000"));
        let bottom = build::shape_rect(&mut ids, [1920.0, bar_h], [0.0, 540.0 - bar_h / 2.0], 0.0);
        let bottom_fill = build::shape_fill(&mut ids, hex("#000000"));
        let g1 = build::shape_group(&mut ids, "Top", vec![top, top_fill]);
        let g2 = build::shape_group(&mut ids, "Bottom", vec![bottom, bottom_fill]);
        contents(&mut bars, vec![g1, g2]);
    }
    p.next_id = next;
    set(&mut bars, "transform/position", Value::Vec3([960.0, 540.0, 0.0]));
    anim(&mut bars, "transform/scale", keys_out(&[(0.0, Value::Vec3([100.0, 140.0, 100.0])), (2.4, Value::Vec3([100.0, 100.0, 100.0]))]));

    // Credit line inside the bottom bar, then the title block above it.
    let credit = |text: &str, size: f64, at: f64, y: f64, tracking: f64, style: &str, fill: [f32; 4], p: &mut Project, comp: &Comp| {
        let mut l = text_layer(
            p,
            comp,
            text,
            TextDoc { text: text.into(), size, style: style.into(), tracking, fill, justify: Justify::Center, ..Default::default() },
            [960.0, y],
        );
        anim(
            &mut l,
            "transform/opacity",
            keys(&[(at, Value::Scalar(0.0)), (at + 1.0, Value::Scalar(100.0)), (12.0, Value::Scalar(100.0)), (12.8, Value::Scalar(0.0))]),
        );
        l
    };
    let credit_marble = {
        let mut l = text_layer(
            p,
            &comp,
            "Credit Blue Marble",
            TextDoc {
                text: "APOLLO 17  ·  7 DECEMBER 1972  ·  NASA".into(),
                size: 22.0,
                style: "Medium".into(),
                tracking: 320.0,
                fill: [0.78, 0.84, 1.0, 1.0],
                justify: Justify::Center,
                ..Default::default()
            },
            [960.0, 1018.0],
        );
        anim(
            &mut l,
            "transform/opacity",
            keys(&[(1.4, Value::Scalar(0.0)), (2.4, Value::Scalar(100.0)), (5.0, Value::Scalar(100.0)), (6.0, Value::Scalar(0.0))]),
        );
        l
    };
    let credit_rise = credit("APOLLO 8  ·  24 DECEMBER 1968  ·  NASA / BILL ANDERS", 22.0, 9.2, 1018.0, 320.0, "Medium", [0.78, 0.84, 1.0, 1.0], p, &comp);
    let tagline = credit("A PALE BLUE WORLD RISES OVER THE MOON", 30.0, 8.6, 690.0, 380.0, "Medium", [0.92, 0.95, 1.0, 1.0], p, &comp);

    // The title, rising letter by letter.
    let mut title = text_layer(
        p,
        &comp,
        "EARTHRISE",
        TextDoc {
            text: "EARTHRISE".into(),
            size: 132.0,
            style: "Bold".into(),
            tracking: 260.0,
            fill: [1.0, 1.0, 1.0, 1.0],
            justify: Justify::Center,
            ..Default::default()
        },
        [960.0, 600.0],
    );
    let mut next = p.next_id;
    {
        let mut ids = Ids(&mut next);
        let pos = build::text_anim_prop(&mut ids, "position").map(|mut pr| {
            pr.value = Value::Vec3([0.0, 70.0, 0.0]);
            pr
        });
        let op = build::text_anim_prop(&mut ids, "opacity").map(|mut pr| {
            pr.value = Value::Scalar(0.0);
            pr
        });
        let mut a = build::text_animator(&mut ids, "Rise", [pos, op].into_iter().flatten().collect());
        if let Some(sel) = a.group_mut("selectors/#1")
            && let Some(adv) = sel.sub_mut("advanced")
            && let Some(sh) = adv.get_mut("shape")
        {
            sh.value = Value::Enum(1);
        }
        if let Some(anims) = title.props.group_mut("text/animators") {
            anims.children.push(a.into());
        }
    }
    p.next_id = next;
    anim(&mut title, "text/animators/#1/selectors/#1/start", keys(&[(7.6, Value::Scalar(0.0)), (10.2, Value::Scalar(100.0))]));
    // The reveal ramps across the letters, so the first ones would show before it starts: keep
    // the layer hidden until then.
    anim(
        &mut title,
        "transform/opacity",
        keys(&[(7.4, Value::Scalar(0.0)), (7.6, Value::Scalar(100.0)), (12.0, Value::Scalar(100.0)), (12.8, Value::Scalar(0.0))]),
    );

    // A dark foot under the title and a soft vignette over everything.
    let mut foot = build::layer(p, &comp, "Foot", LayerSource::Shape, (1920, 1080), None);
    let mut next = p.next_id;
    {
        let mut ids = Ids(&mut next);
        let r = build::shape_rect(&mut ids, [1920.0, 560.0], [0.0, 0.0], 0.0);
        let gf = build::shape_gradient_fill(
            &mut ids,
            false,
            [0.0, -280.0],
            [0.0, 280.0],
            Gradient { colors: vec![(0.0, [0.0, 0.0, 0.0, 1.0]), (1.0, [0.0, 0.0, 0.0, 1.0])], opacities: vec![(0.0, 0.0), (1.0, 0.72)] },
        );
        let g = build::shape_group(&mut ids, "Foot", vec![r, gf]);
        contents(&mut foot, vec![g]);
    }
    p.next_id = next;
    set(&mut foot, "transform/position", Value::Vec3([960.0, 800.0, 0.0]));
    anim(&mut foot, "transform/opacity", keys(&[(6.4, Value::Scalar(0.0)), (8.4, Value::Scalar(100.0))]));

    let mut vignette = build::layer(p, &comp, "Vignette", LayerSource::Shape, (1920, 1080), None);
    let mut next = p.next_id;
    {
        let mut ids = Ids(&mut next);
        let r = build::shape_rect(&mut ids, [1920.0, 1080.0], [0.0, 0.0], 0.0);
        let gf = build::shape_gradient_fill(
            &mut ids,
            true,
            [0.0, 0.0],
            [1180.0, 0.0],
            Gradient { colors: vec![(0.0, [0.0, 0.0, 0.0, 1.0]), (1.0, [0.0, 0.0, 0.0, 1.0])], opacities: vec![(0.0, 0.0), (0.55, 0.0), (1.0, 0.62)] },
        );
        let g = build::shape_group(&mut ids, "Vignette", vec![r, gf]);
        contents(&mut vignette, vec![g]);
    }
    p.next_id = next;
    set(&mut vignette, "transform/position", Value::Vec3([960.0, 540.0, 0.0]));

    // Earthrise: pushes in slowly after the dissolve.
    let (rise_item, rise_size) = rise;
    let mut rise_layer = build::layer(p, &comp, "Earthrise", LayerSource::Footage { item: rise_item }, rise_size, None);
    // Scale in percent of the picture's own pixels: from just covering the frame to a slow push
    // of a tenth more. The position keeps the Earth in the upper right and the horizon clear of
    // the letterbox as the camera creeps in.
    let fit = 1920.0 / rise_size.0.max(1) as f64 * 100.0;
    let (s0, s1) = (fit, fit * 1.08);
    anim(
        &mut rise_layer,
        "transform/scale",
        vec![Keyframe::new(t(5.2), Value::Vec3([s0, s0, 100.0])), Keyframe::new(t(SECONDS), Value::Vec3([s1, s1, 100.0]))],
    );
    anim(
        &mut rise_layer,
        "transform/position",
        vec![Keyframe::new(t(5.2), Value::Vec3([985.0, 480.0, 0.0])), Keyframe::new(t(SECONDS), Value::Vec3([950.0, 480.0, 0.0]))],
    );
    anim(&mut rise_layer, "transform/opacity", keys(&[(5.6, Value::Scalar(0.0)), (7.0, Value::Scalar(100.0))]));

    // Blue Marble: fills the first half, pushing in, then dissolves away.
    let (marble_item, marble_size) = marble;
    let mut marble_layer = build::layer(p, &comp, "Blue Marble", LayerSource::Footage { item: marble_item }, marble_size, None);
    let fit_m = 1160.0 / marble_size.1.max(1) as f64 * 100.0;
    anim(
        &mut marble_layer,
        "transform/scale",
        vec![Keyframe::new(t(0.0), Value::Vec3([fit_m * 0.9, fit_m * 0.9, 100.0])), Keyframe::new(t(7.0), Value::Vec3([fit_m * 1.1, fit_m * 1.1, 100.0]))],
    );
    set(&mut marble_layer, "transform/position", Value::Vec3([960.0, 540.0, 0.0]));
    anim(&mut marble_layer, "transform/opacity", keys(&[(5.0, Value::Scalar(100.0)), (6.2, Value::Scalar(0.0))]));
    marble_layer.out_point = t(6.4);
    // The photograph's own sky is a very dark grey, not black: bring it to black so the edge of
    // the picture doesn't show.
    effect(p, &mut marble_layer, "ec.color.levels", [marble_size.0 as f64, marble_size.1 as f64], &[("inBlack", Value::Scalar(0.05))]);

    // Top first.
    comp.layers = vec![black, credit_marble, credit_rise, tagline, title, bars, vignette, foot, rise_layer, marble_layer].into();
    comp
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use serde_json::json;

    use super::*;
    use crate::{Importer, Session};

    /// Probes still pictures from the file, as the media layer does.
    struct StillImporter;
    impl Importer for StillImporter {
        fn probe(&self, path: &str) -> Result<Footage, String> {
            let (width, height) = image::image_dimensions(path).map_err(|e| e.to_string())?;
            Ok(Footage { path: path.into(), width, height, ..Default::default() })
        }
    }

    /// The embedded pictures are the ones the attribution lists: JPEGs of the recorded size.
    #[test]
    fn embedded_pictures_decode() {
        let sizes: Vec<(u32, u32)> = ART.iter().map(|a| image::load_from_memory(a.bytes).map(|i| (i.width(), i.height())).unwrap()).collect();
        assert_eq!(sizes, [(1399, 1400), (1600, 1067)]);
        let attribution = include_str!("../../../../ATTRIBUTION.md");
        for a in &ART {
            assert!(attribution.contains(&format!("crates/engine/assets/demo/{}", a.file)), "{} has no attribution row", a.file);
        }
    }

    /// With a media layer the demo opens on the photo title sequence at the title's reveal.
    #[test]
    fn demo_opens_on_the_photo_title_sequence() {
        let root = std::env::temp_dir().join(format!("pulso-reel-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let mut s = Session {
            config: Some(Arc::new(crate::config::DirConfig::new(root.join("config")))),
            importer: Some(Arc::new(StillImporter)),
            ..Default::default()
        };
        s.execute("file.openDemoProject", json!({})).unwrap();
        let reel = s.project.find_by_name(EARTHRISE).map(|i| i.id).expect("the Earthrise comp");
        assert_eq!(s.active_comp_id(), Some(reel));
        // The playhead sits on a frame near the reveal (the comp's frame rate may not hit 10.8 exactly).
        assert!((s.time().0 - Tick::from_seconds_f64(10.8).0).abs() < Tick::from_seconds_f64(0.05).0);
        let footage: Vec<_> = s.project.items.values().filter_map(|i| if let ItemKind::Footage(f) = &i.kind { Some(f) } else { None }).collect();
        assert_eq!(footage.len(), 2);
        for f in &footage {
            assert!(std::path::Path::new(&f.path).is_file(), "{}", f.path);
        }
        let comp = s.project.comp(reel).expect("comp");
        let used = comp.layers.iter().filter(|l| matches!(l.source, LayerSource::Footage { .. })).count();
        assert_eq!(used, 2);
        // The procedural comps are still there.
        assert!(s.project.find_by_name(super::super::demo::MAIN_COMP).is_some());
        let img = s.render(reel, s.time(), pulso_render::RenderOpts { scale: 0.25, ..Default::default() });
        // The letterbox bars (two 138 px bands of 1080) are opaque even when no media decoder is present.
        let opaque = img.data.iter().filter(|p| p[3] > 0.99).count();
        assert!(opaque >= img.data.len() / 5, "{opaque} of {} pixels opaque ({}x{})", img.data.len(), img.width, img.height);
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Without a media layer (or when it can't read the pictures) the demo is the procedural one.
    #[test]
    fn demo_without_media_layer_is_procedural() {
        let mut s = Session::default();
        s.execute("file.openDemoProject", json!({})).unwrap();
        assert!(s.project.find_by_name(EARTHRISE).is_none());
        assert_eq!(s.active_comp_id(), s.project.find_by_name(super::super::demo::MAIN_COMP).map(|i| i.id));
    }
}
