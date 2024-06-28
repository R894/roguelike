use bevy::prelude::*;

use crate::{
    board::{components::Position, systems::spawn_map, ValidSpots},
    pieces::components::{Actor, Gold, Health, ItemPicker, Melee, Occupier, Piece},
    states::MainState,
    vectors::Vector2Int,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_player.after(spawn_map));
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, valid_spots: Res<ValidSpots>) {
    commands.spawn((
        Actor::default(),
        Player,
        Occupier,
        ItemPicker,
        Health { value: 10 },
        Melee { damage: 5 },
        Piece {
            kind: "Player".to_string(),
        },
        Gold { value: 0 },
        Position {
            v: valid_spots.0[0],
        },
    ));
}
