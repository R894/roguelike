use std::sync::Arc;

use bevy::prelude::*;

use crate::pieces::components::{Health, Melee};

use super::{EquipItemEvent, Equipment, UnequipItemEvent};

pub fn equip_item_system(
    mut stats_query: Query<(&mut Equipment, &mut Health, &mut Melee)>,
    mut equip_event: EventReader<EquipItemEvent>,
) {
    for event in equip_event.read() {
        if let Ok((mut equipment, mut health, mut melee)) = stats_query.get_mut(event.entity) {
            match event.slot {
                super::EquipmentSlot::Weapon => {
                    let equippable = event.equippable.lock().unwrap();
                    health.max += equippable.health().unwrap_or(0);
                    melee.damage += equippable
                        .damage()
                        .unwrap_or(super::Damage { min: 0, max: 0 })
                        .max;
                    println!("Health: {}, Damage: {}", health.max, melee.damage);

                    equipment.weapon = Some(Arc::clone(&event.equippable));
                }
                super::EquipmentSlot::Chest => {
                    let equippable = event.equippable.lock().unwrap();
                    health.max += equippable.health().unwrap_or(0);
                    melee.damage += equippable
                        .damage()
                        .unwrap_or(super::Damage { min: 0, max: 0 })
                        .max;
                    println!("Health: {}, Damage: {}", health.max, melee.damage);
                    equipment.chest = Some(Arc::clone(&event.equippable));
                }
            }
        }
    }
}

pub fn unequip_item_system(
    mut equipment_query: Query<&mut Equipment>,
    mut stats_query: Query<(&mut Health, &mut Melee)>,
    mut unequip_event: EventReader<UnequipItemEvent>,
) {
    for event in unequip_event.read() {
        if let Ok(mut equipment) = equipment_query.get_mut(event.entity) {
            let (mut health, mut melee) = stats_query.get_mut(event.entity).unwrap();
            match event.slot {
                super::EquipmentSlot::Weapon => {
                    if let Some(weapon) = &mut equipment.weapon {
                        let mut weapon_ref = weapon.lock().unwrap();
                        health.max -= weapon_ref.health().unwrap_or(0);
                        melee.damage -= weapon_ref
                            .damage()
                            .unwrap_or(super::Damage { min: 0, max: 0 })
                            .max;
                        println!("Health: {}, Damage: {}", health.max, melee.damage);
                        weapon_ref.set_equipped(false);
                        drop(weapon_ref);
                        equipment.weapon = None;
                    }
                }
                super::EquipmentSlot::Chest => {
                    if let Some(armor) = &equipment.chest {
                        let mut armor_ref = armor.lock().unwrap();
                        melee.damage -= armor_ref
                            .damage()
                            .unwrap_or(super::Damage { min: 0, max: 0 })
                            .max;
                        let armor_health = armor_ref.health().unwrap_or(0);
                        println!("Subtracting {} from {}", armor_health, health.max);
                        health.max -= armor_health;
                        println!("Health: {}, Damage: {}", health.max, melee.damage);
                        armor_ref.set_equipped(false);
                        drop(armor_ref);
                        equipment.chest = None;
                    }
                }
            }
        }
    }
}
