mod inventory;
pub mod systems;

use bevy::prelude::*;
use inventory::InventoryPlugin;

use crate::states::MainState;

pub struct MenuPlugin;

#[derive(Component)]
pub struct Menu;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InventoryPlugin)
            .add_systems(OnEnter(MainState::Menu), systems::main_menu)
            .add_systems(Update, systems::menu_button_system)
            .add_systems(OnExit(MainState::Menu), systems::despawn_menu)
            .add_systems(OnEnter(MainState::GameOver), systems::game_over_menu)
            .add_systems(OnExit(MainState::GameOver), systems::despawn_menu);
    }
}
