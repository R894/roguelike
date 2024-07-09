use bevy::prelude::*;

use crate::{
    states::MainState,
    ui::{spawn_button, BorderTexture, TextBox, UiFont},
};

use super::Menu;

pub fn main_menu(mut commands: Commands, border_texture: Res<BorderTexture>, font: Res<UiFont>) {
    let start_button = spawn_button(&mut commands, &border_texture, &font, "Start", (150., 75.));
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
                    color: Color::srgb(0.9, 0.9, 0.9),
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

pub fn game_over_menu(
    mut commands: Commands,
    border_texture: Res<BorderTexture>,
    font: Res<UiFont>,
) {
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
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            }],
            ..default()
        },
        ..default()
    };
    let restart_button = spawn_button(
        &mut commands,
        &border_texture,
        &font,
        "Restart",
        (150., 75.),
    );

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

pub fn menu_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<TextBox>)>,
    mut state: ResMut<NextState<MainState>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                state.set(MainState::Game);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
