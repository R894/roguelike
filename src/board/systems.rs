use bevy::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::pieces::components::Occupier;
use crate::vectors::Vector2Int;

use super::components::{Position, Tile};
use super::CurrentBoard;

pub fn spawn_map(mut commands: Commands, mut current: ResMut<CurrentBoard>) {
    let file = File::open("assets/map.txt").expect("No map file found");
    current.tiles = HashMap::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let v = Vector2Int::new(x as i32, y as i32);
                let tile = commands.spawn((Position { v }, Tile)).id();

                if char == '#' {
                    commands.entity(tile).insert(Occupier);
                };
                if char == '~' {};
                current.tiles.insert(v, tile);
            }
        }
    }
}
