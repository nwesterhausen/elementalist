# Data Files for Game Mechanics

The files in this directory contain structured files to keep track of complicated game systems.

See the supported schema files to help format the files.

## YAML Validation

In vscode, if you are using the redhat yaml validation plugin (which is likely), you can edit your `settings.json`
file to inform it of the proper schemas for the `game_data` files:

```json
  "yaml.schemas": {
    "https://schemas.nwest.one/games/elementalist/spell.json": ["**/game_data/spells/*.yaml"]
  }
```
