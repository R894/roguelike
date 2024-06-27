use crate::pieces::components::Occupier;

use super::components::{Position, Tile, Wall};
use super::dungeon::{tunneler, Area, Dungeon};
use super::CurrentBoard;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn spawn_map(mut commands: Commands, mut current: ResMut<CurrentBoard>) {
    let mut dungeon = Dungeon::new(2);
    for idx in 0..4 {
        let tun = match idx % 2 {
            0 => Box::new(tunneler::LShapeTunneler) as Box<dyn tunneler::Tunneler>,
            _ => Box::new(tunneler::RandomTunneler) as Box<dyn tunneler::Tunneler>,
        };
        dungeon.add_area(Area::new(tun))
    }
    dungeon.generate();

    current.tiles = HashMap::new();
    for v in dungeon.to_tiles() {
        let tile = commands.spawn((Position { v }, Tile)).id();
        if dungeon.walls.contains(&v) {
            commands.entity(tile).with_children(|parent| {
                parent.spawn(Occupier).insert(Wall).insert(Position { v });
            });
        }
        current.tiles.insert(v, tile);
    }
}

pub fn despawn_map(mut commands: Commands, tile_query: Query<Entity, With<Tile>>) {
    for entity in tile_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
