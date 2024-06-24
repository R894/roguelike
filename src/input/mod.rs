use std::collections::VecDeque;

use bevy::prelude::*;

use crate::actions::models::{DigAction, WalkAction};
use crate::actions::{Action, ActorQueue};
use crate::board::components::Position;
use crate::pieces::components::Actor;
use crate::player::Player;
use crate::states::GameState;
use crate::vectors::Vector2Int;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputReadyEvent>().add_systems(
            Update,
            player_position.run_if(in_state(GameState::PlayerInput)),
        );
    }
}

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::KeyW, Vector2Int::UP),
    (KeyCode::KeyS, Vector2Int::DOWN),
    (KeyCode::KeyA, Vector2Int::LEFT),
    (KeyCode::KeyD, Vector2Int::RIGHT),
];

#[derive(Event)]
pub struct PlayerInputReadyEvent;

fn player_position(
    keys: ResMut<ButtonInput<KeyCode>>,
    mut player_query: Query<(Entity, &Position, &mut Actor), With<Player>>,
    mut queue: ResMut<ActorQueue>,
    mut ev_input: EventWriter<PlayerInputReadyEvent>,
) {
    let Ok((entity, position, mut actor)) = player_query.get_single_mut() else {
        return;
    };
    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) {
            continue;
        }
        let mut action: (Box<dyn Action>, i32) =
            (Box::new(WalkAction(entity, position.v + dir)), 0);
        if keys.pressed(KeyCode::KeyF) {
            action = (Box::new(DigAction(entity, position.v + dir)), 0);
        }
        actor.0 = vec![action];
        queue.0 = VecDeque::from([entity]);
        ev_input.send(PlayerInputReadyEvent);
    }
}
