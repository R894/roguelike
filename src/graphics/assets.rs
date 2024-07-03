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
    pub speed_multiplier: f32,
    pub instant: bool,
}

impl Default for PathAnimator {
    fn default() -> Self {
        Self {
            path: VecDeque::new(),
            speed_multiplier: 1.,
            instant: false,
        }
    }
}
