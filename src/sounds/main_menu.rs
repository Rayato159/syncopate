use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::GameOptions;

pub fn play_soundtrack(
    asset_server: Res<AssetServer>,
    game_options: Res<GameOptions>,
    audio: Res<Audio>,
) {
    audio
        .play(asset_server.load("soundtracks/main_menu/ready_or_not.ogg"))
        .with_volume(game_options.music_volume)
        .looped();
}

pub fn stop_playing_soundtrack(audio: Res<Audio>) {
    audio.stop();
}
