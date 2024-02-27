//! Elementalist spell components, resources, and systems.
//!
//! This module contains the spell components, resources, and systems for the Elementalist game.
//!
//! ## Spell Casting Overview
//!
//! See the [design document](https://github.com/nwesterhausen/elementalist/blob/main/design_notes/game_design_document.md)
//! for a more comprehensive overview of the spell casting system and how it fits into the game. Here is a brief overview:
//!
//! The player has spell slots which can be slotted with a "spell talisman" and assigned a skill to cast the spell. The spell talisman
//! is a talisman to define how the spell behaves (e.g. projectile, area of effect, etc.). The skill is on of the 16 skills the player
//! can have, and the skill determines the element of the spell (e.g. fire, water, etc.). Together, they create the "spell" which
//! can be cast by the player.
//!
//! ## Spell Definitions
//!
//! Only talisman and skill combinations which result in a valid spell are allowed. The spell definitions are defined by the files
//! in the `game_data` directory. Additional spell definitions are supported through loading mods (yet to be implemented). The spell
//! definitions are loaded into the game and used to validate the player's spell slots.
//!
//! Spells are defined in a `YAML` file with this schema: `https://schemas.nwest.one/games/elementalist/spell.json`
//!
//! To have schema validation in your editor, you can add the following to the top of your `YAML` file:
//!
//! ```yaml
//! # yaml-language-server: $schema=https://schemas.nwest.one/games/elementalist/spell.json
//! ```
//!

mod bundle;
mod definition;
mod lifetime;
mod particles;
mod polymorph;
mod selection;
pub mod talisman;

pub use bundle::SpellBundle;
pub use definition::Spell;
pub use lifetime::SpellLifetime;
pub use particles::SpellParticles;
pub use polymorph::*;
pub use selection::SpellSelection;
