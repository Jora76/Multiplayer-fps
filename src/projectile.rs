use std::time::SystemTime;

use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
// use bevy_rapier3d::{prelude::{ActiveEvents, Collider, CollisionEvent}, rapier::prelude::RigidBody};
use bevy_rapier3d::prelude::*;
use rand::Rng;
use renet::{transport::NetcodeClientTransport, RenetClient};

use crate::{client::client_send_projectile_position, player::{Player, PlayerState}, weapon::Weapon};

#[derive(Debug, Component)]
pub struct Projectile;

#[derive(Debug, Component)]
pub struct ProjectilePosition {
    pub direction: Vec3,
    pub speed: f32,
    pub projectile_id: u64,
}

#[derive(Debug, Component)]
pub struct Lifetime {
    pub timer: Timer,
}

pub fn spawn_projectile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    weapon_query: Query<(&Transform, &Parent), With<Weapon>>,
    player_query: Query<&Transform, With<Player>>,
    mut client: ResMut<RenetClient>,
    transport: ResMut<NetcodeClientTransport>,
) {
    for (weapon_transform, parent) in weapon_query.iter() {
        if let Ok(player_transform) = player_query.get(parent.get()) {
            let weapon_end_offset = weapon_transform.rotation * Vec3::new(0.0, 0., -0.5);
            let weapon_end_position = weapon_transform.translation + weapon_end_offset;

            let spawn_position =
                player_transform.translation + player_transform.rotation * weapon_end_position;

            let mut direction = player_transform.forward().as_vec3(); // Forward direction of the player
            direction.y += 0.03;

            let mut rng = rand::thread_rng();
            let random_offset = Vec3::new(
                rng.gen_range(-0.01..0.01),
                rng.gen_range(-0.01..0.01),
                rng.gen_range(-0.01..0.01),
            );

            direction += random_offset;

            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Sphere::new(0.01)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::srgb(1., 1., 0.),
                        emissive: LinearRgba::rgb(10.0, 10., 0.),
                        ..Default::default()
                    }),
                    transform: Transform::from_translation(spawn_position),
                    ..Default::default()
                })
                .insert(Projectile)
                .insert(ProjectilePosition {
                    direction,
                    speed: 50.0, // Vitesse du projectile
                    // speed: 0.5,
                    projectile_id: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                })
                .insert(Lifetime {
                    timer: Timer::from_seconds(10.0, TimerMode::Once), // Durée de vie du projectile
                })
                .insert(Collider::ball(0.01))
                .insert(RigidBody::Dynamic)
                .insert(NotShadowCaster)
                .insert(ActiveEvents::COLLISION_EVENTS);
            client_send_projectile_position(spawn_position, &mut *client, &*transport, direction);
        }
    }
}

pub fn update_projectiles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &ProjectilePosition, &mut Lifetime),
        With<Projectile>,
    >,
) {
    for (entity, mut transform, projectile_position, mut lifetime) in query.iter_mut() {
        let direction = projectile_position.direction;
        transform.translation += direction * projectile_position.speed * time.delta_seconds();

        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            // entities.projectiles.remove(projectile_position.projectile_id);
            commands.entity(entity).despawn();
        }
    }
}

pub fn detect_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<Entity, With<Projectile>>,
    wall_query: Query<Entity, (Without<Projectile>, Without<Player>)>,
    player_query: Query<Entity, With<Player>>,
    mut player_state: ResMut<PlayerState>,
) {
    for event in collision_events.read() {
        // println!("Event detected");
        match event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                // println!("Started");
                let is_projectile1 = projectile_query.get(*entity1).is_ok();
                let is_wall2 = wall_query.get(*entity2).is_ok();
                let is_projectile2 = projectile_query.get(*entity2).is_ok();
                let is_wall1 = wall_query.get(*entity1).is_ok();
                let is_player1 = player_query.get(*entity1).is_ok();
                let is_player2 = player_query.get(*entity2).is_ok();
                if is_projectile1 && is_wall2 {
                    //    println!("Projectile hit wall!");
                    // commands.entity(*entity1).despawn();
                } else if is_projectile2 && is_wall1 {
                    //    println!("Projectile hit wall!");
                    // commands.entity(*entity2).despawn();
                } else if is_projectile1 && is_player2 {
                    *player_state = PlayerState::Dead;
                    println!("Projectile hit player!");
                    // commands.entity(*entity1).despawn();
                    // commands.entity(*entity2).despawn();
                } else if is_projectile2 && is_player1 {
                    *player_state = PlayerState::Dead;
                    println!("Projectile hit player!");
                    // commands.entity(*entity2).despawn();
                    // commands.entity(*entity1).despawn();
                } else {
                    println!("Toz");
                }
            }
            CollisionEvent::Stopped(_, _, _) => {
                //println!("Stopped");
            }
        }
    }
}
