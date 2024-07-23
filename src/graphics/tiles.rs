use bevy::prelude::*;

use crate::board::components::{Position, Tile, Wall};

use super::{assets::Ascii, TILE_SIZE, TILE_Z};

const ATLAS_PATH: &str = "textures/colored-transparent_packed.png";

pub fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(16.0), 49, 22, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let texture = asset_server.load(ATLAS_PATH);

    commands.insert_resource(Ascii {
        texture: texture_atlas_layout,
        image: texture,
    });
}

pub fn spawn_tile_renderer(
    mut commands: Commands,
    tile_query: Query<(Entity, &Position), Added<Tile>>,
    wall_query: Query<Entity, Added<Wall>>,
    assets: Res<Ascii>,
) {
    for (entity, position) in tile_query.iter() {
        let sprite = Sprite {
            color: Color::rgba(1., 1., 1., 0.5),
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..default()
        };

        let v = super::get_world_position(position, TILE_Z);
        commands
            .entity(entity)
            .insert(SpriteBundle {
                sprite,
                texture: assets.image.clone(),
                transform: Transform::from_translation(Vec3::new(v.x, v.y, v.z)),
                ..Default::default()
            })
            .insert(TextureAtlas {
                index: 1,
                layout: assets.texture.clone(),
            });
    }

    for entity in wall_query.iter() {
        let sprite = Sprite {
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..default()
        };

        let bundle = SpriteBundle {
            sprite,
            texture: assets.image.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        };

        let texture_atlas = TextureAtlas {
            index: 49,
            layout: assets.texture.clone(),
        };

        commands.entity(entity).insert(bundle).insert(texture_atlas);
    }
}
