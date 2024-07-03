use bevy::prelude::*;
use components::Piece;
use rand::Rng;

use crate::{
    board::{components::Position, systems::spawn_map, ValidSpots},
    states::MainState,
};
pub mod components;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_npcs.after(spawn_map))
            .add_systems(OnExit(MainState::Game), despawn_pieces);
    }
}

pub fn spawn_npcs(mut commands: Commands, valid_spots: Res<ValidSpots>) {
    for _ in 0..10 {
        spawn_coin(&mut commands, &valid_spots);
        // spawn_test_npc(&mut commands, &valid_spots);
    }

    spawn_portal(&mut commands, &valid_spots);
}

fn spawn_test_npc(commands: &mut Commands, valid_spots: &Res<ValidSpots>) {
    let rand = rand::thread_rng().gen_range(0..valid_spots.0.len());
    commands.spawn((
        components::Actor::default(),
        components::Health { value: 10 },
        components::Piece {
            kind: "NPC".to_string(),
        },
        components::Melee { damage: 1 },
        components::Occupier,
        Position {
            v: valid_spots.0[rand],
        },
        components::Walk,
    ));
}

fn spawn_coin(commands: &mut Commands, valid_spots: &Res<ValidSpots>) {
    let rand = rand::thread_rng().gen_range(0..valid_spots.0.len());
    commands.spawn((
        components::Gold { value: 1 },
        components::Piece {
            kind: "Coin".to_string(),
        },
        Position {
            v: valid_spots.0[rand],
        },
    ));
}

fn spawn_portal(commands: &mut Commands, valid_spots: &Res<ValidSpots>) {
    let rand = rand::thread_rng().gen_range(0..valid_spots.0.len());
    commands.spawn((
        components::Piece {
            kind: "Portal".to_string(),
        },
        components::Portal,
        Position {
            v: valid_spots.0[rand],
        },
    ));
}

fn despawn_pieces(mut commands: Commands, query: Query<Entity, With<Piece>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
