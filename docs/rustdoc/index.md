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

- [github-markdown-css](https://github.com/sindresorhus/github-markdown-css) (MIT) - CSS file for this page.
- fonts
- some bevy libraries
- yaml library
