# Data Files for Game Mechanics

Elementalist reads in game data from the `game_data` directory at launch. These files contain details about the spells and tilesets, and supporting data like particle effects. This is to allow easily adding content to the game, and to make adding it accessible to players.

| Data                   | Schema URL                                                   |
| ---------------------- | ------------------------------------------------------------ |
| Particle Effect        | `https://schemas.nwest.one/games/elementalist/particle.json` |
| Sprite Atlas / Tileset | `https://schemas.nwest.one/games/elementalist/tileset.json`  |
| Spell                  | `https://schemas.nwest.one/games/elementalist/spell.json`    |

These files can be in any directory that is within the `game_data` directory (they do not be to separated into different data types). So for example, you can put the data for your spells and their particle data in the same directory and it will be loaded.

## Loading Order

Data files are loaded in a specific order so that some data files may reference others (and we can validate that reference).

1. Sprite Atlas and Tilesets
2. Particle Effects
3. Spells

## YAML Validation

The standard way to add validation to a file is to include a comment at the top of the file which points to the schema URL.

### VSCode

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
