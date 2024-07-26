mod systems;

use belly::prelude::*;
use bevy::prelude::*;

use crate::states::MainState;

pub struct UiPlugin;

#[derive(Resource)]
pub struct UiFont(pub Handle<Font>);

#[derive(Component)]
pub struct UiHealth;

#[derive(Component)]
pub struct UiGold;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(OnEnter(MainState::Game), systems::spawn_ui)
            .add_systems(
                Update,
                (systems::update_ui_gold, systems::update_ui_health)
                    .run_if(in_state(MainState::Game)),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.insert_resource(UiFont(font));
    commands.add(StyleSheet::load("stylesheet.ess"));
}
