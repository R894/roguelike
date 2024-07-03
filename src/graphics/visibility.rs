use bevy::prelude::*;

use crate::{
    board::components::{Position, Tile},
    pieces::components::Piece,
};

pub fn update_visibility(
    mut query: Query<(&mut Visibility, &Position, &Tile), Changed<Tile>>,
    mut piece_query: Query<(&Position, &mut Visibility), (With<Piece>, Without<Tile>)>,
) {
    for (mut tile_visibility, tile_pos, tile) in query.iter_mut() {
        if tile.visible || tile.seen {
            *tile_visibility = Visibility::Visible;
        } else {
            *tile_visibility = Visibility::Hidden;
        }

        // update pieces too
        for (piece_pos, mut piece_visibility) in piece_query.iter_mut() {
            if piece_pos.v == tile_pos.v {
                if tile.visible {
                    *piece_visibility = Visibility::Visible;
                } else {
                    *piece_visibility = Visibility::Hidden;
                }
            }
        }
    }
}

pub fn update_tile_colors(mut query: Query<(&mut Sprite, &Tile), Changed<Tile>>) {
    for (mut sprite, tile) in query.iter_mut() {
        let mut color = sprite.color;
        if tile.visible {
            color.set_a(1.0);
        } else if tile.seen {
            color.set_a(0.1);
        }
        sprite.color = color;
    }
}
