use bevy::prelude::*;
use bevy_light_2d::prelude::*;
use rand::prelude::*;

const MAIN_MENU_WIDTH: f32 = 1280.0;
const MAIN_MENU_HEIGHT: f32 = 640.0;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct MainMenuLight;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_image = asset_server.load("ui/main_menu/bg.png");
    let logo_image = asset_server.load("ui/main_menu/logo.png");

    commands
        .spawn((
            MainMenuUI,
            Sprite::from_image(background_image),
            Transform::from_xyz(0., 0., 0.),
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite::from_image(logo_image),
                Transform::from_xyz(0., (MAIN_MENU_HEIGHT / 2.) - (46. / 2.) - 40., 10.),
            ));
        })
        .with_children(|parent| {
            parent.spawn((
                MainMenuLight,
                PointLight2d {
                    intensity: 5.0,
                    radius: 180.,
                    color: Color::srgb(237. / 255., 202. / 225., 148. / 255.),
                    ..Default::default()
                },
                Transform::from_xyz((MAIN_MENU_WIDTH / 2.) - 270., -60., 20.),
            ));
        });
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

// pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
//     for entity in query.iter() {
//         commands.entity(entity).despawn();
//     }
// }
