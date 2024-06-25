use std::collections::HashMap;

use crate::vectors::Vector2Int;
use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub v: Vector2Int,
}

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Wall;

#[derive(Default, Resource)]
pub struct BoardRes {
    pub tiles: HashMap<Vector2Int, Entity>,
}

#[derive(Component)]
pub struct Item;
