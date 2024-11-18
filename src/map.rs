use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_rapier3d::prelude::*;

use std::fs::File;
use std::io::Read;

#[derive(Debug, Component)]
pub struct Ground;

#[derive(Debug, Component)]

pub struct Roof;

pub struct Wall;

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;
const TILE_SIZE: f32 = 2.0;

pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let wall_texture_handle = asset_server.load("textures/wall_texture.png");
    // Créer le matériau texturé pour les murs
    let _wall_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color_texture: Some(wall_texture_handle.clone()),
        alpha_mode: AlphaMode::Opaque,
        ..Default::default()
    });
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(64.0)));
    let roof = meshes.add(Cuboid::new(128., 0.1, 128.));

    let horizontal_wall = meshes.add(Cuboid::new(2., 10., 0.2));
    let vertical_wall = meshes.add(Cuboid::new(0.2, 10., 2.));
    let diagonal_l_wall = meshes.add(Cuboid::new(TILE_SIZE.sqrt() * 2., 10., 0.2));

    let material = materials.add(Color::srgb(0.5, 0.2, 0.1));
    let _temp_mat = materials.add(Color::BLACK);

    let mut file = File::open("assets/maps/map00.txt").unwrap();

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    let tilemap = content.split("\n");

    for (y, tile) in tilemap.into_iter().enumerate() {
        for (x, elem) in tile.chars().enumerate() {
            match elem {
                '_' => {
                    commands.spawn((
                        MaterialMeshBundle {
                            mesh: horizontal_wall.clone(),
                            material: material.clone(),
                            transform: Transform::from_xyz(x as f32 - 64., 0.9, y as f32 - 64.),
                            ..default()
                        },
                        // Collider cubique
                        Collider::cuboid(0.6, 5., 0.6),
                        RigidBody::Fixed, // Le cube est immobile
                    )).insert(ActiveEvents::COLLISION_EVENTS);
                }
                '|' => {
                    commands.spawn((
                        MaterialMeshBundle {
                            mesh: vertical_wall.clone(),
                            material: material.clone(),
                            transform: Transform::from_xyz(x as f32 - 64., 0.9, y as f32 - 64.),
                            ..default()
                        },
                        // Collider cubique
                        Collider::cuboid(0.6, 5., 0.6),
                        RigidBody::Fixed, // Le cube est immobile
                    )).insert(ActiveEvents::COLLISION_EVENTS);
                }
                '/' => {
                    commands.spawn((
                        MaterialMeshBundle {
                            mesh: diagonal_l_wall.clone(),
                            material: material.clone(),
                            // transform: Transform::from_xyz(x as f32 -64., 0.9, y as f32 -64.),
                            transform: Transform {
                                translation: Vec3::new(x as f32 - 64., 0.9, y as f32 - 64.),
                                rotation: Quat::from_rotation_y(45_f32.to_radians()),
                                scale: Vec3::splat(1.0),
                            },
                            ..default()
                        },
                        // Collider cubique
                        Collider::cuboid(TILE_SIZE.sqrt(), 5., 0.6),
                        RigidBody::Fixed, // Le cube est immobile
                    )).insert(ActiveEvents::COLLISION_EVENTS);
                }
                '\\' => {
                    commands.spawn((
                        MaterialMeshBundle {
                            mesh: diagonal_l_wall.clone(),
                            material: material.clone(),
                            // transform: Transform::from_xyz(x as f32 -64., 0.9, y as f32 -64.),
                            transform: Transform {
                                translation: Vec3::new(x as f32 - 64., 0.9, y as f32 - 64.),
                                rotation: Quat::from_rotation_y(135_f32.to_radians()),
                                scale: Vec3::splat(1.0),
                            },
                            ..default()
                        },
                        // Collider cubique
                        Collider::cuboid(TILE_SIZE.sqrt(), 5., 0.6),
                        RigidBody::Fixed, // Le cube est immobile
                    )).insert(ActiveEvents::COLLISION_EVENTS);
                }
                '0' => {
                    commands.spawn((
                        MaterialMeshBundle {
                            mesh: horizontal_wall.clone(),
                            material: material.clone(),
                            transform: Transform::from_xyz(x as f32 - 64., 0.9, y as f32 - 64.),
                            ..default()
                        },
                        // Collider cubique
                        Collider::cuboid(0.6, 5., 0.6),
                        RigidBody::Fixed, // Le cube est immobile
                    )).insert(ActiveEvents::COLLISION_EVENTS);
                    commands.spawn((
                        MaterialMeshBundle {
                            mesh: vertical_wall.clone(),
                            material: material.clone(),
                            transform: Transform::from_xyz(x as f32 - 64., 0.9, y as f32 - 64.),
                            ..default()
                        },
                        // Collider cubique
                        Collider::cuboid(0.6, 5., 0.6),
                        RigidBody::Fixed, // Le cube est immobile
                    )).insert(ActiveEvents::COLLISION_EVENTS);
                }
                _ => {}
            }
        }
    }

    // The world model camera will render the floor and the cubes spawned in this system.
    // Assigning no `RenderLayers` component defaults to layer 0.
    commands
        .spawn((
            MaterialMeshBundle {
                mesh: floor.clone(),
                // material: material.clone(),
                material: _temp_mat,
                ..default()
            },
            // Collider cubique
            Collider::cuboid(64., 0.1, 64.),
            RigidBody::Fixed, // Le sol est immobile
                              // RenderLayers::from_layers(&[0, 2]),
        ))
        .insert(Ground);

    commands
        .spawn((
            MaterialMeshBundle {
                mesh: roof.clone(),
                material: material.clone(),
                transform: Transform::from_xyz(0.0, 10.0, 0.0),
                ..default()
            },
            // Collider cubique
            Collider::cuboid(64., 0.1, 64.),
            RigidBody::Fixed, // Le sol est immobile
        ))
        .insert(Roof);
}

