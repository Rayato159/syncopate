use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{
    GameOptions, MainMenuState,
    ui::{MusicVolumeLevel, ScreenModeButton},
};

#[derive(Component)]
pub struct OptionsUI;

pub fn spawn_options_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_options: Res<GameOptions>,
) {
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
                                    Button,
                                    ScreenModeButton,
                                    Name::new("Fullscreen"),
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
                                    Button,
                                    ScreenModeButton,
                                    Name::new("Windowed"),
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
                        ))
                        .with_children(|parent_3| {
                            parent_3
                                .spawn((
                                    Button,
                                    Name::new("Decrease Volume"),
                                    Node {
                                        width: Val::Px(56.),
                                        height: Val::Px(88.),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        position_type: PositionType::Relative,
                                        ..Default::default()
                                    },
                                ))
                                .with_children(|parent_2| {
                                    parent_2.spawn((
                                        Text::new("-"),
                                        TextColor(Color::WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 48.,
                                            ..Default::default()
                                        },
                                    ));
                                });

                            parent_3
                                .spawn(Node {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    justify_content: JustifyContent::FlexStart,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                })
                                .with_children(|parent_4| {
                                    parent_4
                                        .spawn((Node {
                                            width: Val::Percent(100.),
                                            height: Val::Percent(100.),
                                            ..Default::default()
                                        },))
                                        .with_children(|parent_5| {
                                            parent_5.spawn((
                                                MusicVolumeLevel,
                                                Node {
                                                    width: Val::Percent(
                                                        game_options.music_volume as f32 * 100.0,
                                                    ),
                                                    height: Val::Percent(100.),
                                                    ..Default::default()
                                                },
                                                BackgroundColor(Color::srgba(
                                                    163. / 255.,
                                                    220. / 255.,
                                                    154. / 255.,
                                                    1.0,
                                                )),
                                            ));
                                        });
                                });

                            parent_3
                                .spawn((
                                    Button,
                                    Name::new("Increase Volume"),
                                    Node {
                                        width: Val::Px(56.),
                                        height: Val::Px(88.),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        position_type: PositionType::Relative,
                                        ..Default::default()
                                    },
                                ))
                                .with_children(|parent_2| {
                                    parent_2.spawn((
                                        Text::new("+"),
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

pub fn music_volume_button_handler(
    button_query: Query<(&Interaction, &Name), Changed<Interaction>>,
    mut music_volume_query: Query<&mut Node, With<MusicVolumeLevel>>,
    kira_audio: Res<Audio>,
    mut game_options: ResMut<GameOptions>,
    mut next_state: ResMut<NextState<MainMenuState>>,
) {
    for (interaction, name) in button_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match name.as_str() {
            "Back" => next_state.set(MainMenuState::MainMenu),
            "Increase Volume" => {
                if game_options.music_volume >= 1.0 {
                    return; // Volume is already at maximum
                }

                game_options.music_volume = (game_options.music_volume + 0.1).clamp(0.0, 1.0);

                for mut node in music_volume_query.iter_mut() {
                    node.width = Val::Percent(game_options.music_volume as f32 * 100.0);
                }

                kira_audio.set_volume(game_options.music_volume);
            }
            "Decrease Volume" => {
                if game_options.music_volume <= 0.0 {
                    return; // Volume is already at minimum
                }

                game_options.music_volume = (game_options.music_volume - 0.1).clamp(0.0, 1.0);

                for mut node in music_volume_query.iter_mut() {
                    node.width = Val::Percent(game_options.music_volume as f32 * 100.0);
                }

                kira_audio.set_volume(game_options.music_volume);
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
