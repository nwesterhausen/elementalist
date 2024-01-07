use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

/// Actions that the player can take.
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    /// Move in a direction. Should use a virtual dpad or gamepad stick.
    Move,
    /// Cast primary spell slot.
    CastPrimary,
    /// Cast secondary spell slot.
    CastSecondary,
    /// Cast defensive spell slot.
    CastDefensive,
    /// Cast ultimate spell slot.
    CastUltimate,
    /// Toggle auto-cast on/off.
    ///
    /// Auto-cast by default affects all spells, but can be configured to only affect
    /// certain spells in the gameplay menu.
    ToggleAutoCast,
    /// Toggle auto-aim on/off.
    ///
    /// Auto-aim will automatically target the nearest enemy to the player.
    ToggleAutoAim,
    /// Interact with entities in the world.
    Interact,
}

impl Action {
    /// Returns the default keybinds for this action on keyboard & mouse.
    pub fn default_keyboard_mouse_input(&self) -> UserInput {
        match self {
            Action::Move => UserInput::VirtualDPad(VirtualDPad::wasd()),
            Action::CastPrimary => UserInput::Single(InputKind::Mouse(MouseButton::Left)),
            Action::CastSecondary => UserInput::Single(InputKind::Mouse(MouseButton::Right)),
            Action::CastDefensive => UserInput::Single(InputKind::Keyboard(KeyCode::Space)),
            Action::CastUltimate => UserInput::Single(InputKind::Keyboard(KeyCode::E)),
            Action::ToggleAutoCast => UserInput::Single(InputKind::Keyboard(KeyCode::Q)),
            Action::ToggleAutoAim => UserInput::Single(InputKind::Keyboard(KeyCode::T)),
            Action::Interact => UserInput::Single(InputKind::Keyboard(KeyCode::F)),
        }
    }
    /// Returns the default gamepad input for this action.
    pub fn default_gamepad_input(&self) -> UserInput {
        match self {
            Action::Move => UserInput::Single(InputKind::DualAxis(DualAxis::left_stick())),
            Action::CastPrimary => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::RightTrigger))
            }
            Action::CastSecondary => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::LeftTrigger))
            }
            Action::CastDefensive => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::South))
            }
            Action::CastUltimate => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::North))
            }
            Action::ToggleAutoCast => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::LeftTrigger2))
            }
            Action::ToggleAutoAim => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::RightTrigger2))
            }
            Action::Interact => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::East))
            }
        }
    }
}
