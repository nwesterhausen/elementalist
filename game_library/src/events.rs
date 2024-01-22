//! Events that can be sent to the game state machine.
//!
//! These are available here inside the game library for ease of reference
//! in the game code. Events inside this module didn't really have a good
//! spot to live in the game library, so they were placed here.

use bevy::prelude::*;

/// Cast a spell. Sending this even will cause a spell to be cast.
///
/// # Internal Value
///
/// * `String` - The `get_internal_id()` of the [`game_library::Spell`]
///
/// # Examples
///
/// ```
/// use game_library::events::CastSpell;
///
/// let event = CastSpell("Firebolt01Primary".to_string());
/// ```
#[derive(Event)]
pub struct CastSpell(pub String);
