mod camera;

mod actions;
mod board;
mod globals;
mod graphics;
mod input;
mod manager;
mod menus;
mod pieces;
mod player;
mod states;
mod ui;
mod vectors;

use bevy::prelude::*;
use states::{GameState, MainState};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Roguelike".to_string(),
                        resolution: (globals::WINDOW_WIDTH, globals::WINDOW_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<MainState>()
        .init_state::<GameState>()
        .insert_resource(Msaa::Off)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(actions::ActionsPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(board::BoardPlugin)
        .add_plugins(graphics::GraphicsPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(manager::ManagerPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(pieces::PiecesPlugin)
        .add_plugins(menus::MenuPlugin)
        .run();
}
