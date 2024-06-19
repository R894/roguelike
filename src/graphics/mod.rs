use bevy::prelude::*;

pub mod assets;
pub mod tiles;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, tiles::setup)
            .add_systems(Update, tiles::spawn_tile_renderer);
    }
}
