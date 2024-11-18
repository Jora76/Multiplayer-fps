use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy::{
    app::{App, Plugin, Update},
    math::Vec3,
    prelude::{
        Commands, Component, EventReader, ResMut, Resource, Transform,
    },
};
use renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ClientId, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
};
use serde::{Deserialize, Serialize};

use crate::{player::PlayerData, test::HostState};

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<ClientId, PlayerData>,
}

pub struct Server;

impl Plugin for Server {
    fn build(&self, app: &mut App) {
        // app.init_state::<HostState>();
        let server = RenetServer::new(ConnectionConfig::default());
        // let (server, server_transport) = new_renet_server();
        app.insert_resource(server);
        // app.insert_resource(server_transport);
        app.init_resource::<Lobby>();
        app.init_resource::<HostState>();
        app.add_systems(Update, (server_update_system, server_centralize_messages));
    }
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { id: ClientId, position: Vec3 },
    PlayerDisconnected { id: ClientId },
    PlayerMoved { id: ClientId, position: Vec3 },
    ProjectileSpawned { id: ClientId, position: Vec3, direction: Vec3  },
    TestMessage { message: String },
    PlayerDeath { id: ClientId }
    // ProjectileMoved { id: ClientId, position: Vec3},
}

const PROTOCOL_ID: u64 = 7;
// use crate::player::Player;

pub fn new_renet_server() -> NetcodeServerTransport {
    println!("Creating server");
    // let public_addr = "192.168.101.234:5000".parse().unwrap();
    let public_addr = (local_ip_address::local_ip().unwrap().to_string() + ":5000").parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    // let server = RenetServer::new(ConnectionConfig::default());

    transport
}

pub fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
    mut host_state: ResMut<HostState>,
) {
    if host_state.is_host && !host_state.is_host_initialized {
        let server_transport = new_renet_server();
        commands.insert_resource(server_transport);
        host_state.is_host_initialized = true;
    } else {
        // Handle server events
        for event in server_events.read() {
            match event {
                ServerEvent::ClientConnected { client_id} => {
                    // println!("Server side : Player {} connected.", client_id);
                    // Envoie les données des joueurs connectés au nouveau joueur
                    lobby
                        .players
                        .insert(*client_id, PlayerData::new(*client_id));
                    for &player_id in lobby.players.keys() {
                        let message =
                            bincode::serialize(&ServerMessages::PlayerConnected { id: player_id, position: lobby.players[&player_id].position.translation })
                                .unwrap();
                        server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);
                    }

                    // Envoie la nouvelle connexion aux joueurs déjà connectés
                    let message =
                        bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id, position: lobby.players[client_id].position.translation })
                            .unwrap();
                    server.broadcast_message(DefaultChannel::ReliableOrdered, message);
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!(
                        "Server side : Player {} disconnected: {}",
                        client_id, reason
                    );
                    // if let Some(player_entity) = lobby.players.remove(client_id) {
                    //     commands.entity(player_entity).despawn();
                    // }
                    lobby.players.remove(client_id);
                    let message =
                        bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id })
                            .unwrap();
                    server.broadcast_message(DefaultChannel::ReliableOrdered, message);
                }
            }
        }
    }
}

pub fn server_centralize_messages(
    mut server: ResMut<RenetServer>,
    mut lobby: ResMut<Lobby>,
    host_state: ResMut<HostState>,
) {
    if !host_state.is_host {
        return;
    }
    for client_id in server.clients_id() {
        // println!("Checking messages for client {}", client_id);
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            let server_message: ServerMessages = bincode::deserialize(&message).unwrap();
            match server_message {
                ServerMessages::PlayerMoved { id, position } => {
                    if let Some(player) = lobby.players.get_mut(&id) {
                        player.position = Transform::from_xyz(position.x, position.y, position.z);
                        let message =
                            bincode::serialize(&ServerMessages::PlayerMoved { id, position })
                                .unwrap();
                        server.broadcast_message(DefaultChannel::ReliableOrdered, message);
                        // println!("Player {} moved to {:?}", id, position);
                    }
                }
                ServerMessages::TestMessage { message } => {
                    println!("Message test from client {}: {}", client_id, message);
                }
                ServerMessages::ProjectileSpawned { id, position, direction } => {
                    // println!(
                    //     "Server side : projectiles id :  {} position: {}",
                    //     client_id, position
                    // );
                    let message =
                        bincode::serialize(&ServerMessages::ProjectileSpawned { id, position, direction })
                            .unwrap();
                    server.broadcast_message(DefaultChannel::ReliableOrdered, message);
                }
                ServerMessages::PlayerDeath { id } => {
                    let message =
                        bincode::serialize(&ServerMessages::PlayerDeath { id })
                            .unwrap();
                    server.broadcast_message(DefaultChannel::ReliableOrdered, message);
                }
                _ => {}
            }
        }
    }
}
