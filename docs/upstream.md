# Pulso upstream

| Field | Value |
|---|---|
| Upstream | `storytold/effectcraft` |
| Last observed | `feaa1138d3aa9f5d8c8946fd94b4377d64b45670` |
| Last evaluated | `feaa1138d3aa9f5d8c8946fd94b4377d64b45670` |
| Last integrated | `feaa1138d3aa9f5d8c8946fd94b4377d64b45670` merged on 2026-11-10 |
| Skipped | none in the baseline import |
| Adapted | desktop binary name `pulso` |
| Conflicting | `README.md` (kept ours), `AGENTS.md` (upstream plus our quality contract) |

Every name is Pulso's, internal ones included: crates are `pulso-*`, the binaries `pulso`,
`pulso-cli` and `pulso-web`, and the app id `dev.danielmiguelt.Pulso`. File signatures stay as
upstream wrote them (animation presets tagged `effectcraftAnimationPreset`, the bundled preset
libraries' `effectcraft.animationPresets/1`, motion graphics templates' `effectcraft-template`, and
the Rec.709 profile name in exported After Effects projects), so files saved before the rename
keep opening. The media crates vendored from Toma's upstream keep their `filmcraft-*` names.

## How the sync works

Upstream's code arrives renamed: `scripts/rebrand.py` applies `.github/rebrand.toml` (EffectCraft
becomes Pulso, internal names included) to upstream's tree. Then getnodalia/foundation's
`scripts/nodalia_libs.py app` renames the shared libraries upstream depends on (`craft-*` becomes
`nodalia-*`, Toma's `filmcraft-*` crates become `toma-*`) and moves each pin to the Nodalia
commit made from the same upstream commit; the sync stops if that commit does not exist yet (run
foundation's Nodalia libraries workflow for it). The weekly sync
(`.github/workflows/upstream-sync.yml`) records the result on the `upstream-pulso` branch. Each
commit there has the previous mirror commit and the upstream commit it renames as parents, and
names that commit in an `Upstream-Commit:` trailer. The sync merges the newest mirror commit
into a branch and opens a pull request; files listed in `.github/upstream-keep` keep this
repository's version.

The mirror starts at the renamed `feaa1138`, merged into `main` (with this repository's tree)
by the pull request that renamed the app. `python3 scripts/rebrand.py check` fails when an old
name comes back; a line that must keep one says `rebrand: keep`.
