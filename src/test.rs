use bevy::{prelude::*, window::PrimaryWindow};
use renet::RenetClient;

// #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
// pub enum HostState {
//     Host,
//     #[default]
//     Join,
// }

#[derive(Component, Resource)]
pub struct HostState {
    pub is_host: bool,
    pub is_host_initialized: bool,
}

impl Default for HostState {
    fn default() -> Self {
        HostState {
            is_host: false,
            is_host_initialized: false,
        }
    }
}

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Game,
    Pause,
}
#[derive(Component)]
pub enum Buttons {
    Start,
    Join,
    Create,
    Back,
    Settings,
    Quit,
}

#[warn(dead_code)]
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct _Volume(u32);

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct MenuCamera;

#[derive(Component)]
pub struct StartMenu;

#[derive(Component)]
pub struct SettingsMenu;

#[derive(Component)]
pub enum PlayerKeys {
    Forward,
    Backward,
    Left,
    Right,
    Jump,
    Sprint,
    Shoot,
    Aim,
}

pub fn render_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    create_main_menu(&mut commands, &asset_server);
}

pub fn create_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // Bump the order to render on top of the world model.
                order: 3,
                ..default()
            },
            ..default()
        },
        MenuCamera,
    ));

    // clear_color.0 = Color::srgb(196.0, 203.0, 203.0);

    let main_menu = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    };

    let default_button = ButtonBundle {
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(80.0),
            margin: UiRect::vertical(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    let start_text = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Start".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            }],
            ..Default::default()
        },
        ..default()
    };

    let settings_text = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Settings".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            }],
            ..Default::default()
        },
        ..default()
    };

    let quit_text = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Quit".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            }],
            ..Default::default()
        },
        ..default()
    };

    commands
        .spawn(main_menu)
        .insert(MainMenu)
        .with_children(|parent| {
            parent
                .spawn(default_button.clone())
                .insert(Buttons::Start)
                .with_children(|button| {
                    button.spawn(start_text);
                });
        })
        .with_children(|parent| {
            parent
                .spawn(default_button.clone())
                .insert(Buttons::Settings)
                .with_children(|button| {
                    button.spawn(settings_text);
                });
        })
        .with_children(|parent| {
            parent
                .spawn(default_button.clone())
                .insert(Buttons::Quit)
                .with_children(|button| {
                    button.spawn(quit_text);
                });
        });
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Buttons),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    query: Query<Entity, With<MainMenu>>,
    query_camera: Query<Entity, With<MenuCamera>>,
    query_start: Query<Entity, With<StartMenu>>,
    query_settings: Query<Entity, With<SettingsMenu>>,
    asset_server: Res<AssetServer>,
    client: Res<RenetClient>,
    mut host_state: ResMut<HostState>,
) {
    for (interaction, mut color, button_action) in interaction_query.iter_mut() {
        // let mut error_text: EntityCommands;
        match *interaction {
            Interaction::Pressed => {
                // Change color when clicked
                *color = BackgroundColor(Color::srgba(0.35, 0.75, 0.35, 1.));

                // Perform action based on which button was clicked
                match button_action {
                    Buttons::Start => {
                        println!("Button 1 clicked! Perform action for Button 1.");
                        clear_main_menu(&mut commands, &query);
                        create_start_menu(&mut commands, &asset_server);
                    }
                    Buttons::Settings => {
                        println!("Button 2 clicked! Perform action for Button 2.");
                        clear_main_menu(&mut commands, &query);
                        create_settings_menu(&mut commands, &asset_server)
                    }
                    Buttons::Quit => {
                        println!("Button 3 clicked! Perform action for Button 3.");
                        close(&mut app_exit_events)
                    }

                    Buttons::Join => {
                        if client.is_connected() {
                            next_state.set(GameState::Game);
                        } else {
                            commands.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Error when trying to connect to the server"
                                            .to_string(),
                                        style: TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 40.0,
                                            color: Color::srgb(1., 0., 0.),
                                        },
                                    }],
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        }
                    }
                    Buttons::Create => {
                        host_state.is_host = true;
                        next_state.set(GameState::Game);
                        // error_text.despawn_recursive();
                    }
                    Buttons::Back => {
                        clear_start_menu(&mut commands, &query_start);
                        clear_settings_menu(&mut commands, &query_settings);
                        clear_menu_camera(&mut commands, &query_camera);
                        create_main_menu(&mut commands, &asset_server);
                    }
                }
            }
            Interaction::Hovered => {
                // Change color when hovered
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                // Reset color when not hovered or clicked
                *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
            }
        }
    }
}

fn close(app_exit_events: &mut EventWriter<AppExit>) {
    app_exit_events.send(AppExit::default());
}

pub fn clear(
    mut commands: Commands,
    query_menu: Query<Entity, With<MainMenu>>,
    query_camera: Query<Entity, With<MenuCamera>>,
    query_start: Query<Entity, With<StartMenu>>,
) {
    clear_main_menu(&mut commands, &query_menu);
    clear_start_menu(&mut commands, &query_start);
    clear_menu_camera(&mut commands, &query_camera);
}

