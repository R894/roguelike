mod systems;

use bevy::prelude::*;

use crate::states::MainState;

pub const NORMAL_BUTTON: Color = Color::rgb(0.8, 0.8, 0.8);
pub const HOVERED_BUTTON: Color = Color::rgb(0.9, 0.9, 0.9);
pub const PRESSED_BUTTON: Color = Color::rgb(0.6, 0.6, 0.6);

#[derive(Component)]
pub struct TextBox;

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
            .add_systems(Update, button_system.run_if(in_state(MainState::Menu)))
            .add_systems(Update, button_system.run_if(in_state(MainState::GameOver)))
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
}

pub fn spawn_textbox(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    textbox_text: &str,
    (width, height): (f32, f32),
) -> Entity {
    let image = asset_server.load("textures/fantasy_ui_borders/panel-border-010.png");

    let slicer = TextureSlicer {
        border: BorderRect::square(22.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    let textbox = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(width),
                    height: Val::Px(height),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: image.clone().into(),
                background_color: Color::YELLOW.into(),
                ..default()
            },
            ImageScaleMode::Sliced(slicer.clone()),
        ))
        .insert(TextBox)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                textbox_text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.7, 0.7, 0.7),
                },
            ));
        })
        .id();

    commands
        .spawn(NodeBundle { ..default() })
        .push_children(&[textbox])
        .id()
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<NextState<MainState>>,
) {
    for (interaction, mut bg, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *bg = PRESSED_BUTTON.into();
                text.sections[0].style.color = PRESSED_BUTTON;
                state.set(MainState::Game);
            }
            Interaction::Hovered => {
                *bg = HOVERED_BUTTON.into();
                text.sections[0].style.color = HOVERED_BUTTON;
            }
            Interaction::None => {
                *bg = NORMAL_BUTTON.into();
                text.sections[0].style.color = NORMAL_BUTTON;
            }
        }
    }
}