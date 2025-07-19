pub mod in_game_options_menu;
pub mod main_menu;
pub mod options;
pub mod paused_menu;

use bevy::{prelude::*, window::WindowMode};

use crate::{GameOptions, WindowModeSelection};

#[derive(Component)]
pub struct ScreenModeButton;

#[derive(Component)]
pub struct MusicVolumeLevel;

pub fn screen_mode_button_marker(
    mut button_query: Query<(&Name, &mut BackgroundColor), With<ScreenModeButton>>,
    game_options: Res<GameOptions>,
) {
    for (name, mut background_color) in button_query.iter_mut() {
        match game_options.window_mode {
            WindowModeSelection::Fullscreen => {
                if name.as_str() == "Fullscreen" {
                    *background_color = BackgroundColor(Color::WHITE.with_alpha(0.15));
                }
            }
            WindowModeSelection::Windowed => {
                if name.as_str() == "Windowed" {
                    *background_color = BackgroundColor(Color::WHITE.with_alpha(0.15));
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
                    window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
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
