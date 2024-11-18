use bevy::prelude::*;

pub fn create_settings_menu(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column, // Stack buttons vertically
            justify_content: JustifyContent::Center, // Center the buttons in the container
            align_items: AlignItems::Center,       // Align the buttons horizontally
            ..Default::default()
        },
        ..Default::default()
    });
}
