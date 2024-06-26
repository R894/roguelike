use bevy::prelude::*;

use crate::{
    board::components::Position,
    pieces::components::{Actor, Gold, Health, ItemPicker, Melee, Occupier, Piece},
    states::MainState,
    vectors::Vector2Int,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
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
            v: Vector2Int::new(1, 1),
        },
    ));
}
