# Data Files for Game Mechanics

Elementalist reads in game data from the `game_data` directory at launch. These files contain details about the spells and tilesets, and supporting data like particle effects. This is to allow easily adding content to the game, and to make adding it accessible to players.

These files can be in any directory that is within the `game_data` directory (they do not be to separated into different data types). So for example, you can put the data for your spells and their particle data in the same directory and it will be loaded.

## Schemas for Validation

There are publicly available schemas that define the different data files you may write. These are in `_schemas` but also hosted online for ease of IDE integration via the schema validation settings.

| Data                   | Schema URL                                                   |
| ---------------------- | ------------------------------------------------------------ |
| Particle Effect        | `https://schemas.nwest.one/games/elementalist/particle.json` |
| Sprite Atlas / Tileset | `https://schemas.nwest.one/games/elementalist/tileset.json`  |
| Spell                  | `https://schemas.nwest.one/games/elementalist/spell.json`    |

## Loading Order

Data files are loaded in a specific order so that some data files may reference others (and we can validate that reference). This is to embrace the multi-threaded-ness of bevy as much as possible, to make loading the data as fast as possible. The schedules for the data files are part of a graph with the dependencies needed for inter-linking (e.g. linking a particle effect to a spell) are in the previous node. This is defined with a "schedule" which is just a system set loaded to allow those jobs to be run in parallel.

1. Schedule A: no inter-linking with other data files (the roots)
2. Schedule B: inter-link only with data loaded from Schedule A
3. (Theoretical) Schedule C: can depend on data in Schedule A and Schedule B or just Schedule B.

The current data loading order:

| Schedule | Data to be loaded       |
| -------- | ----------------------- |
| A        | Particle Effects        |
| A        | Sprite Atlas / Tilesets |
| B        | Spell                   |

## YAML Validation

The standard way to add validation to a file is to include a comment at the top of the file which points to the schema URL.

### Specifying the Schema (Recommended)

You can add a comment at the top of the yaml file to indicate what schema to use:

```yaml
# yaml-language-server: $schema=https://schemas.nwest.one/games/elementalist/spell.json
```

This is probably the recommended way if you plan to have different data files in the same directory.

### Specifying the Schema in VSCode (Alternative)

In vscode, if you are using the redhat yaml validation plugin (which is likely), you can edit your `settings.json`
file to inform it of the proper schemas for the vanilla `game_data` file directories:

```json
  "yaml.schemas": {
    "https://schemas.nwest.one/games/elementalist/spell.json": ["**/game_data/spells/*.y*ml"],
    "https://schemas.nwest.one/games/elementalist/spell_tag.json": ["**/game_data/spell_tags/*.y*ml"],
    "https://schemas.nwest.one/games/elementalist/tileset.json": ["**/game_data/tilesets/*.y*ml"],
    "https://schemas.nwest.one/games/elementalist/particle.json": ["**/game_data/particles/*.y*ml"]
  }
```
