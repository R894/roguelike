use bevy::prelude::*;

use crate::{
    pieces::components::{Health, Melee},
    player::inventory::Inventory,
};

use super::{EquipItemEvent, Equipment, EquipmentSlot, Item, UnequipItemEvent};

pub fn equip_event_system(
    mut stats_query: Query<(&mut Equipment, &mut Inventory)>,
    mut equip_event: EventReader<EquipItemEvent>,
) {
    for event in equip_event.read() {
        if let Ok((mut equipment, mut inventory)) = stats_query.get_mut(event.entity) {
            let mut item_to_equip: Option<Box<dyn Item>> = None;
            inventory.items.retain(|item| {
                if item.id() == event.id
                    && item.as_equippable().is_some()
                    && event.slot == item.as_equippable().unwrap().slot()
                {
                    item_to_equip = Some(item.clone());
                    false
                } else {
                    true
                }
            });

            if let Some(item) = item_to_equip {
                equip_item(&mut equipment, item, event.slot.clone());
            }
        }
    }
}

fn equip_item(equipment: &mut Equipment, item: Box<dyn Item>, slot: EquipmentSlot) {
    if item.as_equippable().is_some() {
        match slot {
            EquipmentSlot::Weapon => {
                equipment.weapon = Some(item);
            }
            EquipmentSlot::Chest => {
                equipment.chest = Some(item);
            }
        }
    }
}

pub fn unequip_and_return_item(
    equipment: &mut Equipment,
    slot: EquipmentSlot,
) -> Option<Box<dyn Item>> {
    match slot {
        EquipmentSlot::Weapon => equipment.weapon.take(),
        EquipmentSlot::Chest => equipment.chest.take(),
    }
}

pub fn unequip_event_system(
    mut stats_query: Query<(&mut Equipment, &mut Inventory)>,
    mut unequip_event: EventReader<UnequipItemEvent>,
) {
    for event in unequip_event.read() {
        if let Ok((mut equipment, mut inventory)) = stats_query.get_mut(event.entity) {
            if let Some(item) = unequip_and_return_item(&mut equipment, event.slot.clone()) {
                inventory.items.push(item);
            }
        }
    }
}
