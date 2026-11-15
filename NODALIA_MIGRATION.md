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
| `storytold/craft-libs` | `Cargo.toml`, `nix/package.nix` | to decide | — |
| `storytold/effectcraft` | `.github/workflows/upstream-sync.yml` | none: the intended upstream sync, renamed by `scripts/rebrand.py` | — | <!-- rebrand: keep -->
| `storytold/egui.git` | `xtask/src/lock_patches.rs` | to decide | — |
| `storytold/filmcraft` | `nix/package.nix` | to decide | — |
| `storytold/pdfcraft` | `packaging/macos/package.sh` | to decide | — |
| `storytold/vectorcraft` | `packaging/macos/dmg/README.md`, `packaging/macos/package.sh` | to decide | — |

## Technical debt

- None from the rename: every name is Pulso's, internal ones included (crates `pulso-*`, binaries
  `pulso`, `pulso-cli` and `pulso-web`, app id `dev.danielmiguelt.Pulso`). `scripts/rebrand.py`
  applies `.github/rebrand.toml` to upstream's tree in the weekly sync (`docs/upstream.md`), and the
  "No upstream names" check runs `python3 scripts/rebrand.py check`. File signatures keep their
  upstream names so files saved before the rename keep opening.

## Reusable Nodalia components

getnodalia/foundation (vendored crates, fonts, translations, reusable workflows);
getnodalia/nodalia-design (theme, icons).

## NDL work pending

Check the interface against Foundation's NDL standard: hamburger menu with About, Preferences and
Keyboard shortcuts; theme selector only in Preferences; NDL window controls on Linux only; global
menu exported over D-Bus on Linux.

## Risks and priorities

1. Releases still run the inherited release workflow.
2. Windows ARM64 packaging still runs the inherited workflow.

## Recommended order

1. A Foundation release workflow, tested with a dry run, then Pulso's release on it.
2. A Foundation Windows ARM64 workflow.
3. NDL review of the interface.

## What the first agent should do

1. Read AGENTS.md and this file.
2. Pick the first item of "Recommended order".

## Releases

Pulso releases through `release-auto.yml` like every Nodalia app (Foundation's `release-tag` and
`release-publish`, with `release-linux.yml`, `release-macos.yml` and `release-windows.yml`). The
packages are named `pulso-X.Y.Z-…`.