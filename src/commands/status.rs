use std::time::Duration;

use log::warn;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
};

use rspotify::{
    AuthCodeSpotify,
    clients::OAuthClient,
    model::enums::types::AdditionalType,
    model::PlayableItem,
    model::enums::misc::RepeatState,
    model::device::Device,
};

use crate::CommandError;

pub async fn run(_options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let playback = spotify.current_playback(None, None::<Vec<&AdditionalType>>)
        .await?
        .ok_or("No current playback")?;

    // This will create a message with the format:
    //   {is_playing}
    //   {name} --- {artist}
    //   {progress} / {duration}
    //   Volume: {volume}%
    //   Shuffle: {shuffle}
    //   Repeat: {repeat}

    // increase capacity when adding more lines
    let mut status: Vec<String> = Vec::with_capacity(6);

    let device: Device = playback.device;
    let repeat: RepeatState = playback.repeat_state;
    let shuffle: bool = playback.shuffle_state;
    let playback_progress: Option<Duration> = playback.progress;
    let is_playing: bool = playback.is_playing;
    let item: Option<PlayableItem> = playback.item;

    // first line
    if is_playing {
        status.push("> Playing".to_string());
    } else {
        status.push("> Paused".to_string());
    }

    // unpack item info
    let (item_name, item_artist, item_duration) = match &item {
        Some(PlayableItem::Track(item)) => {
            (Some(&item.name), Some(&item.artists[0].name), Some(&item.duration)) 
        }
        Some(PlayableItem::Episode(item)) => {
            (Some(&item.name), None, Some(&item.duration))
        }
        None => {
            (None, None, None)
        }
    };

    // second line
    match (item_name, item_artist) {
        (Some(name), Some(artist)) => status.push(format!("{name} \u{2014} {artist}")),
        (Some(name), None) => status.push(format!("{name}")),
        (None, _) => (),
    }

    // third line
    if let (Some(progress), Some(duration)) = (playback_progress, item_duration) {
        let progress_seconds = progress.as_secs();
        let duration_seconds = duration.as_secs();
        if duration.as_secs() / (60 * 60) > 0 {
            status.push(format!("{}:{:0>2}:{:0>2} / {}:{:0>2}:{:0>2}",
                progress_seconds / 60 / 60,
                progress_seconds / 60 % 60,
                progress_seconds % 60,
                duration_seconds / 60 / 60,
                duration_seconds / 60 % 60,
                duration_seconds % 60,
            ));
        } else {
            status.push(format!("{}:{:0>2} / {}:{:0>2}",
                progress_seconds / 60 % 60,
                progress_seconds % 60,
                duration_seconds / 60 % 60,
                duration_seconds % 60,
            ));
        }
    }

    // fourth line
    if let Some(volume) = device.volume_percent {
        status.push(format!("Volume: {volume}%"));
    }

    // fifth line
    if shuffle {
        status.push("Shuffle: On".to_string());
    } else {
        status.push("Shuffle: Off".to_string());
    }

    // sixth line
    match repeat {
        RepeatState::Off => status.push("Repeat: Off".to_string()),
        _ => status.push("Repeat: On".to_string()),
    }

    Ok(status.join("\n> "))
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("status")
        .description("Show playback status")
}
