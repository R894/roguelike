use bevy::prelude::*;

use crate::pieces::components::{Health, Melee, Piece};

use super::Equipment;

pub fn update_piece_stats(
    mut piece_query: Query<
        (&mut Equipment, &mut Melee, &mut Health),
        (With<Piece>, Changed<Equipment>),
    >,
) {
    for (equipment, mut melee, mut piece_health) in piece_query.iter_mut() {
        if let Some(weapon) = &equipment.weapon {
            if let Some(damage) = &weapon.damage {
                melee.damage += damage.max;
            }
        }

        if let Some(armor) = &equipment.chest {
            if let Some(health) = armor.health {
                piece_health.max += health;
            }
        }
    }
}
