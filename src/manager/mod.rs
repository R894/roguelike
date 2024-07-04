use bevy::prelude::*;

use crate::actions::{
    ActionsCompleteEvent, GameOverEvent, InvalidPlayerActionEvent, NextLevelEvent, TickEvent,
};
use crate::board::systems::{despawn_map, spawn_map};
use crate::graphics::GraphicsWaitEvent;
use crate::input::PlayerInputReadyEvent;
use crate::pieces::{despawn_pieces, spawn_npcs};
use crate::player::randomly_reposition_player;
use crate::states::{GameState, LevelSetupSet, MainState, TurnSet};

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), game_start)
            .add_systems(OnExit(MainState::Game), game_end)
            .configure_sets(
                Update,
                (TurnSet::Logic, TurnSet::Animation, TurnSet::Tick)
                    .chain()
                    .run_if(in_state(GameState::TurnUpdate)),
            )
            .configure_sets(Update, LevelSetupSet.run_if(on_event::<NextLevelEvent>()))
            .add_systems(
                Update,
                turn_update_start.run_if(on_event::<PlayerInputReadyEvent>()),
            )
            .add_systems(Update, next_level.run_if(on_event::<NextLevelEvent>()))
            .add_systems(Update, game_over.run_if(on_event::<GameOverEvent>()))
            .add_systems(
                Update,
                turn_update_end.run_if(on_event::<ActionsCompleteEvent>()),
            )
            .add_systems(
                Update,
                turn_update_cancel.run_if(on_event::<InvalidPlayerActionEvent>()),
            )
            .add_systems(Update, tick.in_set(TurnSet::Tick))
            .add_systems(
                Update,
                (
                    despawn_map,
                    despawn_pieces,
                    spawn_map,
                    randomly_reposition_player,
                    spawn_npcs,
                )
                    .chain()
                    .in_set(LevelSetupSet),
            );
    }
}

fn game_start(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerInput);
}

fn game_end(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::None);
}

fn turn_update_start(
    mut next_state: ResMut<NextState<GameState>>,
    mut ev_tick: EventWriter<TickEvent>,
) {
    next_state.set(GameState::TurnUpdate);
    ev_tick.send(TickEvent);
}

fn next_level(mut ev_next_level: EventReader<NextLevelEvent>) {
    for _ in ev_next_level.read() {
        println!("Next Level event recieved");
    }
}

fn tick(mut ev_wait: EventReader<GraphicsWaitEvent>, mut ev_tick: EventWriter<TickEvent>) {
    if ev_wait.read().len() == 0 {
        ev_tick.send(TickEvent);
    }
}

fn game_over(mut next_state: ResMut<NextState<MainState>>) {
    next_state.set(MainState::GameOver);
}

fn turn_update_end(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerInput);
}

fn turn_update_cancel(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerInput);
}
