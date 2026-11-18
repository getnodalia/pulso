//! Community and project links (shown in Help ▸, the About dialog and the start screen).

pub const APP_NAME: &str = "Pulso";
pub const APP_SLUG: &str = "pulso";
pub const WEBSITE: &str = "https://getnodalia.com";
pub const APP_PAGE: &str = "https://getnodalia.com/pulso";
pub const GITHUB: &str = "https://github.com/getnodalia/pulso";
pub const ISSUES: &str = "https://github.com/getnodalia/pulso/issues";

/// Pulso's companions in Nodalia's creator suite: (name, slug).
pub const SIBLINGS: &[(&str, &str)] = &[("Lienzo", "lienzo"), ("Pluma", "pluma"), ("Toma", "toma"), ("Ánima", "anima"), ("Trama", "trama"), ("Luz", "luz")];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/getnodalia/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://getnodalia.com/{slug}")
}
