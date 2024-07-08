pub mod systems;

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Equipment {
    pub weapon: Option<Equippable>,
    pub chest: Option<Equippable>,
}

#[derive(Component, Default)]
pub struct Equippable {
    pub damage: Option<Damage>,
    pub health: Option<u32>,
    pub defense: Option<u32>,
}

pub trait Item: Send + Sync {
    fn pick_up(
        &self,
        world: &mut World,
        player_entity: Entity,
        item_entity: Entity,
    ) -> Result<(), ()>;
    fn clone_box(&self) -> Box<dyn Item>;
}

impl Clone for Box<dyn Item> {
    fn clone(&self) -> Self {
        self.as_ref().clone_box()
    }
}

pub struct Damage {
    pub min: u32,
    pub max: u32,
}
