# Nodalia migration: Pulso

Category: secondary app.
Status: Base Nodalia preparada, as of 2026-11-13. Evidence: the pull request that adds this file.

## Current state

Pure-Rust motion-graphics app (egui), MIT OR Apache-2.0. Cargo git dependencies come from getnodalia/foundation and nodalia-design. The gate, FreeBSD,
Nix, packaging lint, AGENTS.md and git-sources checks call getnodalia/foundation's reusable
workflows pinned by commit. SonarCloud was removed; CodeQL and Scorecard remain.

## Desired state

No workflow, dependency or document depends on storytold; Pulso has its own release workflow; the
interface follows Foundation's [NDL standard](https://github.com/getnodalia/foundation/blob/main/standards/ndl.md).

## Inherited dependencies still pending

| Dependency | Where | Replacement | Blocked on |
|---|---|---|---|
| `storytold/craft-common-actions` | `.github/workflows/freebsd.yml`, `.github/workflows/windows-arm64.yml` | Foundation reusable workflows (release, refresh-contributors, windows-arm64) | writing and testing them |
| `storytold/pdfcraft` | `packaging/macos/package.sh` | to decide | — |
| `storytold/vectorcraft` | `packaging/macos/dmg/README.md`, `packaging/macos/package.sh` | to decide | — |

## Shared libraries

Done (2026-11-18): Pulso depends on the shared libraries under Nodalia's names only. The
`nodalia-*` crates come from `nodalia/libs`, Toma's media crates (`toma-*`) from `nodalia/toma`,
and the fonts build input from `nodalia/fonts` (`NODALIA_FONTS_DIR`), all in getnodalia/foundation
at the commits made from the upstream revisions Pulso pinned before. There is no upstream sync.

## Technical debt

- None from the rename: every name is Pulso's, internal ones included (crates `pulso-*`, binaries
  `pulso`, `pulso-cli` and `pulso-web`, app id `dev.danielmiguelt.Pulso`). There is no upstream sync and
  no rename tooling. File signatures keep their
  earlier names so files saved before the rename keep opening.

## Reusable Nodalia components

getnodalia/foundation (vendored crates, fonts, translations, reusable workflows);
getnodalia/nodalia-design (theme, icons).

## NDL

Pulso belongs to the creator suite. Done: the creator app bar (44 pt) with the main menu (About,
Preferences, Keyboard Shortcuts, Quit), Home and the shared workspace selector; panels as islands
in the window colour; the NDL palette with Pulso's accent; Nodalia icons where NDL has the glyph;
the theme selector only in Preferences; NDL's round window controls on Linux only (Preferences >
Appearance > Window Controls brings back the desktop's), native ones on macOS and Windows; the
menus exported over D-Bus to the shell's top bar on Linux (`apps/pulso/src/linux_menu.rs`); the
app icon (Eadweard Muybridge) in About.

Pending: Pulso's own icons that NDL has no glyph for yet (the After Effects-style tools and the
Timeline switches) move to nodalia-design as they are drawn there. Favourite themes are still
starred in Preferences > Appearance, but with the header's Appearance button gone nothing offers
them yet; they come back when the main menu gets a theme submenu, or the setting goes.

## Risks and priorities

1. Releases still run the inherited release workflow.
2. Windows ARM64 packaging still runs the inherited workflow.

## Recommended order

1. A Foundation release workflow, tested with a dry run, then Pulso's release on it.
2. A Foundation Windows ARM64 workflow.

## What the first agent should do

1. Read AGENTS.md and this file.
2. Pick the first item of "Recommended order".

## Releases

Pulso releases through `release-auto.yml` like every Nodalia app (Foundation's `release-tag` and
`release-publish`, with `release-linux.yml`, `release-macos.yml` and `release-windows.yml`). The
packages are named `pulso-X.Y.Z-…`.