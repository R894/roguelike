use bevy::prelude::*;

#[derive(Resource)]
pub struct Ascii {
    pub texture: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
}

#[derive(Component)]
pub struct AsciiText;
