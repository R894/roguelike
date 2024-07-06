use bevy::prelude::*;

use crate::pieces::equipment::Item;

#[derive(Component, Default)]
pub struct Inventory {
    pub items: Vec<Item>,
}
