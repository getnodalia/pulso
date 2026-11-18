<div align="center">
  <a href="https://getnodalia.com">
    <img src="https://raw.githubusercontent.com/danielmigueltejedor/nodalia-web/main/assets/brand/nodalia-wordmark-badge.svg" width="150" alt="Nodalia">
  </a>
  <br><br>
  <img src="https://raw.githubusercontent.com/danielmigueltejedor/pulso/main/assets/app-icon/hicolor/512x512/apps/dev.danielmiguelt.Pulso.png" width="192" alt="Pulso">
  <h1>Pulso</h1>
  <p><strong>The compositing and motion graphics tool of the Nodalia suite.</strong></p>

  <p>
    <img src="https://img.shields.io/badge/status-early%20stage-orange" alt="Early stage">
    <img src="https://img.shields.io/badge/Rust-native-dea584?logo=rust&logoColor=black" alt="Native Rust">
    <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2ea44f" alt="MIT OR Apache-2.0">
  </p>

  <p>
    <a href="#overview">Overview</a> ·
    <a href="#build-and-run">Build</a> ·
    <a href="#project-status">Status</a> ·
    <a href="#the-nodalia-suite">Suite</a> ·
    <a href="#license">License</a>
  </p>
</div>

---

## Overview

**Pulso** is the compositing and motion graphics tool of [Nodalia](https://getnodalia.com): a native app written in Rust,
with no account and no cloud. It runs on Linux, macOS and Windows, and ships 308 effects
of its own.

<p align="center">
  <img src="https://raw.githubusercontent.com/getnodalia/nodalia-web/main/assets/screens/pulso/pulso-dark.jpg" width="900" alt="The demo composition open in Pulso in the dark theme, with the viewer and the timeline">
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/getnodalia/nodalia-web/main/assets/screens/pulso/pulso-light.jpg" width="440" alt="The same composition in the light theme">
  <img src="https://raw.githubusercontent.com/getnodalia/nodalia-web/main/assets/screens/pulso/pulso-home-light.jpg" width="440" alt="Pulso's start screen">
</p>

> [!IMPORTANT]
> **Pulso is at an early stage.** Version 0.1.1 is the latest published build: download it from
> [Releases](https://github.com/getnodalia/pulso/releases). Expect rough edges.

## Build and run

You need a recent stable Rust toolchain.

```bash
git clone https://github.com/getnodalia/pulso.git
cd pulso
cargo run --release -p pulso
```

## Project status

| Area | State |
|---|---|
| Package and binary names | Done: the desktop binary is called `pulso` |
| App icon | Done: portrait of Eadweard Muybridge |
| Nodalia interface | Done: creator-suite app bar, islands and window controls; Pulso's own icons are still to come |
| Shared interface resources | Done: theme, icons and controls come from nodalia-design |
| Installable packages | 0.1.0: DMG, MSI, AppImage, deb, rpm and tar.gz |

## App icon

Nodalia app icons are black-and-white portraits of historical figures over the app colour.
Pulso's is Eadweard Muybridge, who studied motion frame by frame. Provenance and licence are in [`assets/app-icon/`](assets/app-icon/).

## The Nodalia suite

Pulso is one of the applications of [Nodalia](https://getnodalia.com), an independent software
project developed in public by [Daniel Miguel Tejedor](https://danielmigueltejedor.com).

## License

`MIT OR Apache-2.0`. See [`LICENSE-MIT`](LICENSE-MIT), [`LICENSE-APACHE`](LICENSE-APACHE) and
[`NOTICE`](NOTICE).
