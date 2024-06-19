mod camera;

mod board;
mod globals;
mod graphics;
mod player;
mod states;
mod ui;
mod vectors;

use bevy::prelude::*;
use board::BoardPlugin;
use camera::CameraPlugin;

use graphics::GraphicsPlugin;
use player::PlayerPlugin;
use states::MainState;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (globals::WINDOW_WIDTH, globals::WINDOW_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<MainState>()
        .insert_resource(Msaa::Off)
        .add_plugins(CameraPlugin)
        .add_plugins(GraphicsPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(BoardPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
