use bevy::prelude::*;

use crate::{
    board::components::Position,
    graphics::{get_world_position, PIECE_SPEED},
    player::Player,
    states::MainState,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(FixedUpdate, follow_player.run_if(in_state(MainState::Game)));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn follow_player(
    player_query: Query<&Position, With<Player>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let Ok(player_position) = player_query.get_single() else {
        return;
    };
    let mut camera_transform = camera_query.single_mut();

    let position = get_world_position(player_position, 900.);

    camera_transform.translation = camera_transform
        .translation
        .lerp(position, PIECE_SPEED * time.delta_seconds());
}
