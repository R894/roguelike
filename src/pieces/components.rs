use bevy::prelude::*;

use crate::{
    actions::{models::despawn_recursive, Action},
    vectors::Vector2Int,
};

use super::equipment::{Equippable, Item};

#[derive(Component, Default)]
pub struct Actor(pub Vec<(Box<dyn Action>, i32)>);

#[derive(Component)]
pub struct Health {
    pub base: Range,
    pub current: Range,
}

#[derive(Clone, Copy)]
pub struct Damage {
    pub min: u32,
    pub max: u32,
}

#[derive(Clone, Copy)]
pub struct Range {
    pub min: u32,
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

#[derive(Component)]
pub struct Projectile {
    pub destination: Vector2Int,
    pub damage: Damage,
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

    fn id(&self) -> u32 {
        0
    }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn as_mut_equippable(&mut self) -> Option<&mut dyn Equippable> {
        None
    }

    fn as_equippable(&self) -> Option<&dyn Equippable> {
        None
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
        player_health.current.min = player_health
            .current
            .max
            .min(player_health.current.min + self.value);
        despawn_recursive(world, item_entity);
        Ok(())
    }

    fn name(&self) -> String {
        "Health".to_string()
    }

    fn id(&self) -> u32 {
        0
    }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn as_mut_equippable(&mut self) -> Option<&mut dyn Equippable> {
        None
    }

    fn as_equippable(&self) -> Option<&dyn Equippable> {
        None
    }
}

#[derive(Component)]
pub struct Melee {
    pub base_damage: Damage,
    pub current_damage: Damage,
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
