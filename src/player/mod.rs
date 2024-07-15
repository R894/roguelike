pub mod inventory;

use bevy::prelude::*;
use inventory::Inventory;

use crate::{
    board::{components::Position, systems::spawn_map, ValidSpots},
    graphics::TILE_SIZE,
    pieces::{
        components::{Actor, Gold, Health, ItemPicker, Melee, Occupier, Piece},
        equipment::{ChestArmor, Equipment, Sword},
    },
    states::MainState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_player.after(spawn_map));
    }
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, valid_spots: Res<ValidSpots>) {
    commands.spawn((
        Actor::default(),
        Player,
        Occupier,
        ItemPicker,
        Equipment { ..default() },
        Health {
            max: 10,
            current: 10,
        },
        Melee { damage: 5 },
        Inventory {
            items: vec![Box::new(Sword::default()), Box::new(ChestArmor::default())],
        },
        Piece {
            kind: "Player".to_string(),
        },
        Gold { value: 0 },
        Position {
            v: valid_spots.0[0],
        },
    ));
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn randomly_reposition_player(
    mut player_query: Query<(&mut Transform, &mut Position), With<Player>>,
    valid_spots: Res<ValidSpots>,
) {
    if let Ok((mut transform, mut position)) = player_query.get_single_mut() {
        let pos = valid_spots.0[0];
        transform.translation.x = pos.x as f32 * TILE_SIZE;
        transform.translation.y = pos.y as f32 * TILE_SIZE;
        position.v = pos;
    }
}
