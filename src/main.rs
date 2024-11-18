use std::io::{self, Write};

use bevy::input::InputSystem;
use bevy::prelude::*;
use bevy::window::{Cursor, WindowMode, WindowPlugin};
use bevy_fps_ui::FpsCounterPlugin;
use bevy_rapier3d::prelude::NoUserData;
// use bevy_rapier3d::prelude::RapierDebugRenderPlugin;
use bevy_rapier3d::prelude::RapierPhysicsPlugin;

use game_test::camera;
use game_test::keybind::KeyBinds;
use game_test::{map, test};
use bevy_renet::*;
use game_test::{server, client};
use game_test::{player, projectile, weapon};
use transport::NetcodeClientPlugin;
use transport::NetcodeServerPlugin;

fn main() {
    print!("Saisissez votre Username: ");
    io::stdout().flush().unwrap(); // Assurez-vous que le message est affiché avant de lire l'entrée

    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");

    // Supprimer le caractère de nouvelle ligne à la fin de la chaîne
    // let username = username.trim();
    
    let mut cursor = Cursor::default();
    cursor.visible = true;
    // let (server, server_transport) = server::new_renet_server();
    
    App::new()
        // STATES ###############################################
        .init_resource::<player::PlayerState>()
        // INIT RESSOURCES ###########################################
        .init_resource::<player::MovementInput>()
        .insert_resource(KeyBinds::default())
        .init_resource::<player::LookInput>()
        // PLUGINS ###############################################
        .add_plugins(server::Server)
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .add_plugins(client::Client)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                cursor,
                // resizable: false,
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            FpsCounterPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
        ))
        //INIT STATE #############################################
        .init_state::<test::GameState>()
        // MENU #########################################################
        .add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        .add_systems(
            OnEnter(test::GameState::Menu),
            (test::render_main_menu, camera::spawn_view_model),
        )
        .add_systems(
            Update,
            test::button_system.run_if(in_state(test::GameState::Menu)),
        )
        .add_systems(
            OnExit(test::GameState::Menu),
            (test::clear, test::hide_cursor),
        )
        // GAME ######################################################################
        .add_systems(
            OnEnter(test::GameState::Game),
            (
                camera::spawn_crosshair,
                map::spawn_world_model,
                map::spawn_lights,
                camera::spawn_crosshair,
            ),
        )
        .add_systems(
            PreUpdate,
            player::handle_input
                .after(InputSystem)
                .run_if(in_state(test::GameState::Game)),
        )
        .add_systems(
            Update,
            (
                player::handle_input.run_if(in_state(test::GameState::Game)),
                camera::move_camera.run_if(in_state(test::GameState::Game)),
                weapon::pew.run_if(in_state(test::GameState::Game)),
                projectile::detect_collisions.run_if(in_state(test::GameState::Game)),
                player::la_mooooooooooort.run_if(in_state(test::GameState::Game)),
            )
                .run_if(client_connected),
        )
        .add_systems(
            FixedUpdate,
            (
                projectile::update_projectiles.run_if(in_state(test::GameState::Game)),
                player::player_movement.run_if(in_state(test::GameState::Game)),
                weapon::update_arm.run_if(in_state(test::GameState::Game)),
                camera::move_camera.run_if(in_state(test::GameState::Game)),
            ),
        )
        .add_systems(
            PostUpdate,
            (
                camera::update_minimap.run_if(in_state(test::GameState::Game)),
                camera::update_minimap_camera_rotation.run_if(in_state(test::GameState::Game)),
                projectile::update_projectiles.run_if(in_state(test::GameState::Game)),
                player::player_movement.run_if(in_state(test::GameState::Game)),
            ),
        )
        .run();
}
