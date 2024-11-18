use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

#[derive(Resource)]
pub struct KeyBinds {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,

    pub sprint: KeyCode,
    pub jump: KeyCode,

    pub shoot: MouseButton,
    pub aim: MouseButton,
}

impl Default for KeyBinds {
    fn default() -> Self {
        KeyBinds {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            sprint: KeyCode::ShiftLeft,
            jump: KeyCode::Space,
            shoot: MouseButton::Left,
            aim: MouseButton::Right,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum KeybindingState {
    Normal,
    Rebinding(String),
}

impl Default for KeybindingState {
    fn default() -> Self {
        KeybindingState::Normal
    }
}

fn _key_rebinding_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut key_bindings: ResMut<KeyBinds>,
    current_key: Res<State<KeybindingState>>,
    mut next_state: ResMut<NextState<KeybindingState>>,
) {
    if let KeybindingState::Rebinding(action) = current_key.get().clone() {
        for key in keyboard_input.get_just_pressed() {
            println!("Rebinding {} to {:?}", action, key);
            match action.as_str() {
                "move_forward" => key_bindings.move_forward = *key,
                "move_backward" => key_bindings.move_backward = *key,
                "move_left" => key_bindings.move_left = *key,
                "move_right" => key_bindings.move_right = *key,
                "sprint" => key_bindings.sprint = *key,
                "jump" => key_bindings.jump = *key,
                _ => (),
            }
            next_state.set(KeybindingState::Normal);
        }
    }
}
