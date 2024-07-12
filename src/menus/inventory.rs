use bevy::prelude::*;

use crate::{
    pieces::equipment::{EquipItemEvent, Equipment, Equippable, Sword},
    player::{inventory::Inventory, Player},
    ui::{OriginalColors, TextBox, UiFont},
};

const INVENTORY_BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
const INVENTORY_BORDER_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Component)]
pub struct InventoryItemContainer;

#[derive(Component)]
pub struct InventoryMenu;

#[derive(Component)]
// Holds the index of the item in the inventory
pub struct InventoryItemRef {
    pub index: usize,
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum InventoryState {
    #[default]
    Closed,
    Open,
}

pub fn spawn_inventory_menu(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(InventoryMenu)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(80.),
                        height: Val::Percent(80.),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    background_color: INVENTORY_BACKGROUND_COLOR.into(),
                    border_color: INVENTORY_BORDER_COLOR.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(10.0)),
                                width: Val::Percent(100.),
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Inventory",
                                TextStyle {
                                    font: font.0.clone(),
                                    font_size: 28.0,
                                    color: Color::srgb(0.7, 0.7, 0.7),
                                },
                            ));
                        });
                })
                .insert(InventoryItemContainer);
        });
}

pub fn despawn_inventory_menu(mut commands: Commands, query: Query<Entity, With<InventoryMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn inventory_input(
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

pub fn populate_inventory_items(
    mut commands: Commands,
    player_inventory_query: Query<&Inventory, With<Player>>,
    mut inventory_ui_query: Query<Entity, With<InventoryItemContainer>>,
    font: Res<UiFont>,
) {
    if let Ok(player_inventory) = player_inventory_query.get_single() {
        if let Ok(inventory_ui) = inventory_ui_query.get_single_mut() {
            for (index, item) in player_inventory.items.iter().enumerate() {
                commands.entity(inventory_ui).with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            background_color: Color::NONE.into(),
                            ..default()
                        })
                        .insert(OriginalColors {
                            ..Default::default()
                        })
                        .insert(InventoryItemRef { index })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                item.name(),
                                TextStyle {
                                    font: font.0.clone(),
                                    font_size: 20.0,
                                    color: Color::srgb(0.7, 0.7, 0.7),
                                },
                            ));
                        });
                });
            }
        }
    }
}

pub fn equip_inventory_item(
    mut interaction_query: Query<
        (&Interaction, &InventoryItemRef),
        (Changed<Interaction>, Without<TextBox>),
    >,
    player_inventory_query: Query<&Inventory, With<Player>>,
    mut event: EventWriter<EquipItemEvent>,
) {
    if let Ok(player_inventory) = player_inventory_query.get_single() {
        for (interaction, item_ref) in &interaction_query {
            if *interaction == Interaction::Pressed {
                let item = &player_inventory.items[item_ref.index];
                if let Some(equippable) = item.as_equippable() {
                    println!("Equipping {}", item.name());
                    event.send(EquipItemEvent {
                        equippable: equippable.clone_box(),
                        slot: equippable.slot(),
                    });
                } else {
                    println!("Item {} does not implement Equippable", item.name());
                }
            }
        }
    }
}
