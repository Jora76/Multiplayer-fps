use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::{ScalingMode, Viewport};
use bevy::render::view::RenderLayers;
use bevy_rapier3d::prelude::*;

use crate::player::*;
use crate::weapon::spawn_weapon;

// Used by the view model camera and the player's arm.
// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;

#[derive(Debug, Component)]
struct WorldModelCamera;

pub fn spawn_crosshair(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(50.0),
                    left: Val::Percent(50.0),
                    ..default()
                },
                ..default()
            },
            RenderLayers::layer(0),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                concat!("+",),
                TextStyle {
                    font_size: 25.0,
                    ..default()
                },
            ));
        });
}

pub fn move_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    mut player: Query<&mut Transform, With<Player>>,
    mut pitch: Local<f32>,
) {
    let mut transform = player.single_mut();
    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.008;
        let delta_pitch = -motion.delta.y * 0.007;

        // Mettez à jour l'angle de pitch et limitez-le entre -89 et 89 degrés
        *pitch = (*pitch + delta_pitch).clamp(-69.0_f32.to_radians(), 69.0_f32.to_radians());

        // Appliquez la rotation en yaw
        transform.rotate_y(yaw);

        // Appliquez la rotation en pitch en utilisant l'angle limité
        transform.rotation = Quat::from_rotation_y(transform.rotation.to_euler(EulerRot::YXZ).0)
            * Quat::from_rotation_x(*pitch);
    }
}

#[derive(Component, Default)]
pub struct Minimap;

pub fn spawn_view_model(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let minimap_camera = (
        Camera3dBundle {
            projection: Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::Fixed {
                    width: 50.0,
                    height: 50.0,
                },
                near: -10.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 100.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera_3d: Camera3d { ..default() },
            ..default()
        },
        Minimap::default(),
        Name::new("MinimapCamera"),
    );

    commands
        .spawn((
            Player,
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 1.3, 0.0),
                ..default()
            },
            Collider::round_cylinder(0.9, 0.3, 0.2),
            KinematicCharacterController {
                custom_mass: Some(5.0),
                up: Vec3::Y,
                offset: CharacterLength::Absolute(0.01),
                slide: true,
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Relative(0.3),
                    min_width: CharacterLength::Relative(0.5),
                    include_dynamic_bodies: false,
                }),
                // Don’t allow climbing slopes larger than 45 degrees.
                max_slope_climb_angle: 45.0_f32.to_radians(),
                // Automatically slide down on slopes smaller than 30 degrees.
                min_slope_slide_angle: 30.0_f32.to_radians(),
                apply_impulse_to_dynamic_bodies: true,
                snap_to_ground: None,
                ..default()
            },
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        // .insert(GravityScale(0.))
        .with_children(|parent| {
            parent.spawn((
                WorldModelCamera,
                Camera3dBundle {
                    camera: Camera {
                        // Bump the order to render on top of the world model.
                        order: -1,
                        ..default()
                    },
                    projection: PerspectiveProjection {
                        fov: 80.0_f32.to_radians(),
                        ..default()
                    }
                    .into(),
                    ..default()
                },
            ));

            // Spawn view model camera.
            parent.spawn((
                Camera3dBundle {
                    camera: Camera {
                        // Bump the order to render on top of the world model.
                        order: 2,
                        ..default()
                    },
                    projection: PerspectiveProjection {
                        fov: 70.0_f32.to_radians(),
                        ..default()
                    }
                    .into(),
                    ..default()
                },
                // Only render objects belonging to the view model.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));
            // spawn_crosshair(parent);

            spawn_weapon(parent, meshes, materials);
        });
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(180.0),
                right: Val::Px(180.0),
                width: Val::Px(10.0),
                height: Val::Px(10.0),
                ..default()
            },
            background_color: Srgba::new(0.0, 1.0, 0.0, 0.6).into(),
            ..default()
        },
        RenderLayers::layer(0),
    ));

    commands.spawn(minimap_camera);
    println!("minimap camera has spawn");
}

pub fn update_minimap(
    window_query: Query<&Window>,
    mut minimap_camera: Query<&mut Camera, With<Minimap>>,
) {
    let size = 350;
    let window = window_query.get_single().unwrap();

    if let Ok(mut minimap_camera) = minimap_camera.get_single_mut() {
        minimap_camera.viewport = Some(Viewport {
            physical_position: UVec2::new(window.resolution.physical_width() - size - 10, 10),
            physical_size: UVec2::new(size, size),
            ..default()
        });
    }
}

pub fn update_minimap_camera_rotation(
    player_query: Query<&Transform, With<Player>>,
    mut minimap_camera_query: Query<&mut Transform, (With<Minimap>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut minimap_camera_transform) = minimap_camera_query.get_single_mut() {
            minimap_camera_transform.translation =
                player_transform.translation + Vec3::new(0.0, -5.0, 0.0);
            let down_rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
            let (yaw, _, _) = player_transform.rotation.to_euler(EulerRot::YXZ);
            let player_rotation_y = Quat::from_rotation_y(yaw);
            minimap_camera_transform.rotation = player_rotation_y * down_rotation;
        }
    }
}
