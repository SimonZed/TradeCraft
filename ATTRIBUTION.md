# Asset attribution

TradeCraft includes a modified copy of [egui-charts](https://github.com/userFRM/egui-charts), including its terminal. Its upstream license texts and third-party notices are in [vendor/egui-charts](vendor/egui-charts/). The upstream terminal manifest has no license field; TradeCraft adds one based on the upstream repository's license statement at the recorded revision, which does not specifically single out the terminal.

The experimental Pine interpreter is adapted from [DimmOFF's egui-charts fork](https://github.com/DimmOFF/egui-charts/tree/229207f089a8dd7ee89373dce1c9bc96309da7d4/examples/demo/pine_engine). The initial import used revision [13bf91a2](https://github.com/DimmOFF/egui-charts/commit/13bf91a2ea55d4b46c8de1dc1291a35d4fc6dd78); a later three-way merge incorporated changes from [229207f0](https://github.com/DimmOFF/egui-charts/commit/229207f089a8dd7ee89373dce1c9bc96309da7d4). TradeCraft retained its own `mod.rs` and resolved conflicts and made further modifications, including to the study adapter, diagnostics and execution safeguards. The chart's `vendor/egui-charts/src/studies/table.rs` also incorporates code copied and adapted from the fork. The fork's [MIT license at the later revision](https://github.com/DimmOFF/egui-charts/blob/229207f089a8dd7ee89373dce1c9bc96309da7d4/LICENSE-MIT) has the same copyright and terms as the vendored egui-charts MIT license.

The [BB-levels sample](scripts/pine/bb-levels.pine) adapts a script provided by its original author, who confirmed authorship and authorized MIT licensing on November 14, 2026. The sample and TradeCraft's modifications are available under the project's [MIT license](LICENSE).

After the initial import and merge, TradeCraft selectively incorporated further Pine changes through DimmOFF revision [`e7e1e98b`](https://github.com/DimmOFF/egui-charts/commit/e7e1e98b722e59fc40e30b62bca83f5dbe47989c). The reviewed boundary and adopted changes are recorded in [the Pine upstream review](docs/DIMMOFF-PINE-UPSTREAM.md); the boundary does not mean every upstream change was adopted.

TradeCraft uses the default fonts embedded by [epaint_default_fonts 0.37.0](https://crates.io/crates/epaint_default_fonts/0.37.0) through egui's `default_fonts` feature. No font binaries are maintained in this repository.

- **Hack Regular**: Copyright 2018 Source Foundry Authors, MIT; includes DejaVu public-domain work and Bitstream Vera, copyright 2003 Bitstream Inc. The full notices are in [Hack-Regular.txt](licenses/epaint-default-fonts/Hack-Regular.txt).
- **Ubuntu Light**: Ubuntu Font Licence 1.0. The full license and copyright notices are in [UFL.txt](licenses/epaint-default-fonts/UFL.txt).
- **egui-icons**: subset of emoji-icon-font, copyright 2014 John Slegers, MIT. The notice is in [emoji-icon-font-mit-license.txt](licenses/epaint-default-fonts/emoji-icon-font-mit-license.txt). Some glyphs represent third-party trademarks; attribution does not grant trademark rights.

These are asset notices, not a complete third-party dependency license audit. See [ARCHITECTURE.md](ARCHITECTURE.md) for dependency-license findings.
