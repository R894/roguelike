pub mod systems;

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Equipment {
    pub weapon: Option<Item>,
    pub chest: Option<Item>,
}

#[derive(Component, Default)]
pub struct Item {
    pub damage: Option<Damage>,
    pub health: Option<u32>,
    pub defense: Option<u32>,
}

pub struct Damage {
    pub min: u32,
    pub max: u32,
}
