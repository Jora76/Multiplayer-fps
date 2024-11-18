use renet::{transport::NetcodeClientTransport, ClientId, RenetClient};

use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::{control::KinematicCharacterController, prelude::*};

use crate::client::client_send_input;

#[derive(Debug, Component)]
pub struct Player;

use crate::keybind::*;
#[derive(Debug, Component)]
pub struct PlayerData {
    pub id: ClientId,
    pub position: Transform,
}

impl PlayerData {
    pub fn new(id: ClientId) -> PlayerData {
        PlayerData {
            id,
            position: Transform::from_xyz(0.0, 1.3, 0.0),
        }
    }
}

const MOUSE_SENSITIVITY: f32 = 0.3;
const GROUND_TIMER: f32 = 0.5;
const MOVEMENT_SPEED: f32 = 8.0;
const JUMP_SPEED: f32 = 20.0;
const GRAVITY: f32 = -9.81;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Resource)]
pub enum PlayerState {
    #[default]
    Alive,
    Dead,
}

/// Keyboard input vector
#[derive(Default, Resource, Deref, DerefMut)]
pub struct MovementInput(Vec3);

/// Mouse input vector
#[derive(Default, Resource, Deref, DerefMut)]
pub struct LookInput(Vec2);

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    key_binds: Res<KeyBinds>,
    mut movement: ResMut<MovementInput>,
    mut look: ResMut<LookInput>,
    mut mouse_events: EventReader<MouseMotion>,
) {
    if keyboard.pressed(key_binds.move_forward) {
        movement.z -= 1.0;
    }
    if keyboard.pressed(key_binds.move_backward) {
        movement.z += 1.0;
    }
    if keyboard.pressed(key_binds.move_left) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(key_binds.move_right) {
        movement.x += 1.0;
    }
    **movement = movement.normalize_or_zero();
    if keyboard.pressed(key_binds.sprint) {
        **movement *= 2.0;
    }
    if keyboard.pressed(key_binds.jump) {
        movement.y = 1.0;
    }

    for event in mouse_events.read() {
        look.x -= event.delta.x * MOUSE_SENSITIVITY;
        look.y -= event.delta.y * MOUSE_SENSITIVITY;
        look.y = look.y.clamp(-89.9, 89.9); // Limit pitch
    }
}

pub fn player_movement(
    time: Res<Time>,
    mut input: ResMut<MovementInput>,
    mut player: Query<(
        &mut Transform,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    )>,
    mut vertical_movement: Local<f32>,
    mut grounded_timer: Local<f32>,
    client: ResMut<RenetClient>,
    transport: ResMut<NetcodeClientTransport>,
) {
    let input_pressed = input.length() > 0.0;
    let Ok((transform, mut controller, output)) = player.get_single_mut() else {
        return;
    };
    if input_pressed {
        let delta_time = time.delta_seconds();
        // Retrieve input
        let mut movement = Vec3::new(input.x, 0.0, input.z) * MOVEMENT_SPEED;
        let jump_speed = input.y * JUMP_SPEED;
        // Clear input
        **input = Vec3::ZERO;
        // Check physics ground check
        if output.map(|o| o.grounded).unwrap_or(false) {
            *grounded_timer = GROUND_TIMER;
            *vertical_movement = 0.0;
        }
        // If we are grounded we can jump
        if *grounded_timer > 0.0 {
            *grounded_timer -= delta_time;
            // If we jump we clear the grounded tolerance
            if jump_speed > 0.0 {
                *vertical_movement = jump_speed;
                *grounded_timer = 0.0;
            }
        }
        movement.y = *vertical_movement;
        *vertical_movement += GRAVITY * delta_time * controller.custom_mass.unwrap_or(1.0);
        controller.translation = Some(transform.rotation * (movement * delta_time));
        client_send_input(transform.translation, client, transport);
    }
}

pub fn player_look(
    mut player: Query<&mut Transform, (With<KinematicCharacterController>, Without<Camera>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
    input: Res<LookInput>,
) {
    let Ok(mut transform) = player.get_single_mut() else {
        return;
    };
    transform.rotation = Quat::from_axis_angle(Vec3::Y, input.x.to_radians());
    let Ok(mut transform) = camera.get_single_mut() else {
        return;
    };
    transform.rotation = Quat::from_axis_angle(Vec3::X, input.y.to_radians());
}

pub fn la_mooooooooooort(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_state: ResMut<PlayerState>,
) {
    if *player_state == PlayerState::Dead {
        println!("You are dead");
        commands.spawn(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "You are dead".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::srgb(1., 0., 0.),
                    },
                }],
                justify: JustifyText::Center,
                ..Default::default()
            },
            ..Default::default()
        });
        *player_state = PlayerState::Alive;
    }
}
