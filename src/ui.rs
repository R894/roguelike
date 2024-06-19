use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.8, 0.8, 0.8);
pub const HOVERED_BUTTON: Color = Color::rgb(0.9, 0.9, 0.9);
pub const PRESSED_BUTTON: Color = Color::rgb(0.6, 0.6, 0.6);

#[derive(Component)]
pub struct TextBox;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
    }
}

pub fn test_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let textbox = spawn_textbox(&mut commands, &asset_server, "Hey there :)", (300., 300.));
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
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
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
) {
    for (interaction, mut bg, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *bg = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *bg = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *bg = NORMAL_BUTTON.into();
            }
        }
    }
}
