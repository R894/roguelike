mod inventory;
pub mod systems;

use bevy::prelude::*;
use inventory::{equip_inventory_item, InventoryState};

use crate::states::MainState;

pub struct MenuPlugin;

#[derive(Component)]
pub struct Menu;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InventoryState>()
            .add_systems(OnEnter(MainState::Menu), systems::main_menu)
            .add_systems(
                OnEnter(InventoryState::Open),
                (
                    inventory::spawn_inventory_menu,
                    inventory::populate_inventory_items,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                inventory::inventory_input.run_if(in_state(MainState::Game)),
            )
            .add_systems(
                OnExit(InventoryState::Open),
                inventory::despawn_inventory_menu,
            )
            .add_systems(Update, systems::menu_button_system)
            .add_systems(Update, equip_inventory_item)
            .add_systems(OnExit(MainState::Menu), systems::despawn_menu)
            .add_systems(OnEnter(MainState::GameOver), systems::game_over_menu)
            .add_systems(OnExit(MainState::GameOver), systems::despawn_menu);
    }
}
