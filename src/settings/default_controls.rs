use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

use super::Action;

/// Exhaustively match all possible actions and return the default keyboard/mouse input
///
/// ## Arguments
///
/// - `action`: the action to match
///
/// ## Returns
///
/// The default keyboard/mouse input for the given action
pub fn default_keyboard_mouse_input(action: Action) -> UserInput {
    match action {
        Action::MoveGeneric => UserInput::VirtualDPad(VirtualDPad::wasd()),
        Action::MoveUp => UserInput::Single(InputKind::Keyboard(KeyCode::W)),
        Action::MoveDown => UserInput::Single(InputKind::Keyboard(KeyCode::S)),
        Action::MoveLeft => UserInput::Single(InputKind::Keyboard(KeyCode::A)),
        Action::MoveRight => UserInput::Single(InputKind::Keyboard(KeyCode::D)),
        Action::CastPrimary => UserInput::Single(InputKind::Mouse(MouseButton::Left)),
        Action::CastSecondary => UserInput::Single(InputKind::Mouse(MouseButton::Right)),
        Action::CastDefensive => UserInput::Single(InputKind::Keyboard(KeyCode::Space)),
        Action::CastUltimate => UserInput::Single(InputKind::Keyboard(KeyCode::E)),
        Action::ToggleAutoCast => UserInput::Single(InputKind::Keyboard(KeyCode::Q)),
        Action::ToggleAutoAim => UserInput::Single(InputKind::Keyboard(KeyCode::T)),
        Action::CancelBack => UserInput::Single(InputKind::Keyboard(KeyCode::X)),
        Action::InteractSelect => UserInput::Single(InputKind::Keyboard(KeyCode::F)),
        Action::Pause => UserInput::Single(InputKind::Keyboard(KeyCode::Tab)),
    }
}

/// Exhaustively match all possible actions and return the default gamepad input
///
/// ## Arguments
///
/// - `action`: the action to match
///
/// ## Returns
///
/// The default gamepad input for the given action
pub fn default_gamepad_input(action: Action) -> UserInput {
    match action {
        Action::MoveGeneric => UserInput::Single(InputKind::DualAxis(DualAxis::left_stick())),
        Action::MoveUp => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadDown)),
        Action::MoveDown => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadUp)),
        Action::MoveLeft => {
            UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadLeft))
        }
        Action::MoveRight => {
            UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadRight))
        }
        Action::CastPrimary => {
            UserInput::Single(InputKind::GamepadButton(GamepadButtonType::RightTrigger))
        }
        Action::CastSecondary => {
            UserInput::Single(InputKind::GamepadButton(GamepadButtonType::LeftTrigger))
        }
        Action::CastDefensive => {
            UserInput::Single(InputKind::GamepadButton(GamepadButtonType::West))
        }
        Action::CastUltimate => {
            UserInput::Single(InputKind::GamepadButton(GamepadButtonType::North))
        }
        Action::ToggleAutoCast => {
            UserInput::Single(InputKind::GamepadButton(GamepadButtonType::LeftTrigger2))
        }
        Action::ToggleAutoAim => {
            UserInput::Single(InputKind::GamepadButton(GamepadButtonType::RightThumb))
        }
        Action::CancelBack => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::East)),
        Action::InteractSelect => {
            UserInput::Single(InputKind::GamepadButton(GamepadButtonType::South))
        }
        Action::Pause => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::Start)),
    }
}

/// Map the default controls or load from the settings file
///
/// ## Returns
///
/// The populated input map
pub fn map_controls() -> InputMap<Action> {
    // Create a default map to populate
    let mut input_map = InputMap::default();

    // First load the default controls
    for action in Action::variants() {
        input_map.insert(action.default_keyboard_mouse_input(), action);
        input_map.insert(action.default_gamepad_input(), action);
    }

    // Todo: load the controls from the settings file (we need a settings file first)

    // Return the populated map
    input_map
}
