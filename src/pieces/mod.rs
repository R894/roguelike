use bevy::prelude::*;
use components::{Damage, Piece, Range};
use equipment::{
    systems::{equip_event_system, player_equip_event_system, unequip_event_system, update_stats},
    EquipItemEvent, PlayerEquipItemEvent, UnequipItemEvent,
};
use rand::Rng;

use crate::{
    board::{components::Position, systems::spawn_map, ValidSpots},
    player::{despawn_player, Player},
    states::MainState,
    vectors::Vector2Int,
};
pub mod components;
pub mod equipment;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EquipItemEvent>()
            .add_event::<UnequipItemEvent>()
            .add_event::<PlayerEquipItemEvent>()
            .add_systems(OnEnter(MainState::Game), spawn_npcs.after(spawn_map))
            .add_systems(OnExit(MainState::Game), (despawn_pieces, despawn_player))
            .add_systems(
                Update,
                equip_event_system.run_if(on_event::<EquipItemEvent>()),
            )
            .add_systems(
                Update,
                player_equip_event_system.run_if(on_event::<PlayerEquipItemEvent>()),
            )
            .add_systems(Update, update_stats)
            .add_systems(
                Update,
                unequip_event_system.run_if(on_event::<UnequipItemEvent>()),
            );
    }
}

pub fn spawn_npcs(mut commands: Commands, valid_spots: Res<ValidSpots>) {
    for _ in 0..10 {
        spawn_coin(&mut commands, &valid_spots);
        spawn_test_npc(&mut commands, &valid_spots);
        spawn_health_drop(&mut commands, &valid_spots)
    }
    spawn_portal(&mut commands, &valid_spots);
}

fn spawn_test_npc(commands: &mut Commands, valid_spots: &Res<ValidSpots>) {
    let rand = rand::thread_rng().gen_range(0..valid_spots.0.len());
    commands.spawn((
        components::Actor::default(),
        components::Health {
            base: Range { min: 10, max: 10 },
            current: Range { min: 10, max: 10 },
        },
        components::Piece {
            kind: "NPC".to_string(),
        },
        components::Melee {
            base_damage: Damage { min: 1, max: 1 },
            current_damage: Damage { min: 1, max: 1 },
        },
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
        components::ItemContainer {
            item: Box::new(components::GoldDrop { value: 1 }),
        },
        components::Piece {
            kind: "Coin".to_string(),
        },
        Position {
            v: valid_spots.0[rand],
        },
    ));
}

fn spawn_health_drop(commands: &mut Commands, valid_spots: &Res<ValidSpots>) {
    let rand = rand::thread_rng().gen_range(0..valid_spots.0.len());
    commands.spawn((
        components::ItemContainer {
            item: Box::new(components::HealthDrop { value: 5 }),
        },
        components::Piece {
            kind: "Health".to_string(),
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

pub fn spawn_projectile(
    commands: &mut Commands,
    source: Vector2Int,
    destination: Vector2Int,
) -> Entity {
    commands
        .spawn((
            components::Actor::default(),
            components::Projectile {
                destination,
                damage: Damage { min: 5, max: 5 },
            },
            components::Piece {
                kind: "Coin".to_string(),
            },
            Position { v: source },
        ))
        .id()
}

pub fn despawn_pieces(
    mut commands: Commands,
    query: Query<Entity, (With<Piece>, Without<Player>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
