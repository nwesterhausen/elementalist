use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// Possible actions in the game
///
/// This is a list of all the possible actions in the game. Of course, mouse input
/// is not included here, but it is still possible to bind mouse buttons to actions.
///
/// These actions should be able to act as a "re-mappable" list of actions, so that
/// the player can change the key bindings to their liking.
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    /// Move up
    ///
    /// - move the player up
    /// - move the cursor up (in menus)
    MoveUp,
    /// Move down
    ///
    /// - move the player down
    /// - move the cursor down (in menus)
    MoveDown,
    /// Move left
    ///
    /// - move the player left
    /// - move the cursor left (in menus)
    MoveLeft,
    /// Move right
    ///
    /// - move the player right
    /// - move the cursor right (in menus)
    MoveRight,
    /// Generic move, for use with a d-pad or joystick
    ///
    /// - move the player in the given direction
    /// - move the cursor in the given direction (in menus)
    MoveGeneric,
    /// Cast primary
    ///
    /// - cast the primary spell
    CastPrimary,
    /// Cast secondary
    ///
    /// - cast the secondary spell
    CastSecondary,
    /// Cast defensive
    ///
    /// - cast the defensive spell
    CastDefensive,
    /// Cast ultimate
    ///
    /// - cast the ultimate spell
    CastUltimate,
    /// Toggle auto-cast
    ///
    /// - toggle auto-cast for the all spells (as configured in the settings)
    ToggleAutoCast,
    /// Toggle auto-aim
    ///
    /// - toggle auto-aim for the all spells (as configured in the settings)
    ToggleAutoAim,
    /// Multi-purpose interact
    ///
    /// - interact with an object in game
    /// - select an option in a menu
    /// - generic "next" for a dialog
    InteractSelect,
    /// Multi-purpose back
    ///
    /// - go back in a menu
    /// - cancel an action
    /// - close (if in a root menu/dialog/etc)
    CancelBack,
    /// Pauses game to open then menu
    ///
    /// - pause & open the in-game menu
    /// - unpause & close the in-game menu (if in the root of the in-game menu)
    Pause,
}

impl Action {
    /// Get the default keyboard/mouse input for the given action
    ///
    /// ## Returns
    ///
    /// The default keyboard/mouse input for the given action
    pub fn default_keyboard_mouse_input(self) -> UserInput {
        super::default_controls::default_keyboard_mouse_input(self)
    }

    /// Get the default gamepad input for the given action
    ///
    /// ## Returns
    ///
    /// The default gamepad input for the given action
    pub fn default_gamepad_input(self) -> UserInput {
        super::default_controls::default_gamepad_input(self)
    }
}
