pub mod systems;

use bevy::prelude::*;

#[derive(Component)]
pub struct Equipment {
    pub armor: Option<Armor>,
    pub weapon: Option<Weapon>,
}

#[derive(Component)]
pub struct Armor {
    pub armor: u32,
}

#[derive(Component)]
pub struct Weapon {
    pub damage: u32,
}
