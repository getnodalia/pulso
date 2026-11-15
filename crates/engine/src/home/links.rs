//! Community and project links (shown in Help ▸, the About dialog, the start screen and the
//! header's Discord button).

pub const APP_NAME: &str = "Pulso";
pub const APP_SLUG: &str = "pulso";
pub const DISCORD: &str = "https://github.com/getnodalia/pulso/issues";
pub const WEBSITE: &str = "https://getnodalia.com";
pub const APP_PAGE: &str = "https://getnodalia.com/pulso";
pub const GITHUB: &str = "https://github.com/getnodalia/pulso";
pub const ISSUES: &str = "https://github.com/getnodalia/pulso/issues";

/// Sibling apps of the Nodalia family: (name, what it is, GitHub repo, app page).
pub const SIBLINGS: &[(&str, &str)] =
    &[("Lienzo", "photocraft"), ("Pluma", "vectorcraft"), ("Toma", "filmcraft"), ("Luz", "lightcraft"), ("Papel", "pdfcraft"), ("Pliego", "designcraft")];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/storytold/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://getnodalia.com/apps/{slug}")
}
