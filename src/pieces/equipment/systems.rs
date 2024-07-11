use bevy::prelude::*;

use crate::{
    pieces::components::{Health, Melee, Piece},
    player::Player,
};

use super::{EquipItemEvent, Equipment};

#[allow(clippy::type_complexity)]
pub fn update_piece_stats(
    mut piece_query: Query<
        (&mut Equipment, &mut Melee, &mut Health),
        (With<Piece>, Changed<Equipment>),
    >,
) {
    for (equipment, mut melee, mut piece_health) in piece_query.iter_mut() {
        if let Some(weapon) = &equipment.weapon {
            if let Some(damage) = &weapon.damage() {
                melee.damage += damage.max;
            }
        }

        if let Some(armor) = &equipment.chest {
            if let Some(health) = armor.health() {
                piece_health.max += health;
            }
        }
    }
}

pub fn equip_item_system(
    mut equipment_query: Query<&mut Equipment, With<Player>>,
    mut pickup_event: EventReader<EquipItemEvent>,
) {
    if let Ok(mut equipment) = equipment_query.get_single_mut() {
        for event in pickup_event.read() {
            match event.slot {
                super::EquipmentSlot::Weapon => equipment.weapon = Some(event.equippable.clone()),
                super::EquipmentSlot::Chest => equipment.chest = Some(event.equippable.clone()),
            }
        }
    }
}
