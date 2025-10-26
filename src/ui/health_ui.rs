use bevy::prelude::*;

use crate::characters::thunwa::ThunwaHealth;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthFill;

#[derive(Resource)]
pub struct HealthFlickerTimer {
    pub timer: Timer,
    pub is_flickering: bool,
    pub flicker_count: i32,
    pub initialized: bool,
}

impl Default for HealthFlickerTimer {
    fn default() -> Self {
        HealthFlickerTimer {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            is_flickering: false,
            flicker_count: 0,
            initialized: false,
        }
    }
}

pub fn spawn_health_ui(mut commands: Commands) {
    commands
        .spawn((
            HealthBar,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                width: Val::Px(400.0),
                height: Val::Px(48.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect {
                    left: Val::Px(2.0),
                    right: Val::Px(2.0),
                    top: Val::Px(2.0),
                    bottom: Val::Px(2.0),
                },
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            BorderColor(Color::WHITE),
        ))
        .with_children(|parent| {
            // Health bar background
            parent
                .spawn(Node {
                    width: Val::Px(396.0),
                    height: Val::Px(44.0),
                    ..Default::default()
                })
                .insert(BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)))
                .with_children(|parent| {
                    // Health fill bar - Start with specified color
                    parent.spawn((
                        HealthFill,
                        Node {
                            width: Val::Px(396.0),
                            height: Val::Px(44.),
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            ..Default::default()
                        },
                        BackgroundColor(Color::srgba(
                            99.0 / 255.0,
                            163.0 / 255.0,
                            97.0 / 255.0,
                            1.0,
                        )),
                    ));
                });
        });
}

pub fn despawn_health_ui(mut commands: Commands, health_bar_query: Query<Entity, With<HealthBar>>) {
    for entity in health_bar_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update_health_ui(
    thunwa_health: Res<ThunwaHealth>,
    mut health_fill_query: Query<&mut Node, With<HealthFill>>,
    mut flicker_timer: ResMut<HealthFlickerTimer>,
) {
    // Skip flicker on first initialization
    if !flicker_timer.initialized {
        flicker_timer.initialized = true;
        return;
    }

    if thunwa_health.is_changed() && thunwa_health.current < thunwa_health.max {
        // Start flicker when health changes (indicating damage)
        flicker_timer.is_flickering = true;
        flicker_timer.timer.reset();
        flicker_timer.flicker_count = 0;

        // Update health bar fill width
        if let Ok(mut fill_node) = health_fill_query.single_mut() {
            let health_percentage = thunwa_health.current / thunwa_health.max;
            fill_node.width = Val::Px(396.0 * health_percentage);
        }
    }
}

pub fn update_health_bar_color(
    thunwa_health: Res<ThunwaHealth>,
    mut health_fill_query: Query<&mut BackgroundColor, With<HealthFill>>,
    flicker_timer: Res<HealthFlickerTimer>,
) {
    // Only update if not flickering (flicker handled separately)
    if !flicker_timer.is_flickering {
        if let Ok(mut bg_color) = health_fill_query.single_mut() {
            let health_percentage = thunwa_health.current / thunwa_health.max;

            // Normal color coding based on health percentage
            // Start color (> 80%), Yellow (50-80%), Red (< 50%)
            if health_percentage > 0.8 {
                bg_color.0 = Color::srgba(99.0 / 255.0, 163.0 / 255.0, 97.0 / 255.0, 1.0); // Start color
            } else if health_percentage > 0.5 {
                bg_color.0 = Color::srgb(0.8, 0.8, 0.0); // Yellow
            } else {
                bg_color.0 = Color::srgb(0.6, 0.2, 0.0); // Dark red (not as bright as flicker)
            }
        }
    }
}

pub fn update_health_flicker(
    time: Res<Time>,
    mut flicker_timer: ResMut<HealthFlickerTimer>,
    mut health_fill_query: Query<&mut BackgroundColor, With<HealthFill>>,
    thunwa_health: Res<ThunwaHealth>,
) {
    if flicker_timer.is_flickering {
        flicker_timer.timer.tick(time.delta());

        if let Ok(mut bg_color) = health_fill_query.single_mut() {
            // Simple red/normal alternation
            if flicker_timer.timer.just_finished() {
                flicker_timer.flicker_count += 1;
            }

            if flicker_timer.flicker_count % 2 == 0 {
                bg_color.0 = Color::srgb(0.8, 0.0, 0.0); // Red
            } else {
                // Normal health-based color
                let health_percentage = thunwa_health.current / thunwa_health.max;
                if health_percentage > 0.8 {
                    bg_color.0 = Color::srgba(99.0 / 255.0, 163.0 / 255.0, 97.0 / 255.0, 1.0);
                } else if health_percentage > 0.5 {
                    bg_color.0 = Color::srgb(0.8, 0.8, 0.0);
                } else {
                    bg_color.0 = Color::srgb(0.6, 0.2, 0.0);
                }
            }

            // Stop after 8 flickers (4 red, 4 normal = 0.8 seconds total)
            if flicker_timer.flicker_count >= 8 {
                flicker_timer.is_flickering = false;
                flicker_timer.flicker_count = 0;
                flicker_timer.timer.reset();
            }
        }
    }
}
