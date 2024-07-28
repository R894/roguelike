use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    actions::{
        models::{MeleeHitAction, ProjectileFlyAction, WalkAction},
        ActionExecutedEvent,
    },
    board::components::Position,
    pieces::components::Piece,
};

use super::{
    assets::{Ascii, PathAnimator},
    PIECE_SPEED, PIECE_Z, POSITION_TOLERANCE, TILE_SIZE,
};

pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Piece), Added<Piece>>,
    assets: Res<Ascii>,
) {
    for (entity, position, piece) in query.iter() {
        let sprite_idx = match piece.kind.as_str() {
            "Player" => 25,
            "Coin" => 522,
            "Portal" => 297,
            "Health" => 529,
            _ => 269,
        };
        let sprite = Sprite {
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            color: Color::WHITE,
            ..default()
        };
        let v = super::get_world_position(position, PIECE_Z);
        commands
            .entity(entity)
            .insert(SpriteBundle {
                sprite,
                texture: assets.image.clone(),
                transform: Transform::from_translation(v),

                ..default()
            })
            .insert(TextureAtlas {
                index: sprite_idx,
                layout: assets.texture.clone(),
            });

        if sprite_idx == 1 {
            commands.entity(entity).with_children(|parent| {
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.2, 0.2, 0.2),
                        custom_size: Some(Vec2::splat(TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 0., -1.),
                    ..default()
                });
            });
        }
    }
}

pub fn path_animator_update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PathAnimator, &mut Transform)>,
    time: Res<Time>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    for (entity, mut animator, mut transform) in query.iter_mut() {
        if animator.path.is_empty() {
            // this entity has completed it's animation
            commands.entity(entity).remove::<PathAnimator>();
            continue;
        }

        let target = *animator.path.front().unwrap();
        if animator.instant {
            transform.translation = target;
            animator.path.clear();
            continue;
        }

        let d = (target - transform.translation).length();
        ev_wait.send(super::GraphicsWaitEvent);
        if d > POSITION_TOLERANCE {
            transform.translation = transform.translation.lerp(
                target,
                animator.speed_multiplier * PIECE_SPEED * time.delta_seconds(),
            );
        } else {
            // the entity is at the desired path position
            transform.translation = target;
            animator.path.pop_front();
        }
    }
}

pub fn walk_animation(mut commands: Commands, mut ev_action: EventReader<ActionExecutedEvent>) {
    for ev in ev_action.read() {
        let action = ev.0.as_any();
        if let Some(action) = action.downcast_ref::<WalkAction>() {
            let target = super::get_world_vec(action.1, PIECE_Z);
            commands.entity(action.0).insert(PathAnimator {
                path: VecDeque::from([target]),
                instant: true,
                ..default()
            });
        }

        if let Some(action) = action.downcast_ref::<ProjectileFlyAction>() {
            if let Some(v) = action.1.first() {
                let target = super::get_world_vec(*v, PIECE_Z);
                if let Some(mut entity) = commands.get_entity(action.0) {
                    entity.insert(PathAnimator {
                        path: VecDeque::from([target]),
                        instant: true,
                        ..default()
                    });
                }
            }
        }
    }
}

pub fn melee_animation(
    mut commands: Commands,
    query: Query<&Position>,
    mut ev_action: EventReader<ActionExecutedEvent>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    for ev in ev_action.read() {
        let action = ev.0.as_any();
        if let Some(action) = action.downcast_ref::<MeleeHitAction>() {
            let Ok(base_position) = query.get(action.attacker) else {
                continue;
            };
            let base = super::get_world_position(base_position, PIECE_Z);
            let target = base + 0.25 * (super::get_world_vec(action.target, PIECE_Z) - base);
            commands.entity(action.attacker).insert(PathAnimator {
                path: VecDeque::from([target, base]),
                speed_multiplier: 5.0,
                ..default()
            });
            ev_wait.send(super::GraphicsWaitEvent);
        }
    }
}
