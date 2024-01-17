use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

/// Actions that the player can take.
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    /// Move in a direction. Should use a virtual dpad or gamepad stick.
    Move,
    /// Aim in a direction. Uses a mouse in the default keyboard & mouse input map.
    Look,
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

impl PlayerAction {
    /// Returns the default keybinds for this action on keyboard & mouse.
    pub fn default_keyboard_mouse_input(self) -> UserInput {
        match self {
            PlayerAction::Move => UserInput::VirtualDPad(VirtualDPad::wasd()),
            PlayerAction::Look => UserInput::VirtualDPad(VirtualDPad::arrow_keys()),
            PlayerAction::CastPrimary => UserInput::Single(InputKind::Mouse(MouseButton::Left)),
            PlayerAction::CastSecondary => UserInput::Single(InputKind::Mouse(MouseButton::Right)),
            PlayerAction::CastDefensive => UserInput::Single(InputKind::Keyboard(KeyCode::Space)),
            PlayerAction::CastUltimate => UserInput::Single(InputKind::Keyboard(KeyCode::E)),
            PlayerAction::ToggleAutoCast => UserInput::Single(InputKind::Keyboard(KeyCode::Q)),
            PlayerAction::ToggleAutoAim => UserInput::Single(InputKind::Keyboard(KeyCode::T)),
            PlayerAction::Interact => UserInput::Single(InputKind::Keyboard(KeyCode::F)),
        }
    }
    /// Returns the default gamepad input for this action.
    pub fn default_gamepad_input(self) -> UserInput {
        match self {
            PlayerAction::Move => UserInput::Single(InputKind::DualAxis(DualAxis::left_stick())),
            PlayerAction::Look => UserInput::Single(InputKind::DualAxis(DualAxis::right_stick())),
            PlayerAction::CastPrimary => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::RightTrigger))
            }
            PlayerAction::CastSecondary => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::LeftTrigger))
            }
            PlayerAction::CastDefensive => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::South))
            }
            PlayerAction::CastUltimate => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::North))
            }
            PlayerAction::ToggleAutoCast => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::LeftTrigger2))
            }
            PlayerAction::ToggleAutoAim => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::RightTrigger2))
            }
            PlayerAction::Interact => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::East))
            }
        }
    }

    /// Returns the default input map for this action.
    pub fn default_input_map() -> InputMap<PlayerAction> {
        let mut input_map = InputMap::default();

        for variant in PlayerAction::variants() {
            input_map.insert(variant.default_keyboard_mouse_input(), variant);
            input_map.insert(variant.default_gamepad_input(), variant);
        }

        input_map
    }
}
