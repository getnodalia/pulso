// SPDX-License-Identifier: MIT OR Apache-2.0
//! English texts the engine hands the interface (layer style names, template summaries, primitive
//! names). The panels draw them through `tr`, so each needs a row in the panel catalogs, and this
//! list is where the source shows the row is used.

/// The texts, in the order the engine names them.
#[allow(dead_code)] // read only as source text by the `CONVERTED` check
pub(crate) const ENGINE_TEXT: &[&str] =
    &["Best", "Bevel and Emboss", "Color Overlay", "Cube", "Drop Shadow", "Gradient Overlay", "Inner Glow", "Inner Shadow", "Outer Glow", "Quality", "Satin"];
