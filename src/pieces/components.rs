use bevy::prelude::*;

use crate::actions::{models::despawn_recursive, Action};

use super::equipment::Item;

#[derive(Component, Default)]
pub struct Actor(pub Vec<(Box<dyn Action>, i32)>);

#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

#[derive(Component)]
pub struct ItemContainer {
    pub item: Box<dyn Item>,
}

#[derive(Component)]
pub struct Gold {
    pub value: u32,
}

#[derive(Component, Clone)]
pub struct GoldDrop {
    pub value: u32,
}

impl Item for GoldDrop {
    fn pick_up(
        &self,
        world: &mut World,
        player_entity: Entity,
        item_entity: Entity,
    ) -> Result<(), ()> {
        let mut player_gold = world.get_mut::<Gold>(player_entity).ok_or(())?;
        player_gold.value += self.value;
        despawn_recursive(world, item_entity);
        Ok(())
    }
    fn name(&self) -> String {
        "Gold".to_string()
    }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }
}

#[derive(Component, Clone)]
pub struct HealthDrop {
    pub value: u32,
}

impl Item for HealthDrop {
    fn pick_up(
        &self,
        world: &mut World,
        player_entity: Entity,
        item_entity: Entity,
    ) -> Result<(), ()> {
        let mut player_health = world.get_mut::<Health>(player_entity).ok_or(())?;
        player_health.current = player_health.max.min(player_health.current + self.value);
        despawn_recursive(world, item_entity);
        Ok(())
    }

    fn name(&self) -> String {
        "Health".to_string()
    }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }
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

#[derive(Component)]
pub struct ItemPicker;

#[derive(Component)]
pub struct Portal;
