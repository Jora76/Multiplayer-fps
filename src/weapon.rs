// use bevy::input::mouse;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use renet::transport::NetcodeClientTransport;
use renet::RenetClient;

use crate::keybind::KeyBinds;
use crate::player::*;
use crate::projectile::*;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;

#[derive(Debug)]
pub struct FireRateTimer {
    pub timer: Timer,
}

impl Default for FireRateTimer {
    fn default() -> Self {
        FireRateTimer {
            timer: Timer::from_seconds(0.3, TimerMode::Repeating), // 0.3 seconde entre chaque tir
        }
    }
}

#[derive(Component)]
pub struct Weapon;

pub fn spawn_weapon(
    parent: &mut ChildBuilder,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.8));
    let arm_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.8, 0.8),
        ..Default::default()
    });

    parent.spawn((
        MaterialMeshBundle {
            mesh: arm,
            material: arm_material,
            transform: Transform::from_xyz(0.15, -0.1, -0.25),
            ..Default::default()
        },
        Weapon,
        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
        NotShadowCaster,
    ));
}

pub fn pew(
    key_binds: Res<KeyBinds>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands<'_, '_>,
    meshes: ResMut<'_, Assets<Mesh>>,
    materials: ResMut<'_, Assets<StandardMaterial>>,
    player_query: Query<'_, '_, &Transform, With<Player>>,
    weapon_query: Query<'_, '_, (&Transform, &Parent), With<Weapon>>,
    time: Res<Time>,
    mut fire_rate_timer: Local<FireRateTimer>,
    asset_server: Res<AssetServer>,
    client: ResMut<RenetClient>,
    transport: ResMut<NetcodeClientTransport>,
) {
    if input.pressed(key_binds.shoot) && fire_rate_timer.timer.tick(time.delta()).just_finished() {
        fire_rate_timer.timer = Timer::from_seconds(0.08, TimerMode::Once);

        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/0437.ogg"),
            ..default()
        });
        spawn_projectile(commands, meshes, materials, weapon_query, player_query, client, transport);
    }
}

pub fn update_arm(
    key_binds: Res<KeyBinds>,
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut arm_query: Query<&mut Transform, With<Weapon>>,
    mut swing_state: Local<f32>,
    mut initial_x: Local<Option<f32>>,
) {
    if let Ok(mut arm_transform) = arm_query.get_single_mut() {
        if initial_x.is_none() {
            *initial_x = Some(arm_transform.translation.x);
        }
        let init_x = initial_x.unwrap();

        let is_aiming = mouse_input.pressed(key_binds.aim);

        if is_aiming {
            arm_transform.translation.x = 0.;
        } else {
            let is_moving = key_input.pressed(key_binds.move_forward)
                || key_input.pressed(key_binds.move_backward)
                || key_input.pressed(key_binds.move_left)
                || key_input.pressed(key_binds.move_right);

            if is_moving {
                *swing_state += time.delta_seconds() * 6.0; // Adjust the swing speed
            } else {
                *swing_state = 0.0;
            }

            let swing_amount = (*swing_state).sin() * 0.1; // Adjust the swing amplitude

            arm_transform.translation.x = init_x + swing_amount;
        }
    }
}