pub fn clear_main_menu(commands: &mut Commands, query: &Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn clear_start_menu(commands: &mut Commands, query: &Query<Entity, With<StartMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn clear_settings_menu(commands: &mut Commands, query: &Query<Entity, With<SettingsMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn clear_menu_camera(commands: &mut Commands, query: &Query<Entity, With<MenuCamera>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    println!("camera menu clear")
}

pub fn hide_cursor(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.visible = false;
    }
}

pub fn create_start_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let start_menu = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    let default_button = ButtonBundle {
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(80.0),
            margin: UiRect::vertical(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    let join_text = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Join".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            }],
            ..Default::default()
        },
        ..default()
    };

    let create_text = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Create".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            }],
            ..Default::default()
        },
        ..default()
    };

    let back_text = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Back".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            }],
            ..Default::default()
        },
        ..default()
    };

    commands
        .spawn(start_menu)
        .insert(StartMenu)
        .with_children(|parent| {
            parent
                .spawn(default_button.clone())
                .insert(Buttons::Join)
                .with_children(|button| {
                    button.spawn(join_text);
                });
        })
        .with_children(|parent| {
            parent
                .spawn(default_button.clone())
                .insert(Buttons::Create)
                .with_children(|button| {
                    button.spawn(create_text);
                });
        })
        .with_children(|parent| {
            parent
                .spawn(default_button.clone())
                .insert(Buttons::Back)
                .with_children(|button| {
                    button.spawn(back_text);
                });
        });
}

pub fn create_settings_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let settings_menu = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    };

    NodeBundle {
        style: Style {
            width: Val::Px(300.0),
            height: Val::Px(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect {
                left: Val::Px(10.0),
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                bottom: Val::Px(10.0),
            },
            ..default()
        },
        // background_color: BackgroundColor(Color::srgb(100.0, 100.0, 100.0)),
        ..default()
    };

    ButtonBundle {
        style: Style {
            width: Val::Px(90.0),
            height: Val::Px(50.0),
            margin: UiRect::vertical(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        border_color: BorderColor(Color::srgb(233.0, 233.0, 233.0)),
        border_radius: BorderRadius {
            top_left: Val::Px(8.0),
            top_right: Val::Px(8.0),
            bottom_left: Val::Px(8.0),
            bottom_right: Val::Px(8.0),
        },
        background_color: BackgroundColor(Color::srgb(255.0, 0.0, 0.0)),
        ..default()
    };

    let _forward_text = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Foward".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            }],
            ..Default::default()
        },
        ..default()
    };

    TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "S".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::BLACK,
                },
            }],
            ..Default::default()
        },
        ..default()
    };

    TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Left".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            }],
            ..Default::default()
        },
        ..default()
    };

    TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Backward".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            }],
            ..Default::default()
        },
        ..default()
    };

    ButtonBundle {
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(80.0),
            margin: UiRect::vertical(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Back".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            }],
            ..Default::default()
        },
        ..default()
    };

    let settings_menu_entity = commands.spawn(settings_menu).insert(SettingsMenu).id();

    let zqsd = create_zqsd(commands, &asset_server);

    commands
        .entity(settings_menu_entity)
        .with_children(|parent| {
            parent.spawn(zqsd);
        });
    // .with_children(|main_node| {
    //     main_node
    //         .spawn(default_key_node.clone())
    //         .with_children(|parent| {
    //             parent
    //                 .spawn(default_key_button.clone())
    //                 .insert(PlayerKeys::Forward)
    //                 .with_children(|parent| {
    //                     parent.spawn(forward_text);
    //                 });
    //         });
    // })
    // .with_children(|main_node| {
    //     main_node
    //         .spawn(default_key_node.clone())
    //         .with_children(|parent| {
    //             parent.spawn(test_text);
    //         })
    //         .with_children(|parent| {
    //             parent
    //                 .spawn(default_key_button.clone())
    //                 .insert(PlayerKeys::Backward)
    //                 .with_children(|parent| {
    //                     parent.spawn(backward_text);
    //                 });
    //         });
    // })
    // .with_children(|parent| {
    //     parent
    //         .spawn(default_key_button.clone())
    //         .insert(PlayerKeys::Left)
    //         .with_children(|parent| {
    //             parent.spawn(left_text);
    //         });
    // })
    // .with_children(|parent| {
    //     parent.spawn(default_key_button.clone());
    // })
    // .with_children(|parent| {
    //     parent.spawn(default_key_button.clone());
    // })
    // .with_children(|parent| {
    //     parent
    //         .spawn(default_button.clone())
    //         .insert(Buttons::Back)
    //         .with_children(|button| {
    //             button.spawn(back_text);
    //         });
    // });
}

pub fn create_zqsd(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let key_button = ButtonBundle {
        style: Style {
            width: Val::Px(70.),
            height: Val::Px(70.),
            ..default()
        },
        border_color: BorderColor(Color::srgb(182., 182., 182.)),
        ..default()
    };

    let key_text = |text: String| TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: text,
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            }],
            ..Default::default()
        },
        ..default()
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(210.0),
                height: Val::Px(140.0),
                display: Display::Grid,
                grid_template_columns: vec![
                    GridTrack::fr(1.0),
                    GridTrack::fr(1.0),
                    GridTrack::fr(1.0),
                ],
                grid_template_rows: vec![GridTrack::fr(1.0), GridTrack::fr(1.0)],
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(67., 67., 67.)),
            ..default()
        })
        // Z button
        .with_children(|main_node| {
            main_node.spawn(key_button.clone()).with_children(|button| {
                button.spawn(key_text("Z".to_string()));
            });
        });
}
