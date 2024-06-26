use bevy::prelude::*;
use components::Piece;

use crate::{board::components::Position, states::MainState, vectors::Vector2Int};
pub mod components;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_npcs)
            .add_systems(OnExit(MainState::Game), despawn_pieces);
    }
}

pub fn spawn_npcs(mut commands: Commands) {
    spawn_test_npc(&mut commands, Vector2Int::new(3, 5));
    spawn_test_npc(&mut commands, Vector2Int::new(5, 5));
    spawn_coin(&mut commands, Vector2Int::new(7, 5));
    spawn_coin(&mut commands, Vector2Int::new(9, 5));
    spawn_coin(&mut commands, Vector2Int::new(11, 5));
}

fn spawn_test_npc(commands: &mut Commands, v: Vector2Int) {
    commands.spawn((
        components::Actor::default(),
        components::Health { value: 10 },
        components::Piece {
            kind: "NPC".to_string(),
        },
        components::Melee { damage: 1 },
        components::Occupier,
        Position { v },
        components::Walk,
    ));
}

fn spawn_coin(commands: &mut Commands, v: Vector2Int) {
    commands.spawn((
        components::Gold { value: 1 },
        components::Piece {
            kind: "Coin".to_string(),
        },
        Position { v },
    ));
}

fn despawn_pieces(mut commands: Commands, query: Query<Entity, With<Piece>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
