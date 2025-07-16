use bevy::prelude::*;

use crate::MainMenuState;

#[derive(Component)]
pub struct OptionsUI;

#[derive(Component)]
pub struct OptionsMenu;

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
        .with_children(|parent| {
            parent
                .spawn((
                    OptionsMenu,
                    Name::new("Resolution"),
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
                                Text::new("Resolution"),
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

            parent
                .spawn((
                    OptionsMenu,
                    Name::new("Resolution"),
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
                                Text::new("Window Mode"),
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

            parent
                .spawn((
                    OptionsMenu,
                    Name::new("Resolution"),
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
                                Text::new("Audio Volume"),
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

            parent
                .spawn((
                    OptionsMenu,
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
            "Window Mode" => return,
            "Resolution" => return,
            "Audio" => return,
            "Back" => next_state.set(MainMenuState::MainMenu),
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
