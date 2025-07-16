use bevy::prelude::*;

use crate::{GameState, MainMenuState, PauseOptionsState, PauseState};

#[derive(Component)]
pub struct PausedUI;

#[derive(Component)]
pub struct PausedMenu;

const PAUSED_MENU_LIST: [&str; 2] = ["Options", "Return To Main Menu"];

pub fn spawn_paused_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("ui/fonts/pixeloid_mono.ttf");
    let font_bold = asset_server.load("ui/fonts/pixeloid_mono_bold.ttf");

    commands
        .spawn((
            PausedUI,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
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
                        Text::new("Paused"),
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
            PAUSED_MENU_LIST.iter().for_each(|label| {
                parent
                    .spawn((
                        PausedMenu,
                        Name::new(label.to_string()),
                        Button,
                        Node {
                            width: Val::Px(648.),
                            height: Val::Px(96.),
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

pub fn button_pressed_handler(
    button_query: Query<(&Interaction, &Name), Changed<Interaction>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    mut next_main_menu_state: ResMut<NextState<MainMenuState>>,
    mut next_pause_options_state: ResMut<NextState<PauseOptionsState>>,
) {
    for (interaction, name) in button_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match name.as_str() {
            "Options" => {
                next_pause_options_state.set(PauseOptionsState::Options);
            }
            "Return To Main Menu" => {
                next_pause_state.set(PauseState::None);
                next_pause_options_state.set(PauseOptionsState::None);
                next_game_state.set(GameState::MainMenu);
                next_main_menu_state.set(MainMenuState::MainMenu);
            }
            _ => return,
        }
    }
}

// Toggle the paused state when Escape is pressed
pub fn pause_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<PauseState>>,
    mut next_pause_options_state: ResMut<NextState<PauseOptionsState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(PauseState::Paused);
        next_pause_options_state.set(PauseOptionsState::Paused);
    }
}

pub fn un_pause_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<PauseState>>,
    mut next_pause_options_state: ResMut<NextState<PauseOptionsState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(PauseState::InGame);
        next_pause_options_state.set(PauseOptionsState::None);
    }
}

pub fn despawn_paused_menu(mut commands: Commands, query: Query<Entity, With<PausedUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
