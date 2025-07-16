use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::MASTER_VOLUME;

pub fn play_soundtrack(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("soundtracks/main_menu/bro_turned_into_horror.ogg"))
        .with_volume(MASTER_VOLUME)
        .looped();
}

pub fn stop_playing_soundtrack(audio: Res<Audio>) {
    audio.stop();
}
