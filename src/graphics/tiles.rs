use bevy::prelude::*;

use crate::board::components::{Position, Tile};

use super::{
    assets::{Ascii, AsciiText},
    TILE_SIZE, TILE_Z,
};

const ATLAS_PATH: &str = "textures/Ascii.png";

pub fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let layout =
        TextureAtlasLayout::from_grid(Vec2::splat(9.0), 16, 16, Some(Vec2::splat(2.0)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let texture = asset_server.load(ATLAS_PATH);

    commands.insert_resource(Ascii {
        texture: texture_atlas_layout,
        image: texture,
    });
}

pub fn spawn_ascii_text(
    commands: &mut Commands,
    ascii: &Res<Ascii>,
    to_print: &str,
    left_center: Vec3,
) -> Entity {
    let mut character_sprites = Vec::new();
    for (i, char) in to_print.chars().enumerate() {
        assert!(char as usize <= 255);
        character_sprites.push(spawn_sprite(
            commands,
            (i as f32 * TILE_SIZE, 0.0, 0.0),
            char as usize,
            Sprite {
                color: Color::rgb(0.8, 0.8, 0.8),
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            ascii,
        ))
    }

    commands
        .spawn(Name::new(format!("Text - {}", to_print)))
        .insert(AsciiText)
        .insert(Transform {
            translation: left_center,
            ..default()
        })
        .insert(GlobalTransform::default())
        .push_children(&character_sprites)
        .id()
}

pub fn spawn_sprite(
    commands: &mut Commands,
    translation: (f32, f32, f32),
    sprite_index: usize,
    sprite: Sprite,
    ascii: &Res<Ascii>,
) -> Entity {
    commands
        .spawn(SpriteSheetBundle {
            sprite,
            transform: Transform {
                translation: Vec3::new(translation.0, translation.1, translation.2),
                ..default()
            },
            texture: ascii.image.clone(),
            atlas: TextureAtlas {
                index: sprite_index,
                layout: ascii.texture.clone(),
            },
            ..default()
        })
        .id()
}

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    assets: Res<Ascii>,
) {
    for (entity, position) in query.iter() {
        let sprite = Sprite {
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..default()
        };

        let v = super::get_world_position(position, TILE_Z);
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture: assets.image.clone(),
            transform: Transform::from_translation(Vec3::new(
                v.x + TILE_SIZE / 8.,
                v.y + TILE_SIZE / 4.,
                v.z,
            )),
            atlas: TextureAtlas {
                index: '.' as usize,
                layout: assets.texture.clone(),
            },
            ..Default::default()
        });
    }
}
