use crate::pieces::components::Occupier;
use crate::player::Player;
use crate::vectors::{line_of_sight, Vector2Int};

use super::components::{Position, Tile, VisionBlocker, Wall};
use super::dungeon::{room, tunneler, Area, Dungeon};
use super::{CurrentBoard, ValidSpots};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub const VISIBILITY_RANGE: i32 = 10;

pub fn spawn_map(mut commands: Commands, mut current: ResMut<CurrentBoard>) {
    let mut dungeon = Dungeon::new(2);
    for idx in 0..4 {
        let tun = match idx % 2 {
            0 => Box::new(tunneler::LShapeTunneler) as Box<dyn tunneler::Tunneler>,
            _ => Box::new(tunneler::RandomTunneler) as Box<dyn tunneler::Tunneler>,
        };
        let gen = Box::new(room::BubbleGenerator {
            room_count: (3, 5),
            room_size: (4, 8),
            room_padding: Some(2),
            extra_connection_chance: 0.25,
        }) as Box<dyn room::RoomGenerator>;
        dungeon.add_area(Area::new(tun, gen))
    }
    dungeon.generate();

    let valid_spots = dungeon.get_valid_spots();

    current.tiles = HashMap::new();
    for v in dungeon.to_tiles() {
        let tile = commands
            .spawn((
                Position { v },
                Tile {
                    visible: false,
                    seen: false,
                },
            ))
            .id();
        if dungeon.walls.contains(&v) {
            commands.entity(tile).with_children(|parent| {
                parent
                    .spawn(Occupier)
                    .insert(Wall)
                    .insert(VisionBlocker)
                    .insert(Position { v });
            });
        }
        current.tiles.insert(v, tile);
    }

    commands.insert_resource(ValidSpots(valid_spots));
}

pub fn update_tile_visibility(
    player_query: Query<&Position, With<Player>>,
    mut tile_query: Query<(&mut Tile, &Position), Without<Player>>,
    blocker_query: Query<&Position, With<VisionBlocker>>,
) {
    let Ok(player_position) = player_query.get_single() else {
        return;
    };

    let blocker_positions: HashSet<Vector2Int> = blocker_query.iter().map(|b| b.v).collect();

    let area = player_position.v.circle_area(VISIBILITY_RANGE);

    let visible_positions = line_of_sight(player_position.v, area, &blocker_positions);

    for (mut tile, position) in tile_query.iter_mut() {
        let within_range = position.v.distance(player_position.v) <= VISIBILITY_RANGE;

        let in_sight = visible_positions.contains(&position.v);

        if within_range && in_sight {
            tile.visible = true;
            tile.seen = true;
        } else {
            tile.visible = false;
        }
    }
}

pub fn despawn_map(
    mut commands: Commands,
    tile_query: Query<Entity, With<Tile>>,
    mut valid_spots: ResMut<ValidSpots>,
) {
    for entity in tile_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    valid_spots.0.clear();
}
