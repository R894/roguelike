use bevy::prelude::*;

use crate::{
    pieces::components::{Gold, Health},
    player::Player,
};

use super::{UiFont, UiGold, UiHealth};

pub fn spawn_ui(mut commands: Commands, font: Res<UiFont>) {
    let health = spawn_health_ui(&mut commands, &font);

    let gold = spawn_gold_ui(&mut commands, &font);

    let node_bundle = NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::FlexEnd,
            row_gap: Val::Px(10.),
            column_gap: Val::Px(10.),
            padding: UiRect::all(Val::Px(14.)),
            ..default()
        },
        ..default()
    };

    commands.spawn(node_bundle).push_children(&[health, gold]);
}

fn spawn_health_ui(commands: &mut Commands, font: &Res<UiFont>) -> Entity {
    commands
        .spawn(TextBundle::from_section(
            "Health: ",
            TextStyle {
                font: font.0.clone(),
                font_size: 30.0,
                color: Color::srgb(0.7, 0.7, 0.7),
            },
        ))
        .insert(UiHealth)
        .id()
}

fn spawn_gold_ui(commands: &mut Commands, font: &Res<UiFont>) -> Entity {
    commands
        .spawn(TextBundle::from_section(
            "Gold: ",
            TextStyle {
                font: font.0.clone(),
                font_size: 30.0,
                color: Color::srgb(0.7, 0.7, 0.7),
            },
        ))
        .insert(UiGold)
        .id()
}

pub fn update_ui_health(
    mut text_query: Query<&mut Text, With<UiHealth>>,
    health_query: Query<&Health, With<Player>>,
) {
    let health = health_query
        .get_single()
        .unwrap_or(&Health { current: 0, max: 0 });
    for mut text in &mut text_query {
        text.sections[0].value = format!("Health: {}", health.current);
    }
}

pub fn update_ui_gold(
    mut text_query: Query<&mut Text, With<UiGold>>,
    gold_query: Query<&Gold, With<Player>>,
) {
    let gold = gold_query.get_single().unwrap_or(&Gold { value: 0 });
    for mut text in &mut text_query {
        text.sections[0].value = format!("Gold: {}", gold.value);
    }
}
