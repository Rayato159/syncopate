use bevy::{prelude::*, window::WindowMode};

use crate::{GameOptions, MainMenuState, WindowModeSelection};

#[derive(Component)]
pub struct OptionsUI;

#[derive(Component)]
pub struct ScreenModeButton;

pub fn spawn_options_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("ui/fonts/pixeloid_mono.ttf");
    let font_bold = asset_server.load("ui/fonts/pixeloid_mono_bold.ttf");

    commands
        .spawn((
            OptionsUI,
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
                        Text::new("Options"),
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
        .with_children(|parent_1| {
            parent_1
                .spawn(Node {
                    width: Val::Percent(80.),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Relative,
                    ..Default::default()
                })
                .with_children(|parent_2| {
                    parent_2.spawn((
                        Text::new("Screen Mode"),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextFont {
                            font: font.clone(),
                            font_size: 48.,
                            ..Default::default()
                        },
                    ));

                    parent_2
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(32.),
                            align_items: AlignItems::Center,
                            position_type: PositionType::Relative,
                            ..Default::default()
                        })
                        .with_children(|parent_3| {
                            parent_3
                                .spawn((
                                    ScreenModeButton,
                                    Name::new("Fullscreen"),
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
                                                Text::new("Fullscreen"),
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

                            parent_3
                                .spawn((
                                    ScreenModeButton,
                                    Name::new("Windowed"),
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
                                                Text::new("Windowed"),
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
                        });
                });

            parent_1
                .spawn(Node {
                    width: Val::Percent(80.),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Relative,
                    ..Default::default()
                })
                .with_children(|parent_2| {
                    parent_2.spawn((
                        Text::new("Music Volume"),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextFont {
                            font: font.clone(),
                            font_size: 48.,
                            ..Default::default()
                        },
                    ));
                })
                .with_children(|parent_2| {
                    parent_2
                        .spawn((
                            Name::new("Music Volume"),
                            Button,
                            Node {
                                width: Val::Px(502.),
                                height: Val::Px(88.),
                                margin: UiRect::right(Val::Px(16.)),
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
                            BackgroundColor(Color::WHITE.with_alpha(1.0)),
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
                                        Text::new("100%"),
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
                });

            parent_1
                .spawn((
                    Name::new("Back"),
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
                                Text::new("Back"),
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
        });
}

pub fn button_pressed_handler(
    button_query: Query<(&Interaction, &Name), Changed<Interaction>>,
    mut next_state: ResMut<NextState<MainMenuState>>,
) {
    for (interaction, name) in button_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match name.as_str() {
            "Back" => next_state.set(MainMenuState::MainMenu),
            _ => return,
        }
    }
}

pub fn screen_mode_button_marker(
    mut button_query: Query<(&Name, &mut BackgroundColor), With<ScreenModeButton>>,
    game_options: Res<GameOptions>,
) {
    for (name, mut background_color) in button_query.iter_mut() {
        match game_options.window_mode {
            WindowModeSelection::Fullscreen => {
                if name.as_str() == "Fullscreen" {
                    *background_color = BackgroundColor(Color::WHITE.with_alpha(0.15));
                } else {
                    *background_color = BackgroundColor(Color::NONE);
                }
            }
            WindowModeSelection::Windowed => {
                if name.as_str() == "Windowed" {
                    *background_color = BackgroundColor(Color::WHITE.with_alpha(0.15));
                } else {
                    *background_color = BackgroundColor(Color::NONE);
                }
            }
        }
    }
}

pub fn screen_mode_button_handler(
    button_query: Query<(&Interaction, &Name), Changed<Interaction>>,
    mut game_options: ResMut<GameOptions>,
    mut windows_query: Query<&mut Window>,
) {
    for (interaction, name) in button_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match name.as_str() {
            "Fullscreen" => {
                if game_options.window_mode == WindowModeSelection::Fullscreen {
                    return; // Already in fullscreen mode
                }

                if let Ok(mut window) = windows_query.single_mut() {
                    window.mode = WindowMode::Fullscreen(
                        MonitorSelection::Primary,
                        VideoModeSelection::Current,
                    );
                    game_options.window_mode = WindowModeSelection::Fullscreen;
                }
            }
            "Windowed" => {
                if game_options.window_mode == WindowModeSelection::Windowed {
                    return; // Already in windowed mode
                }

                if let Ok(mut window) = windows_query.single_mut() {
                    window.mode = WindowMode::Windowed;
                    game_options.window_mode = WindowModeSelection::Windowed;
                }
            }
            _ => return,
        }
    }
}

pub fn back_by_keyboard_input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MainMenuState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(MainMenuState::MainMenu);
    }
}

pub fn despawn_options_menu(mut commands: Commands, query: Query<Entity, With<OptionsUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
