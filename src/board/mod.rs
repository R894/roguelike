use bevy::prelude::*;
use std::collections::HashMap;

use crate::{states::MainState, vectors::Vector2Int};

pub mod components;
pub mod dungeon;
pub mod systems;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBoard>()
            .add_systems(OnEnter(MainState::Game), systems::spawn_map)
            .add_systems(Update, systems::update_tile_visibility)
            .add_systems(OnExit(MainState::Game), systems::despawn_map);
    }
}

#[derive(Default, Resource)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, Entity>,
}

#[derive(Resource)]
pub struct ValidSpots(pub Vec<Vector2Int>);
