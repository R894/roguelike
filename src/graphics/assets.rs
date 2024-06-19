use bevy::prelude::*;

pub const TILE_SIZE: f32 = 32.;

#[derive(Resource)]
pub struct Ascii {
    pub texture: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
}

#[derive(Component)]
pub struct AsciiText;
