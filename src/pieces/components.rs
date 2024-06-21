use bevy::prelude::*;

use crate::actions::Action;

#[derive(Component, Default)]
pub struct Actor(pub Vec<(Box<dyn Action>, i32)>);

#[derive(Component)]
pub struct Health {
    pub value: u32,
}

#[derive(Component)]
pub struct Melee {
    pub damage: u32,
}

#[derive(Component)]
pub struct Occupier;

#[derive(Component)]
pub struct Piece {
    pub kind: String,
}

#[derive(Component)]
pub struct Walk;
