# Asset attribution

Every non-code asset in this repository (images, icons, fonts, example drawings, presets) is listed
here with its author, source and licence. `cargo xtask assets` (part of `cargo xtask ci`) fails if
an asset file is missing from this table.

**Policy (mandatory):** CADCraft contains **no Autodesk, Adobe or Avid iconography, images,
artwork, fonts, hatch patterns, linetypes, templates or presets.** Every asset is original work by
CADCraft contributors or third-party material under an open licence (OSI open source, public domain
/ CC0, or Creative Commons that allows redistribution). Screenshots of Autodesk software are never
committed. Font files are not added here: shared fonts live in
[storytold/craft-fonts](https://github.com/storytold/craft-fonts), an optional build input. The one
exception is the ArtCraft brand in `docs/brand/`: ArtCraft trademarks, not open source, used under
`docs/brand/LICENSE-brand.txt`.

Generated-in-code assets are original and have no file to list:
- the UI icon set (`crates/ui-egui/src/icons.rs`);
- the "CADCraft Stroke" single-stroke drafting font (`crates/fonts/src/stroke.rs`);
- the standard linetype and hatch-pattern libraries (`crates/doc/src/library.rs`; industry-common
  names, our own dash/spacing values);
- the colour index palette, generated from its structure (`crates/color/src/lib.rs`);
- the sample drawings (`crates/engine/src/sample.rs`).

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `assets/app-icon/hicolor/128x128/apps/ai.storyteller.cadcraft.png` | CADCraft contributors | generated from `assets/app-icon/cadcraft-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/16x16/apps/ai.storyteller.cadcraft.png` | CADCraft contributors | generated from `assets/app-icon/cadcraft-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/24x24/apps/ai.storyteller.cadcraft.png` | CADCraft contributors | generated from `assets/app-icon/cadcraft-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/256x256/apps/ai.storyteller.cadcraft.png` | CADCraft contributors | generated from `assets/app-icon/cadcraft-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/32x32/apps/ai.storyteller.cadcraft.png` | CADCraft contributors | generated from `assets/app-icon/cadcraft-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/48x48/apps/ai.storyteller.cadcraft.png` | CADCraft contributors | generated from `assets/app-icon/cadcraft-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/512x512/apps/ai.storyteller.cadcraft.png` | CADCraft contributors | generated from `assets/app-icon/cadcraft-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/64x64/apps/ai.storyteller.cadcraft.png` | CADCraft contributors | generated from `assets/app-icon/cadcraft-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/cadcraft.icns` | CADCraft contributors | generated (packaging/icons.sh) | MIT OR Apache-2.0 | macOS icon |
| `assets/app-icon/cadcraft.ico` | CADCraft contributors | generated (packaging/icons.sh) | MIT OR Apache-2.0 | Windows icon |
| `assets/app-icon/cadcraft-64.png` | CADCraft contributors | generated | MIT OR Apache-2.0 | |
| `assets/app-icon/cadcraft-256.png` | CADCraft contributors | generated | MIT OR Apache-2.0 | |
| `assets/app-icon/cadcraft-1024.png` | CADCraft contributors | generated | MIT OR Apache-2.0 | |
| `assets/app-icon/cadcraft-macos-512.png` | CADCraft contributors | generated (macOS margin variant) | MIT OR Apache-2.0 | |
| `examples/apartment.dxf` | CADCraft contributors | sample drawing made with cadcraft-cli commands | MIT OR Apache-2.0 | Original |
| `docs/images/ui-apartment.png` | CADCraft contributors | screenshot of CADCraft itself (examples/apartment.dxf) | MIT OR Apache-2.0 | Original; no Autodesk UI |
| `docs/images/ui-layout.png` | CADCraft contributors | screenshot of CADCraft itself (a layout of examples/apartment.dxf) | MIT OR Apache-2.0 | Original; no Autodesk UI |
| `docs/images/ui-bracket.png` | CADCraft contributors | screenshot of CADCraft itself (sample drawing generated in code) | MIT OR Apache-2.0 | Original; no Autodesk UI |
| `docs/brand/artcraft-logo-white.png` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/artcraft-logo-white.svg` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/artcraft-logo.png` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/artcraft-logo.svg` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/artcraft-mark-black.png` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/artcraft-mark-black.svg` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/artcraft-mark.png` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/artcraft-mark.svg` | ArtCraft Team | storytold/craftrules `assets/brand/` | ArtCraft brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `crates/ui-egui/assets/jf-openhuninn-2.1.ttf` | justfont Co., LTD. (jf open huninn, based on Kosugi Maru / Varela Round) | https://github.com/justfont/open-huninn-font release v2.1 | SIL Open Font License 1.1 (`crates/ui-egui/assets/OFL.txt`) | Bundled UI font for the Traditional Chinese (zh-TW) localisation of this fork, embedded at build time so Chinese UI text renders on every platform without a system CJK font. This is a deliberate fork-local exception to the upstream "fonts live in craft-fonts" convention, requested by the fork owner. |
| `crates/ui-egui/assets/OFL.txt` | justfont Co., LTD. / SIL International | licence text shipped with jf open huninn 2.1 | SIL Open Font License 1.1 | Licence for the bundled huninn font above |
