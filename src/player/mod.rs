use bevy::prelude::*;

use crate::{
    board::components::Position,
    pieces::components::{Actor, Piece},
    vectors::Vector2Int,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Actor::default(),
        Player,
        Piece {
            kind: "Player".to_string(),
        },
        Position {
            v: Vector2Int::new(0, 0),
        },
    ));
}
