# Elementalist

Documentation is automatically built from the main branch. It can provide an overview of the supporting **`game_library`** used by **Elementalist**.

## Documentation

The following generated documentation is available:

- [Elementalist](elementalist/index.html) - the game built with `bevy`
  - Plugins and systems that make the game a game
  - Input set up from `leafwing_input_manager`
  - Heavily relies on the `game_library` definitions
- [game_library](game_library/index.html) - the supporting library behind the game:
  - Shared components or components which are used for queries
  - Resource structs
  - Event structs
  - Data enums (e.g. skills or stats)
  - Shared library of functions for reuse in the game (e.g. one to calculate slope from player to mouse)
  - Unit tests for all the `impl` of components, resources, etc.
  - The data file import library

## Open Source Libraries and Resources

A listing of the open source libraries and resources used in this project:

- CSS
  - [github-markdown-css](https://github.com/sindresorhus/github-markdown-css) (MIT) - CSS file for this page.
- Fonts
  - [Almendra](https://fonts.google.com/specimen/Almendra) (OFL)
  - [Almendra Display](https://fonts.google.com/specimen/Almendra+Display) (OFL)
  - [Almendra SC](https://fonts.google.com/specimen/Almendra+SC) (OFL)
  - [Nanum Gothic Coding](https://fonts.google.com/specimen/Nanum+Gothic+Coding) (OFL)
  - [Red Hat Display](https://fonts.google.com/specimen/Red+Hat+Text) (OFL)
  - [Red Hat Text](https://fonts.google.com/specimen/Red+Hat+Text) (OFL)
  - [Syne Mono](https://fonts.google.com/specimen/Syne+Mono) (OFL)
  - [Syne Tactile](https://fonts.google.com/specimen/Syne+Tactile) (OFL)
  - [Open Dyslexic](https://opendyslexic.org/about) (OFL)
- Bevy Libraries
  - [bevy](https://crates.io/crates/bevy) (MIT)
  - [leafwing-input-manager](https://crates.io/crates/leafwing-input-manager) (MIT)
  - [bevy_hanabi](https://crates.io/crates/bevy_hanabi) (MIT)
  - [bevy_pkv](https://crates.io/crates/bevy_pkv) (MIT)
- Bevy Libraries (for debugging/development)
  - [egui_doc](https://crates.io/crates/egui_dock) (MIT)
  - [bevy-inspector-egui](https://crates.io/crates/bevy-inspector-egui) (MIT)
- Other rust dependencies
  - [tracing](https://crates.io/crates/tracing) (MIT)
  - [embed-resource](https://crates.io/crates/embed-resource) (MIT)
  - [built](https://crates.io/crates/built) (MIT)
  - [winit](https://crates.io/crates/winit) (Apache-2.0)
  - [image](https://crates.io/crates/image) (MIT)
  - [rand](https://crates.io/crates/rand) (MIT)
  - [noise](https://crates.io/crates/noise) (MIT)
  - [serde](https://crates.io/crates/serde) (MIT)
  - [serde_default_utils](https://crates.io/crates/serde_default_utils) (MIT)
  - [serde_yaml](https://crates.io/crates/serde_yaml) (MIT)
  - [walkdir](https://crates.io/crates/walkdir) (MIT)
