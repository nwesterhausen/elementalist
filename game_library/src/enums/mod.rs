//! Enums used in the game library and Elementalist game.

mod biome;
mod cast_category;
mod cast_slot;
mod cast_type;
mod game_systems;
mod magic;
mod particle_attachment;
mod skill;
mod spell_collision;
mod spell_target;
mod stat;

pub use biome::*;
pub use cast_category::CastCategory;
pub use cast_slot::CastSlot;
pub use cast_type::CastType;
pub use game_systems::GameSystem;
pub use magic::MagicType;
pub use particle_attachment::ParticleAttachment;
pub use skill::Skill;
pub use spell_collision::SpellCollision;
pub use spell_target::SpellTarget;
pub use stat::StatEnum;
