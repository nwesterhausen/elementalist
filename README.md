# ![game icon](./game_assets/Icon.png) Elementalist

[![Documentation](https://github.com/nwesterhausen/elementalist/actions/workflows/docs.yml/badge.svg)](https://nwesterhausen.github.io/elementalist/)
[![Build & Test](https://github.com/nwesterhausen/elementalist/actions/workflows/rust.yml/badge.svg)](https://github.com/nwesterhausen/elementalist/actions/workflows/rust.yml)
[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/8337/badge)](https://www.bestpractices.dev/projects/8337)

|                                     |                        |
| ----------------------------------- | ---------------------- |
| ![wizard](./game_assets/Wizard.png) | A game made with bevy. |

- [Design Notes](./design_notes) include info about the gameplay loop and gameplay.
- [Game](./game) is the bevy app
- [Game Assets](./game_assets) has the raw graphics assets (which get exported to [game/assets](./game/assets/)
- [Game Data](./game_data) has the data files that create spells (and other things) in the game
- [Game Library](./game_library) has most of the components and resources used by the game (for ease of testing and reference)

## How to Install / Use

Find the latest release in the [releases](https://github.com/nwesterhausen/elementalist/releases) and download the one appropriate to your platform. Unzip and run the game!

Because the game loads assets and data, the archives contain the appropriate directories already (note on MacOS it is hidden inside the .app file). Without the `game_data`
directory, no spells will be loaded, which will make the game unplayable.
