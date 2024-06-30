use bevy::prelude::*;

use crate::board::components::{Position, Tile, Wall};

use super::{assets::Ascii, TILE_SIZE, TILE_Z};

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
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture: assets.image.clone(),
            transform: Transform::from_translation(Vec3::new(v.x, v.y, v.z)),
            atlas: TextureAtlas {
                index: '.' as usize,
                layout: assets.texture.clone(),
            },
            ..Default::default()
        });
    }

    for entity in wall_query.iter() {
        let sprite = Sprite {
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..default()
        };

        let bundle = SpriteSheetBundle {
            sprite,
            texture: assets.image.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            atlas: TextureAtlas {
                index: '#' as usize,
                layout: assets.texture.clone(),
            },
            ..Default::default()
        };
        commands.entity(entity).insert(bundle);
    }
}

pub fn update_tile_visibility(mut query: Query<(&mut Visibility, &Tile), Changed<Tile>>) {
    for (mut visibility, tile) in query.iter_mut() {
        if !tile.visible {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}
