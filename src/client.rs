use bevy::{
    app::{App, Plugin, Update},
    asset::Assets,
    color::{Color, LinearRgba},
    math::Vec3,
    pbr::{NotShadowCaster, PbrBundle, StandardMaterial},
    prelude::{
        in_state, Capsule3d, Commands, Entity, IntoSystemConfigs, Mesh, ResMut, Resource, Sphere, Transform
    },
    time::{Timer, TimerMode},
};
use bevy_rapier3d::prelude::{ActiveEvents, Collider, RigidBody};
use renet::{
    transport::{ClientAuthentication, NetcodeClientTransport},
    ClientId, ConnectionConfig, DefaultChannel, RenetClient,
};
use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use crate::server::{Lobby, ServerMessages};
use crate::{
    projectile::{Lifetime, Projectile, ProjectilePosition},
    test,
};

#[derive(Debug, Default, Resource)]
pub struct Entities {
    pub players: HashMap<ClientId, Entity>,
    // pub projectiles: HashMap<ClientId, Entity>,
}

pub struct Client;

impl Plugin for Client {
    fn build(&self, app: &mut App) {
        let (client, client_transport) = new_renet_client();
        app.insert_resource(client);
        app.insert_resource(client_transport);
        app.init_resource::<Entities>();
        app.add_systems(
            Update,
            client_sync_players.run_if(in_state(test::GameState::Game)),
        );
    }
}

const PROTOCOL_ID: u64 = 7;

pub fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let server_addr = (local_ip_address::local_ip().unwrap().to_string() + ":5000").parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(ConnectionConfig::default());

    (client, transport)
}

pub fn client_sync_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    transport: ResMut<NetcodeClientTransport>,
    mut entities: ResMut<Entities>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { id, position } => {
                // println!("Client side : hashmap player: {:?}", entities.players);
                if id != transport.client_id() {
                    let player_entity = commands
                        .spawn(PbrBundle {
                            mesh: meshes.add(Capsule3d::new(0.3, 1.8)),
                            material: materials.add(StandardMaterial {
                                base_color: Color::srgb(0.8, 0.7, 0.6),
                                ..Default::default()
                            }),
                            transform: Transform::from_translation(position),
                            ..Default::default()
                        })
                        .insert(Collider::round_cylinder(0.9, 0.3, 0.2))
                        .insert(RigidBody::Dynamic)
                        .insert(NotShadowCaster)
                        .insert(ActiveEvents::COLLISION_EVENTS)
                        .id();

                    // Ajouter l'entité à la hashmap
                    entities.players.insert(id, player_entity);
                    let message =
                        bincode::serialize(&ServerMessages::PlayerConnected { id, position })
                            .unwrap();
                    client.send_message(DefaultChannel::ReliableOrdered, message);
                }
            }
            ServerMessages::PlayerDisconnected { id } => {
                println!("Client side : Player {} disconnected.", id);
                lobby.players.remove(&id);
            }
            ServerMessages::PlayerMoved { id, position } => {
                if let Some(player_entity) = entities.players.get(&id) {
                    commands
                        .entity(*player_entity)
                        .insert(Transform::from_translation(position));
                    // println!("Client side : Player {} moved to {:?}", id, position);
                }
            }
            ServerMessages::ProjectileSpawned {
                id,
                position,
                direction,
            } => {
                // println!("Projectile spawned from server id: {:?} et position: {:?}, client receiver id : {}", id, position, transport.client_id());
                if id != transport.client_id() {
                    commands
                        .spawn(PbrBundle {
                            mesh: meshes.add(Sphere::new(0.01)),
                            material: materials.add(StandardMaterial {
                                base_color: Color::srgb(1., 1., 0.),
                                emissive: LinearRgba::rgb(10.0, 10., 0.),
                                ..Default::default()
                            }),
                            transform: Transform::from_translation(position),
                            ..Default::default()
                        })
                        .insert(Projectile)
                        .insert(ProjectilePosition {
                            direction,
                            speed: 70.0, // Vitesse du projectile
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

                    // Ajouter l'entité à la hashmap
                    // entities.projectiles.insert(id, projectile_entity);
                }
            }
            ServerMessages::TestMessage { message } => {
                println!("Client side : Message test from server : {}", message);
            }
            ServerMessages::PlayerDeath { id } => {
                if id != transport.client_id() {
                    if let Some(player_entity) = entities.players.get(&id) {
                        commands.entity(*player_entity).despawn();
                        entities.players.remove(&id);
                    }
                }
            }
        }
    }
}

pub fn client_send_input(
    client_position: Vec3,
    mut client: ResMut<RenetClient>,
    transport: ResMut<NetcodeClientTransport>,
) {
    let input_message = bincode::serialize(&ServerMessages::PlayerMoved {
        id: transport.client_id(),
        position: client_position,
    })
    .unwrap();

    client.send_message(DefaultChannel::ReliableOrdered, input_message);
}

pub fn client_send_position(
    client_position: Vec3,
    mut client: ResMut<RenetClient>,
    transport: ResMut<NetcodeClientTransport>,
) {
    // println!("le joueur (clients) 1 {:?}", client_position);
    let input_message = bincode::serialize(&ServerMessages::PlayerMoved {
        id: transport.client_id(),
        position: client_position,
    })
    .unwrap();
    client.send_message(DefaultChannel::ReliableOrdered, input_message);
}

pub fn client_send_projectile_position(
    projectile_position: Vec3,
    client: &mut RenetClient,
    transport: &NetcodeClientTransport,
    direction: Vec3,
) {
    // println!("Projectile position: {:?}", projectile_position);
    let input_message = bincode::serialize(&ServerMessages::ProjectileSpawned {
        id: transport.client_id(),
        position: projectile_position,
        direction,
    })
    .unwrap();
    client.send_message(DefaultChannel::ReliableOrdered, input_message);
}

// pub fn client_send_he_is_dead(
//     mut client: ResMut<RenetClient>,
//     transport: ResMut<NetcodeClientTransport>,
// ) {
//     // println!("le joueur (clients) 1 {:?}", client_position);
//     let input_message = bincode::serialize(&ServerMessages::PlayerDeath {
//         id: transport.client_id(),
//     })
//     .unwrap();
//     client.send_message(DefaultChannel::ReliableOrdered, input_message);
// }
