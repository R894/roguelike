pub mod systems;

use std::{
    default,
    sync::{Arc, Mutex},
};

use bevy::prelude::*;

use crate::{actions::models::despawn_recursive, player::inventory::Inventory};

use super::components::ItemContainer;

#[derive(Component, Clone, PartialEq, Eq)]
pub enum EquipmentSlot {
    Weapon,
    Chest,
}

#[derive(Event)]
pub struct EquipItemEvent {
    pub entity: Entity,
    pub slot: EquipmentSlot,
    pub id: u32,
}

#[derive(Event)]
pub struct UnequipItemEvent {
    pub entity: Entity,
    pub slot: EquipmentSlot,
}

#[derive(Component, Default)]
pub struct Equipment {
    pub weapon: Option<Box<dyn Item>>,
    pub chest: Option<Box<dyn Item>>,
}

pub trait Equippable: Send + Sync {
    fn name(&self) -> String;
    fn slot(&self) -> EquipmentSlot;
    fn damage(&self) -> Option<Damage>;
    fn health(&self) -> Option<u32>;
    fn defense(&self) -> Option<u32>;
    fn clone_box(&self) -> Box<dyn Equippable>;
}

impl Clone for Box<dyn Equippable> {
    fn clone(&self) -> Self {
        self.as_ref().clone_box()
    }
}

pub trait Item: Send + Sync {
    fn pick_up(
        &self,
        world: &mut World,
        player_entity: Entity,
        item_entity: Entity,
    ) -> Result<(), ()>;
    fn id(&self) -> u32;
    fn name(&self) -> String;
    fn clone_box(&self) -> Box<dyn Item>;
    fn as_equippable(&self) -> Option<&dyn Equippable>;
    fn as_mut_equippable(&mut self) -> Option<&mut dyn Equippable>;
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

#[derive(Component, Clone, Default)]
pub struct Sword {
    equipped: bool,
    id: u32,
}

impl Equippable for Sword {
    fn slot(&self) -> EquipmentSlot {
        EquipmentSlot::Weapon
    }
    fn name(&self) -> String {
        Item::name(self)
    }
    fn damage(&self) -> Option<Damage> {
        Some(Damage { min: 5, max: 10 })
    }
    fn health(&self) -> Option<u32> {
        None
    }
    fn defense(&self) -> Option<u32> {
        None
    }

    fn clone_box(&self) -> Box<dyn Equippable> {
        Box::new(self.clone())
    }
}

impl Item for Sword {
    fn pick_up(
        &self,
        world: &mut World,
        player_entity: Entity,
        item_entity: Entity,
    ) -> Result<(), ()> {
        let item = {
            let item_container_ref = world.get::<ItemContainer>(item_entity).ok_or(())?;
            item_container_ref.item.clone()
        };

        let mut inventory = world.get_mut::<Inventory>(player_entity).ok_or(())?;
        inventory.items.push(item);

        despawn_recursive(world, item_entity);
        Ok(())
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        "Sword".to_string()
    }

    fn as_equippable(&self) -> Option<&dyn Equippable> {
        Some(self)
    }

    fn as_mut_equippable(&mut self) -> Option<&mut dyn Equippable> {
        Some(self)
    }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }
}

#[derive(Component, Clone, Default)]
pub struct ChestArmor {
    equipped: bool,
    id: u32,
}

impl Equippable for ChestArmor {
    fn slot(&self) -> EquipmentSlot {
        EquipmentSlot::Chest
    }
    fn name(&self) -> String {
        Item::name(self)
    }
    fn damage(&self) -> Option<Damage> {
        None
    }
    fn health(&self) -> Option<u32> {
        Some(10)
    }
    fn defense(&self) -> Option<u32> {
        Some(5)
    }

    fn clone_box(&self) -> Box<dyn Equippable> {
        Box::new(self.clone())
    }
}

impl Item for ChestArmor {
    fn pick_up(
        &self,
        world: &mut World,
        player_entity: Entity,
        item_entity: Entity,
    ) -> Result<(), ()> {
        let item = {
            let item_container_ref = world.get::<ItemContainer>(item_entity).ok_or(())?;
            item_container_ref.item.clone()
        };

        let mut inventory = world.get_mut::<Inventory>(player_entity).ok_or(())?;
        inventory.items.push(item);

        despawn_recursive(world, item_entity);
        Ok(())
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        "Chest Armor".to_string()
    }

    fn as_equippable(&self) -> Option<&dyn Equippable> {
        Some(self)
    }

    fn as_mut_equippable(&mut self) -> Option<&mut dyn Equippable> {
        Some(self)
    }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }
}
