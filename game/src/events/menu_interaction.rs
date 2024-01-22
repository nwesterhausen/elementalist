use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

/// Actions that can be taken in the menu.
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MenuInteraction {
    /// Move up a list. This should select the previous item in a vertical list.
    ///
    /// When at the top of a list, this should wrap to the bottom.
    Up,
    /// Move down a list. This should select the next item in a vertical list.
    ///
    /// When at the bottom of a list, this should wrap to the top.
    Down,
    /// Select the currently highlighted item.
    ///
    /// This should be used to select a menu item, or to confirm a selection.
    ///
    /// - On "checkbox" style menu items, this should toggle the checkbox.
    /// - On other menu items, this should "go into" that item so that Left/Right
    /// can be used to change the value of the item.
    /// - On "back" menu items, this should go back to the previous menu.
    Select,
    /// Go back or cancel a selection.
    ///
    /// This should be used to go back to the previous menu, or to cancel a selection.
    ///
    /// - Pressed while nothing is selected should go back to the previous menu (or exit)
    /// - Pressed while a menu item is selected should cancel the selection.
    Back,
}

impl MenuInteraction {
    /// Returns the default keybinds for this action on keyboard & mouse.
    pub const fn default_keyboard_mouse_input(self) -> UserInput {
        match self {
            Self::Up => UserInput::Single(InputKind::Keyboard(KeyCode::W)),
            Self::Down => UserInput::Single(InputKind::Keyboard(KeyCode::S)),
            Self::Select => UserInput::Single(InputKind::Keyboard(KeyCode::F)),
            Self::Back => UserInput::Single(InputKind::Keyboard(KeyCode::X)),
        }
    }
    /// Returns the default gamepad input for this action.
    pub const fn default_gamepad_input(self) -> UserInput {
        match self {
            Self::Up => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadUp)),
            Self::Down => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadDown)),
            Self::Select => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::East)),
            Self::Back => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::South)),
        }
    }

    /// Returns the default input mapping for this action.
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        for variant in Self::variants() {
            input_map.insert(variant.default_keyboard_mouse_input(), variant);
            input_map.insert(variant.default_gamepad_input(), variant);
        }

        input_map
    }
}