pub fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                color: Color::from(tailwind::ORANGE_300),
                // shadows_enabled: true,
                // radius: 90.,
                intensity: 20000000.,
                range: 300000000.,
                ..default()
            },
            transform: Transform::from_xyz(-32.0, 11.0, -32.),
            ..default()
        },
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                color: Color::from(tailwind::ORANGE_300),
                // shadows_enabled: true,
                // radius: 90.,
                intensity: 20000000.,
                range: 300000000.,
                ..default()
            },
            transform: Transform::from_xyz(32.0, 11.0, 32.),
            ..default()
        },
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                color: Color::from(tailwind::ORANGE_300),
                // shadows_enabled: true,
                // radius: 90.,
                intensity: 20000000.,
                range: 300000000.,
                ..default()
            },
            transform: Transform::from_xyz(32.0, 11.0, -32.),
            ..default()
        },
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                color: Color::from(tailwind::ORANGE_300),
                // shadows_enabled: true,
                // radius: 90.,
                intensity: 20000000.,
                range: 300000000.,
                ..default()
            },
            transform: Transform::from_xyz(-32.0, 11.0, 32.),
            ..default()
        },
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
    commands.insert_resource(AmbientLight {
        color: Color::from(tailwind::ORANGE_800),
        brightness: 10.,
    });

    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         illuminance: 10_000.,
    //         shadows_enabled: false,
    //         ..default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(0., 0., 0.),
    //         rotation: Quat::from_rotation_x(-90_f32.to_radians()),
    //             // * Quat::from_rotation_y(45.0_f32.to_radians()),
    //         ..default()
    //     },
    //     cascade_shadow_config: CascadeShadowConfigBuilder {
    //         maximum_distance: 2500.,
    //         minimum_distance: 0.2,
    //         num_cascades: 3,
    //         first_cascade_far_bound: 200.,
    //         ..default()
    //     }
    //     .into(),
    //     ..default()
    // });
}
