use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Resource)]
pub struct Ascii {
    pub texture: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
}

#[derive(Component)]
pub struct PathAnimator {
    pub path: VecDeque<Vec3>,
    pub speed_modifier: f32,
}
