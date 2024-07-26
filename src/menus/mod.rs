mod inventory;
pub mod systems;

use bevy::prelude::*;
use inventory::InventoryPlugin;
use systems::start_game_event_system;

use crate::states::MainState;

pub struct MenuPlugin;

#[derive(Event)]
pub struct StartGameEvent;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InventoryPlugin)
            .add_event::<StartGameEvent>()
            .add_systems(OnEnter(MainState::Menu), systems::main_menu)
            .add_systems(
                Update,
                start_game_event_system.run_if(on_event::<StartGameEvent>()),
            )
            .add_systems(OnExit(MainState::Menu), systems::despawn_menu)
            .add_systems(OnEnter(MainState::GameOver), systems::game_over_menu)
            .add_systems(OnExit(MainState::GameOver), systems::despawn_menu);
    }
}
