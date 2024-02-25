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
    /// Pause the game and open the menu.
    Pause,
    /// Open the status overlay.
    StatusOverlay,
}

impl PlayerAction {
    /// Returns the default keybinds for this action on keyboard & mouse.
    pub const fn default_keyboard_mouse_input(self) -> UserInput {
        match self {
            Self::Move => UserInput::VirtualDPad(VirtualDPad::wasd()),
            Self::Look => UserInput::VirtualDPad(VirtualDPad::arrow_keys()),
            Self::CastPrimary => UserInput::Single(InputKind::Mouse(MouseButton::Left)),
            Self::CastSecondary => UserInput::Single(InputKind::Mouse(MouseButton::Right)),
            Self::CastDefensive => UserInput::Single(InputKind::PhysicalKey(KeyCode::Space)),
            Self::CastUltimate => UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyE)),
            Self::ToggleAutoCast => UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyQ)),
            Self::ToggleAutoAim => UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyT)),
            Self::Interact => UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyF)),
            Self::Pause => UserInput::Single(InputKind::PhysicalKey(KeyCode::Escape)),
            Self::StatusOverlay => UserInput::Single(InputKind::PhysicalKey(KeyCode::Tab)),
        }
    }
    /// Returns the default gamepad input for this action.
    pub const fn default_gamepad_input(self) -> UserInput {
        match self {
            Self::Move => UserInput::Single(InputKind::DualAxis(DualAxis::left_stick())),
            Self::Look => UserInput::Single(InputKind::DualAxis(DualAxis::right_stick())),
            Self::CastPrimary => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::RightTrigger))
            }
            Self::CastSecondary => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::LeftTrigger))
            }
            Self::CastDefensive => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::South))
            }
            Self::CastUltimate => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::North))
            }
            Self::ToggleAutoCast => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::LeftTrigger2))
            }
            Self::ToggleAutoAim => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::RightTrigger2))
            }
            Self::Interact => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::East)),
            Self::Pause => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::Start)),
            Self::StatusOverlay => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::Select))
            }
        }
    }
    /// Returns the variants of this enum.
    #[must_use]
    pub const fn variants() -> [Self; 11] {
        [
            Self::Move,
            Self::Look,
            Self::CastPrimary,
            Self::CastSecondary,
            Self::CastDefensive,
            Self::CastUltimate,
            Self::ToggleAutoCast,
            Self::ToggleAutoAim,
            Self::Interact,
            Self::Pause,
            Self::StatusOverlay,
        ]
    }

    /// Returns the default input map for this action.
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        for variant in Self::variants() {
            input_map.insert(variant, variant.default_keyboard_mouse_input());
            input_map.insert(variant, variant.default_gamepad_input());
        }

        input_map
    }
}
