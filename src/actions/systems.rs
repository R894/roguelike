use bevy::prelude::*;
use rand::prelude::*;

use crate::actions::models::ProjectileShootAction;
use crate::board::systems::VISIBILITY_RANGE;
use crate::board::CurrentBoard;
use crate::pieces::components::{Melee, Occupier, Projectile, Walk};
use crate::player::Player;
use crate::vectors::{find_path, ORTHO_DIRECTIONS};
use crate::{board::components::Position, pieces::components::Actor};

use super::models::{MeleeHitAction, ProjectileFlyAction, WalkAction};
use super::{
    ActionExecutedEvent, ActionsCompleteEvent, ActorQueue, InvalidPlayerActionEvent,
    NextActorEvent, PendingActions,
};

const PLAYER_ATTACK_SCORE: i32 = 100;
const MOVE_SCORE: i32 = 50;

fn execute_action(action: Box<dyn super::Action>, world: &mut World) -> bool {
    if let Ok(result) = action.execute(world) {
        if let Some(mut pending) = world.get_resource_mut::<PendingActions>() {
            pending.0.extend(result);
        }
        world.send_event(ActionExecutedEvent(action));
        return true;
    }
    false
}

pub fn process_action_queue(world: &mut World) {
    if process_pending_actions(world) {
        return;
    }

    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else {
        return;
    };
    let Some(entity) = queue.0.pop_front() else {
        world.send_event(ActionsCompleteEvent);
        return;
    };
    let Some(mut actor) = world.get_mut::<Actor>(entity) else {
        world.send_event(NextActorEvent);
        return;
    };
    // clear the Actor vec
    let mut possible_actions = actor.0.drain(..).collect::<Vec<_>>();
    // highest score first
    possible_actions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut success = false;
    for action in possible_actions {
        success = success || execute_action(action.0, world);
        if success {
            break;
        }
    }
    if !success && world.get::<Player>(entity).is_some() {
        world.send_event(InvalidPlayerActionEvent);
        return;
    }
    world.send_event(NextActorEvent);
}

fn process_pending_actions(world: &mut World) -> bool {
    let pending = match world.get_resource_mut::<PendingActions>() {
        Some(mut res) => res.0.drain(..).collect::<Vec<_>>(),
        _ => return false,
    };
    let mut success = false;
    for action in pending {
        success = success || execute_action(action, world);
    }
    success
}

pub fn populate_actor_queue(
    query: Query<Entity, (With<Actor>, Without<Player>)>,
    mut queue: ResMut<ActorQueue>,
) {
    queue.0.extend(query.iter());
}

pub fn plan_walk(
    mut query: Query<(&Position, &mut Actor), With<Walk>>,
    queue: Res<ActorQueue>,
    player_query: Query<&Position, With<Player>>,
    occupier_query: Query<&Position, With<Occupier>>,
    board: Res<CurrentBoard>,
) {
    let Some(entity) = queue.0.front() else {
        return;
    };
    let Ok((position, mut actor)) = query.get_mut(*entity) else {
        return;
    };
    let Ok(player_position) = player_query.get_single() else {
        return;
    };
    if position.v.distance(player_position.v) > VISIBILITY_RANGE {
        return;
    }
    // get all possible move targets
    let positions = ORTHO_DIRECTIONS
        .iter()
        .map(|d| *d + position.v)
        .collect::<Vec<_>>();
    // find possible path to the player
    let path_to_player = find_path(
        position.v,
        player_position.v,
        &board.tiles.keys().cloned().collect(),
        &occupier_query.iter().map(|p| p.v).collect(),
    );
    let mut rng = thread_rng();
    let actions = positions
        .iter()
        .map(|v| {
            // randomize movement choices
            let mut d = rng.gen_range(-10..0);
            if let Some(path) = &path_to_player {
                // however prioritze a movement if it leads to the player
                if path.contains(v) {
                    d = 5
                }
            }
            (
                Box::new(WalkAction(*entity, *v)) as Box<dyn super::Action>,
                MOVE_SCORE + d,
            )
        })
        .collect::<Vec<_>>();
    actor.0.extend(actions);
}

pub fn plan_melee(
    mut query: Query<(&mut Actor, &Melee)>,
    player_query: Query<&Position, With<Player>>,
    queue: Res<ActorQueue>,
) {
    let Some(entity) = queue.0.front() else {
        return;
    };
    let Ok((mut actor, melee)) = query.get_mut(*entity) else {
        return;
    };
    let Ok(player_position) = player_query.get_single() else {
        return;
    };
    let action = Box::new(MeleeHitAction {
        attacker: *entity,
        target: player_position.v,
        damage: melee.current_damage.min,
    });
    actor.0.push((
        action,
        PLAYER_ATTACK_SCORE + melee.current_damage.min as i32,
    ))
}

pub fn process_projectiles(mut query: Query<(&mut Actor, Entity, &Projectile), Added<Projectile>>) {
    for (mut actor, entity, projectile) in query.iter_mut() {
        println!("Processing projectile {:?}", projectile.destination);
        actor.0.push((
            Box::new(ProjectileShootAction(entity, projectile.destination)),
            0,
        ));
    }
}
