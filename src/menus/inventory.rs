use bevy::prelude::*;

use crate::{
    pieces::equipment::{EquipItemEvent, Equipment},
    player::{inventory::Inventory, Player},
    states::MainState,
    ui::{OriginalColors, TextBox, UiFont},
};

const INVENTORY_BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
const INVENTORY_BORDER_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Component)]
struct InventoryItemContainer;

#[derive(Component)]
struct InventoryEquipmentContainer;

#[derive(Component)]
struct InventoryMenu;

#[derive(Component)]
enum EquipmentButtonSlot {
    Weapon,
    Chest,
}

#[derive(Component)]
// Holds the index of the item in the inventory
pub struct InventoryItemRef {
    pub index: usize,
}

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
                    populate_inventory_items,
                    init_inventory_equipment,
                )
                    .chain(),
            )
            .add_systems(OnExit(InventoryState::Open), despawn_inventory_menu)
            .add_systems(
                Update,
                populate_inventory_equipment.run_if(in_state(InventoryState::Open)),
            )
            .add_systems(
                Update,
                (equip_inventory_item, unequip_item_system).run_if(in_state(InventoryState::Open)),
            );
    }
}

fn spawn_inventory_menu(mut commands: Commands, font: Res<UiFont>) {
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
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(InventoryItemContainer);
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                row_gap: Val::Px(5.0),
                                column_gap: Val::Px(5.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(InventoryEquipmentContainer);
                });
        });
}

fn despawn_inventory_menu(mut commands: Commands, query: Query<Entity, With<InventoryMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
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

fn populate_inventory_items(
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

fn init_inventory_equipment(
    mut commands: Commands,
    mut inventory_equipment_query: Query<Entity, With<InventoryEquipmentContainer>>,
    font: Res<UiFont>,
) {
    if let Ok(inventory_equipment) = inventory_equipment_query.get_single_mut() {
        add_inventory_button(
            &mut commands,
            inventory_equipment,
            "Weapon: None",
            font.0.clone(),
        );
        add_inventory_button(
            &mut commands,
            inventory_equipment,
            "Chest: None",
            font.0.clone(),
        );
    }
}

fn populate_inventory_equipment(
    mut commands: Commands,
    player_equipment_query: Query<&Equipment, (With<Player>, Changed<Equipment>)>,
    mut inventory_equipment_query: Query<Entity, With<InventoryEquipmentContainer>>,
    font: Res<UiFont>,
) {
    if let Ok(player_equipment) = player_equipment_query.get_single() {
        if let Ok(inventory_equipment) = inventory_equipment_query.get_single_mut() {
            // clear old equipment
            commands.entity(inventory_equipment).despawn_descendants();
            let mut weapon_name = "None".to_string();
            let mut chest_name = "None".to_string();
            if let Some(weapon) = &player_equipment.weapon {
                weapon_name = weapon.name();
            }

            if let Some(chest) = &player_equipment.chest {
                chest_name = chest.name();
            }

            add_equipment_button(
                &mut commands,
                inventory_equipment,
                format!("Weapon: {}", weapon_name).as_str(),
                font.0.clone(),
                EquipmentButtonSlot::Weapon,
            );

            add_equipment_button(
                &mut commands,
                inventory_equipment,
                format!("Chest: {}", chest_name).as_str(),
                font.0.clone(),
                EquipmentButtonSlot::Chest,
            );
        }
    }
}

fn equip_inventory_item(
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

fn unequip_item_system(
    interaction_query: Query<
        (&Interaction, &EquipmentButtonSlot),
        (Changed<Interaction>, With<EquipmentButtonSlot>),
    >,
    mut player_equipment_query: Query<&mut Equipment, With<Player>>,
) {
    if let Ok(mut player_equipment) = player_equipment_query.get_single_mut() {
        for (interaction, slot) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match slot {
                    EquipmentButtonSlot::Weapon => player_equipment.weapon = None,
                    EquipmentButtonSlot::Chest => player_equipment.chest = None,
                }
            }
        }
    }
}

fn add_inventory_button(commands: &mut Commands, entity: Entity, name: &str, font: Handle<Font>) {
    commands.entity(entity).with_children(|parent| {
        parent
            .spawn(ButtonBundle {
                background_color: Color::NONE.into(),
                ..default()
            })
            .insert(OriginalColors {
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    name,
                    TextStyle {
                        font,
                        font_size: 20.0,
                        color: Color::srgb(0.7, 0.7, 0.7),
                    },
                ));
            });
    });
}

fn add_equipment_button(
    commands: &mut Commands,
    entity: Entity,
    name: &str,
    font: Handle<Font>,
    slot: EquipmentButtonSlot,
) {
    commands.entity(entity).with_children(|parent| {
        parent
            .spawn(ButtonBundle {
                background_color: Color::NONE.into(),
                ..default()
            })
            .insert(OriginalColors {
                ..Default::default()
            })
            .insert(slot)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    name,
                    TextStyle {
                        font,
                        font_size: 20.0,
                        color: Color::srgb(0.7, 0.7, 0.7),
                    },
                ));
            });
    });
}
