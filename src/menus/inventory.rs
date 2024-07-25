use belly::prelude::*;
use bevy::prelude::*;

use crate::{
    pieces::equipment::{Equipment, EquipmentSlot, PlayerEquipItemEvent, UnequipItemEvent},
    player::{inventory::Inventory, Player},
    states::MainState,
};

const INVENTORY_BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
enum InventoryState {
    #[default]
    Closed,
    Open,
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InventoryState>()
            .add_systems(Update, inventory_input.run_if(in_state(MainState::Game)))
            .add_systems(
                OnEnter(InventoryState::Open),
                (
                    spawn_inventory_menu,
                    init_inventory_equipment,
                    init_inventory_items,
                )
                    .chain(),
            )
            .add_systems(OnExit(InventoryState::Open), despawn_inventory_menu)
            .add_systems(
                Update,
                (populate_inventory_equipment, update_inventory_items)
                    .run_if(in_state(InventoryState::Open)),
            );
    }
}

fn spawn_inventory_menu(mut commands: Commands) {
    commands.add(eml! {
        <body id="inventory" s:padding="5%" s:width="100%" s:height="100%">
            <div id="inventory-menu" s:background-color=INVENTORY_BACKGROUND_COLOR>
                <div s:flex-direction="column" s:height="100%">
                    "Inventory"
                    <div id="items"/>
                </div>
                <div s:flex-direction="column" s:height="100%">
                    "Equipment"
                    <div id="equipment"/>
                </div>
            </div>
        </body>
    });
}

fn despawn_inventory_menu(mut elements: Elements) {
    elements.select("#inventory").remove();
}

fn inventory_input(
    mut next_state: ResMut<NextState<InventoryState>>,
    keys: ResMut<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(InventoryState::Closed);
    }

    if keys.just_pressed(KeyCode::KeyI) {
        next_state.set(InventoryState::Open);
    }
}

fn init_inventory_items(
    player_inventory_query: Query<&Inventory, (With<Player>, With<Inventory>)>,
    mut elements: Elements,
) {
    if let Ok(player_inventory) = player_inventory_query.get_single() {
        let mut inv = elements.select("#items");
        for item in player_inventory.items.iter() {
            let name = item.name().clone();
            let item_id = item.id();
            if let Some(slot) = item.as_equippable().map(|eq| eq.slot()) {
                inv.add_child(eml! {
                    <button on:press=move |ctx| {ctx.send_event(PlayerEquipItemEvent { slot: slot.clone(), id: item_id })}><div>{name}</div></button>
                });
            }
        }
    }
}

fn update_inventory_items(
    player_inventory_query: Query<&Inventory, (With<Player>, Changed<Inventory>)>,
    mut elements: Elements,
) {
    if let Ok(player_inventory) = player_inventory_query.get_single() {
        elements.select("#items > button").remove();
        let mut inv = elements.select("#items");
        for item in player_inventory.items.iter() {
            let name = item.name().clone();
            let item_id = item.id();
            if let Some(slot) = item.as_equippable().map(|eq| eq.slot()) {
                inv.add_child(eml! {
                    <button on:press=move |ctx| {ctx.send_event(PlayerEquipItemEvent { slot: slot.clone(), id: item_id })}><div>{name}</div></button>
                });
            }
        }
    }
}

fn init_inventory_equipment(
    player_equipment_query: Query<(Entity, &Equipment), With<Player>>,
    mut elements: Elements,
) {
    if let Ok((player_entity, player_equipment)) = player_equipment_query.get_single() {
        // clear old equipment
        elements.select("#equipment > button").remove();
        let mut equipment = elements.select("#equipment");

        let mut weapon_name = "None".to_string();
        let mut chest_name = "None".to_string();
        if let Some(weapon) = &player_equipment.weapon {
            weapon_name = weapon.name();
        }

        if let Some(chest) = &player_equipment.chest {
            chest_name = chest.name();
        }

        equipment.add_child(eml! {
                <button on:press=move |ctx| {ctx.send_event(UnequipItemEvent { slot: EquipmentSlot::Weapon, entity: player_entity })}><div>{weapon_name}</div></button>
            });

        equipment.add_child(eml! {
                <button on:press=move |ctx| {ctx.send_event(UnequipItemEvent { slot: EquipmentSlot::Chest, entity: player_entity })}><div>{chest_name}</div></button>
            });
    }
}

fn populate_inventory_equipment(
    player_equipment_query: Query<(Entity, &Equipment), (With<Player>, Changed<Equipment>)>,
    mut elements: Elements,
) {
    if let Ok((player_entity, player_equipment)) = player_equipment_query.get_single() {
        // clear old equipment
        elements.select("#equipment > button").remove();
        let mut equipment = elements.select("#equipment");

        let mut weapon_name = "None".to_string();
        let mut chest_name = "None".to_string();
        if let Some(weapon) = &player_equipment.weapon {
            weapon_name = weapon.name();
        }

        if let Some(chest) = &player_equipment.chest {
            chest_name = chest.name();
        }

        equipment.add_child(eml! {
                <button on:press=move |ctx| {ctx.send_event(UnequipItemEvent { slot: EquipmentSlot::Weapon, entity: player_entity })}><div>{weapon_name}</div></button>
            });

        equipment.add_child(eml! {
                <button on:press=move |ctx| {ctx.send_event(UnequipItemEvent { slot: EquipmentSlot::Chest, entity: player_entity })}><div>{chest_name}</div></button>
            });
    }
}
