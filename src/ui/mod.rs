mod systems;

use belly::prelude::*;
use bevy::prelude::*;

use crate::states::MainState;

pub const NORMAL_BUTTON: Color = Color::rgb(0.8, 0.8, 0.8);
pub const HOVERED_BUTTON: Color = Color::rgb(0.9, 0.9, 0.9);
pub const PRESSED_BUTTON: Color = Color::rgb(0.6, 0.6, 0.6);

#[derive(Component)]
pub struct TextBox;

#[derive(Component, Default)]
pub struct OriginalColors {
    pub text: ButtonColorOptions,
    pub background: Option<ButtonColorOptions>,
}

#[derive(Clone)]
pub struct ButtonColorOptions {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

impl Default for ButtonColorOptions {
    fn default() -> Self {
        Self {
            normal: NORMAL_BUTTON,
            hovered: HOVERED_BUTTON,
            pressed: PRESSED_BUTTON,
        }
    }
}
pub struct UiPlugin;

#[derive(Resource)]
pub struct UiFont(pub Handle<Font>);

#[derive(Resource)]
pub struct BorderTexture(pub Handle<Image>);

#[derive(Component)]
pub struct UiHealth;

#[derive(Component)]
pub struct UiGold;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(Update, button_system)
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
    let border_texture: Handle<Image> =
        asset_server.load("textures/fantasy_ui_borders/panel-border-010.png");
    commands.insert_resource(UiFont(font));
    commands.insert_resource(BorderTexture(border_texture));
    commands.add(StyleSheet::load("stylesheet.ess"));
}

pub fn spawn_button(
    commands: &mut Commands,
    border_texture: &Res<BorderTexture>,
    font: &Res<UiFont>,
    button_text: &str,
    (width, height): (f32, f32),
) -> Entity {
    let image = &border_texture.0;

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
                ..default()
            },
            ImageScaleMode::Sliced(slicer.clone()),
        ))
        .insert(OriginalColors { ..default() })
        .insert(TextBox)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                button_text,
                TextStyle {
                    font: font.0.clone(),
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

#[allow(clippy::type_complexity)]
fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &OriginalColors,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut bg, children, colors) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if let Some(bg_colors) = &colors.background {
                    *bg = bg_colors.pressed.into();
                }
                text.sections[0].style.color = colors.text.pressed;
            }
            Interaction::Hovered => {
                if let Some(bg_colors) = &colors.background {
                    *bg = bg_colors.hovered.into();
                }
                text.sections[0].style.color = colors.text.hovered;
            }
            Interaction::None => {
                if let Some(bg_colors) = &colors.background {
                    *bg = bg_colors.normal.into();
                }
                text.sections[0].style.color = colors.text.normal;
            }
        }
    }
}
