use crate::pieces::components::Occupier;
use crate::player::Player;

use super::components::{Position, Tile, Wall};
use super::dungeon::{room, tunneler, Area, Dungeon};
use super::{CurrentBoard, ValidSpots};
use bevy::prelude::*;
use std::collections::HashMap;

const VISIBILITY_RANGE: i32 = 10;

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

    commands.insert_resource(ValidSpots(valid_spots));

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
                parent.spawn(Occupier).insert(Wall).insert(Position { v });
            });
        }
        current.tiles.insert(v, tile);
    }
}

pub fn update_tile_visibility(
    player_query: Query<&Position, (With<Player>, Changed<Position>)>,
    mut tile_query: Query<(&mut Tile, &Position), Without<Player>>,
) {
    let Ok(player_position) = player_query.get_single() else {
        return;
    };

    for (mut tile, position) in tile_query.iter_mut() {
        tile.visible = position.v.distance(player_position.v) <= VISIBILITY_RANGE;
    }
}

pub fn despawn_map(mut commands: Commands, tile_query: Query<Entity, With<Tile>>) {
    for entity in tile_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
