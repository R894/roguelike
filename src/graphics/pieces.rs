use bevy::prelude::*;

use crate::{board::components::Position, pieces::components::Piece};

use super::{assets::Ascii, PIECE_SPEED, PIECE_Z, POSITION_TOLERANCE, TILE_SIZE};

pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Piece), Added<Piece>>,
    assets: Res<Ascii>,
) {
    for (entity, position, piece) in query.iter() {
        let sprite_idx = match piece.kind.as_str() {
            "Player" => 1,
            _ => 63,
        };
        let sprite = Sprite {
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            color: Color::RED,
            ..default()
        };
        let v = super::get_world_position(position, PIECE_Z);
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture: assets.image.clone(),
            transform: Transform::from_translation(v),
            atlas: TextureAtlas {
                index: sprite_idx,
                layout: assets.texture.clone(),
            },
            ..Default::default()
        });
    }
}

pub fn update_piece_position(
    mut query: Query<(&Position, &mut Transform), With<Piece>>,
    time: Res<Time>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    let mut animating = false;
    for (position, mut transform) in query.iter_mut() {
        let target = super::get_world_position(position, PIECE_Z);
        let d = (target - transform.translation).length();
        if d > POSITION_TOLERANCE {
            transform.translation = transform
                .translation
                .lerp(target, PIECE_SPEED * time.delta_seconds());
            animating = true;
        } else {
            transform.translation = target;
        }
    }
    if animating {
        ev_wait.send(super::GraphicsWaitEvent);
    }
}
