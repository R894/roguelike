use bevy::prelude::*;

use crate::{
    globals::TILE_SIZE,
    graphics::{assets::Ascii, tiles::spawn_sprite},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<Ascii>) {
    let player_sprite = Sprite {
        color: Color::rgb(0.3, 0.3, 0.9),
        custom_size: Some(Vec2::splat(TILE_SIZE)),
        ..default()
    };
    spawn_sprite(
        &mut commands,
        (TILE_SIZE, TILE_SIZE * -1., 300.),
        1,
        player_sprite,
        &ascii,
    );
}
