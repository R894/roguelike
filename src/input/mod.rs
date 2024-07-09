use std::collections::VecDeque;

use bevy::prelude::*;

use crate::actions::models::{DigAction, MeleeHitAction, WalkAction};
use crate::actions::{Action, ActorQueue};
use crate::board::components::Position;
use crate::graphics::assets::Ascii;
use crate::graphics::TILE_SIZE;
use crate::pieces::components::{Actor, Melee};
use crate::player::Player;
use crate::states::GameState;
use crate::vectors::Vector2Int;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputReadyEvent>()
            .init_state::<ActionDirectionSelectionState>()
            .add_systems(
                Update,
                player_position.run_if(in_state(GameState::PlayerInput)),
            )
            .add_systems(
                OnEnter(ActionDirectionSelectionState::Pending),
                display_action_arrows,
            )
            .add_systems(
                OnExit(ActionDirectionSelectionState::Pending),
                clear_action_arrows,
            );
    }
}

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::KeyW, Vector2Int::UP),
    (KeyCode::KeyS, Vector2Int::DOWN),
    (KeyCode::KeyA, Vector2Int::LEFT),
    (KeyCode::KeyD, Vector2Int::RIGHT),
];

const UP_INDEX: usize = 1052;
const LEFT_INDEX: usize = 1055;
const RIGHT_INDEX: usize = 1053;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum ActionDirectionSelectionState {
    #[default]
    None,
    Pending,
}

#[derive(Event)]
pub struct PlayerInputReadyEvent;

#[derive(Component)]
pub struct Arrows;

fn player_position(
    keys: ResMut<ButtonInput<KeyCode>>,
    mut player_query: Query<(Entity, &Position, &Melee, &mut Actor), With<Player>>,
    mut queue: ResMut<ActorQueue>,
    mut next_state: ResMut<NextState<ActionDirectionSelectionState>>,
    state: Res<State<ActionDirectionSelectionState>>,
    mut ev_input: EventWriter<PlayerInputReadyEvent>,
) {
    let Ok((entity, position, melee, mut actor)) = player_query.get_single_mut() else {
        return;
    };

    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(ActionDirectionSelectionState::None);
        return;
    }

    if keys.just_pressed(KeyCode::KeyF) {
        next_state.set(ActionDirectionSelectionState::Pending);
        return;
    }

    if state.get() == &ActionDirectionSelectionState::Pending {
        for (key, dir) in DIR_KEY_MAPPING {
            if !keys.just_pressed(key) {
                continue;
            }

            let dig_action: (Box<dyn Action>, i32) =
                (Box::new(DigAction(entity, position.v + dir)), 0);
            actor.0 = vec![dig_action];
            queue.0 = VecDeque::from([entity]);
            ev_input.send(PlayerInputReadyEvent);
            next_state.set(ActionDirectionSelectionState::None);
            return;
        }
    }

    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) {
            continue;
        }
        if *state.get() != ActionDirectionSelectionState::None {
            continue;
        }

        let move_action: (Box<dyn Action>, i32) =
            (Box::new(WalkAction(entity, position.v + dir)), 0);

        let melee_action: (Box<dyn Action>, i32) = (
            Box::new(MeleeHitAction {
                attacker: entity,
                target: position.v + dir,
                damage: melee.damage,
            }),
            0,
        );

        actor.0 = vec![move_action, melee_action];
        queue.0 = VecDeque::from([entity]);
        ev_input.send(PlayerInputReadyEvent);
    }
}

pub fn display_action_arrows(
    mut comands: Commands,
    player_query: Query<Entity, With<Player>>,
    assets: Res<Ascii>,
) {
    if let Ok(entity) = player_query.get_single() {
        spawn_arrows(&mut comands, entity, assets);
    }
}

pub fn spawn_arrows(commands: &mut Commands, entity: Entity, assets: Res<Ascii>) {
    let base_sprite = Sprite {
        color: Color::srgba(1., 1., 1., 0.5),
        custom_size: Some(Vec2::splat(TILE_SIZE)),
        ..default()
    };

    let arrow_up = spawn_arrow(
        commands,
        Vec3::new(0., 1. * TILE_SIZE, 50.),
        base_sprite.clone(),
        &assets,
        UP_INDEX,
    );

    let mut down_sprite = base_sprite.clone();
    down_sprite.flip_y = true;
    let arrow_down = spawn_arrow(
        commands,
        Vec3::new(0.0, -TILE_SIZE, 50.0),
        down_sprite,
        &assets,
        UP_INDEX,
    );

    let arrow_left = spawn_arrow(
        commands,
        Vec3::new(-TILE_SIZE, 0.0, 50.0),
        base_sprite.clone(),
        &assets,
        LEFT_INDEX,
    );

    let arrow_right = spawn_arrow(
        commands,
        Vec3::new(TILE_SIZE, 0.0, 50.0),
        base_sprite,
        &assets,
        RIGHT_INDEX,
    );
    commands
        .entity(entity)
        .push_children(&[arrow_up, arrow_down, arrow_left, arrow_right]);
}

fn spawn_arrow(
    commands: &mut Commands,
    position: Vec3,
    sprite: Sprite,
    assets: &Res<Ascii>,
    index: usize,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite,
            texture: assets.image.clone(),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .insert(TextureAtlas {
            index,
            layout: assets.texture.clone(),
        })
        .insert(Arrows)
        .id()
}

pub fn clear_action_arrows(mut commands: Commands, arrow_query: Query<Entity, With<Arrows>>) {
    for entity in arrow_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
