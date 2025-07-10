use bevy::prelude::*;
use bevy_light_2d::prelude::*;
use rand::prelude::*;

use crate::{GameState, camera::MainMenuCamera};

const MAIN_MENU_WIDTH: f32 = 1920.;

#[derive(Component)]
pub struct MainMenuScene;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct MainMenuLight;

#[derive(Component)]
pub struct Menu;

const MAIN_MENU_LIST: [&str; 4] = ["New Game", "Load Game", "Options", "Quit"];

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_image = asset_server.load("ui/main_menu/bg.png");
    let font = asset_server.load("ui/fonts/pixeloid_mono.ttf");
    let font_bold = asset_server.load("ui/fonts/pixeloid_mono_bold.ttf");

    commands
        .spawn((MainMenuScene, Sprite::from_image(background_image)))
        .with_children(|parent| {
            parent.spawn((
                MainMenuLight,
                PointLight2d {
                    intensity: 5.0,
                    radius: 180.,
                    color: Color::srgb(237. / 255., 202. / 225., 148. / 255.),
                    ..Default::default()
                },
                Transform::from_xyz((MAIN_MENU_WIDTH / 2.) - 415., -90., 500.),
            ));
        });

    commands
        .spawn((
            MainMenuUI,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            parent
                .spawn({
                    Node {
                        width: Val::Percent(100.),
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        ..Default::default()
                    }
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Syncopate"),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextFont {
                            font: font_bold.clone(),
                            font_size: 94.,
                            ..Default::default()
                        },
                    ));
                });
        })
        .with_children(|parent| {
            MAIN_MENU_LIST.iter().for_each(|label| {
                parent
                    .spawn((
                        Menu,
                        Name::new(label.to_string()),
                        Button,
                        Node {
                            width: Val::Px(502.),
                            height: Val::Px(88.),
                            position_type: PositionType::Relative,
                            border: UiRect {
                                left: Val::Px(2.),
                                right: Val::Px(2.),
                                top: Val::Px(2.),
                                bottom: Val::Px(2.),
                            },
                            ..Default::default()
                        },
                        BorderColor(Color::WHITE),
                        BackgroundColor(Color::WHITE.with_alpha(0.0)),
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn(Node {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new(label.to_string()),
                                    TextColor(Color::WHITE),
                                    TextLayout::new_with_justify(JustifyText::Center),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 48.,
                                        ..Default::default()
                                    },
                                ));
                            });
                    });
            })
        });
}

pub fn ui_interaction(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgba(0.8, 0.8, 0.8, 0.15));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgba(0.8, 0.8, 0.8, 0.07));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::NONE);
            }
        }
    }
}

pub fn button_pressed_handler(
    button_query: Query<(&Interaction, &Name), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, name) in button_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match name.as_str() {
            "New Game" => {
                next_state.set(GameState::InGame);
            }
            "Load Game" => return,
            "Options" => return,
            "Quit" => {
                std::process::exit(0);
            }
            _ => return,
        }
    }
}

#[derive(Resource)]
pub struct MainMenuLightFlickerTimer {
    pub timer: Timer,
}

impl Default for MainMenuLightFlickerTimer {
    fn default() -> Self {
        MainMenuLightFlickerTimer {
            timer: Timer::from_seconds(0.08, TimerMode::Repeating),
        }
    }
}

pub fn light_flicker(
    time: Res<Time>,
    mut flicker_timer: ResMut<MainMenuLightFlickerTimer>,
    mut query: Query<&mut PointLight2d, With<MainMenuLight>>,
) {
    if flicker_timer.timer.tick(time.delta()).just_finished() {
        let intensity: f32 = rand::rng().random_range(2.0..8.0);
        for mut light in query.iter_mut() {
            light.intensity = intensity;
        }
    }
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_ui_query: Query<
        Entity,
        (
            With<MainMenuUI>,
            Without<MainMenuScene>,
            Without<MainMenuCamera>,
        ),
    >,
    main_menu_scene_query: Query<
        Entity,
        (
            With<MainMenuScene>,
            Without<MainMenuUI>,
            Without<MainMenuCamera>,
        ),
    >,
    main_menu_camera_query: Query<
        Entity,
        (
            With<MainMenuCamera>,
            Without<MainMenuUI>,
            Without<MainMenuScene>,
        ),
    >,
) {
    for entity in main_menu_camera_query.iter() {
        commands.entity(entity).despawn();
    }

    for entity in main_menu_ui_query.iter() {
        commands.entity(entity).despawn();
    }

    for entity in main_menu_scene_query.iter() {
        commands.entity(entity).despawn();
    }
}
