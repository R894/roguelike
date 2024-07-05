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

pub fn update_tile_colors(
    mut query: Query<(&mut Sprite, Option<&Children>, &Tile), Changed<Tile>>,
    mut sprite_query: Query<&mut Sprite, Without<Tile>>,
) {
    for (mut sprite, children, tile) in query.iter_mut() {
        let mut color = sprite.color;
        if tile.visible {
            color.set_a(1.0);
        } else if tile.seen {
            color.set_a(0.1);
        }
        sprite.color = color;

        if let Some(children) = children {
            update_children_colors(&mut sprite_query, children, color);
        }
    }
}

fn update_children_colors(
    query: &mut Query<&mut Sprite, Without<Tile>>,
    children: &Children,
    color: Color,
) {
    for child in children.iter() {
        if let Ok(mut sprite) = query.get_mut(*child) {
            sprite.color = color;
        }
    }
}
