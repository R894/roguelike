use bevy::prelude::*;

use crate::pieces::components::{Health, Melee, Piece};

use super::Equipment;

pub fn update_piece_stats(
    mut piece_query: Query<
        (&mut Equipment, &mut Melee, &mut Health),
        (With<Piece>, Changed<Equipment>),
    >,
) {
    for (equipment, mut melee, mut health) in piece_query.iter_mut() {
        if let Some(weapon) = &equipment.weapon {
            melee.damage += weapon.damage;
        }

        if let Some(armor) = &equipment.armor {
            health.value += armor.armor;
        }
    }
}
