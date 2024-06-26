use bevy::prelude::*;

use crate::ui::{spawn_textbox, UiFont};

use super::Menu;

pub fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>, font: Res<UiFont>) {
    let start_button = spawn_textbox(&mut commands, &asset_server, "Start", (150., 75.));
    let text = TextBundle {
        style: Style {
            width: Val::Auto,
            height: Val::Auto,
            ..default()
        },
        text: Text {
            sections: vec![TextSection {
                value: "Roguelike".to_string(),
                style: TextStyle {
                    font: font.0.clone(),
                    font_size: 90.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            }],
            ..default()
        },
        ..default()
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                column_gap: Val::Px(30.0),
                row_gap: Val::Px(30.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(Menu)
        .with_children(|parent| {
            parent.spawn(text);
        })
        .push_children(&[start_button]);
}

pub fn game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>, font: Res<UiFont>) {
    let text = TextBundle {
        style: Style {
            width: Val::Auto,
            height: Val::Auto,
            ..default()
        },
        text: Text {
            sections: vec![TextSection {
                value: "Game Over!".to_string(),
                style: TextStyle {
                    font: font.0.clone(),
                    font_size: 90.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            }],
            ..default()
        },
        ..default()
    };
    let restart_button = spawn_textbox(&mut commands, &asset_server, "Restart", (150., 75.));

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                column_gap: Val::Px(30.0),
                row_gap: Val::Px(30.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(Menu)
        .with_children(|parent| {
            parent.spawn(text);
        })
        .push_children(&[restart_button]);
}

pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
