# Elementalist

Documentation is automatically build from the main branch. It can provide an overview of the supporting **`game_library`** used by **Elementalist**.

## Documentation

The following generated documentation is available:

- [Elementalist](elementalist/index.html) - the game built with `bevy`, fully documented
- [game_library](game_library/index.html) - the supporting library behind the game:
  - Shared components or components which are used for queries
  - Resource structs
  - Event structs
  - Data enums (e.g. skills or stats)
  - Shared library of functions for reuse in the game (e.g. one to calculate slope from player to mouse)
  - Unit tests for all the `impl` of components, resources, etc.
  - The data file import library
