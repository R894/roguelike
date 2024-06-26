mod systems;

use bevy::prelude::*;

use crate::states::MainState;

pub struct MenuPlugin;

#[derive(Component)]
pub struct Menu;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Menu), systems::spawn_menu)
            .add_systems(OnExit(MainState::Menu), systems::despawn_menu)
            .add_systems(OnEnter(MainState::GameOver), systems::game_over_menu)
            .add_systems(OnExit(MainState::GameOver), systems::despawn_menu);
    }
}
