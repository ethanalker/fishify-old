use log::warn;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
};

use rspotify::{
    AuthCodeSpotify,
    clients::OAuthClient,
    model::enums::types::AdditionalType,
    model::device::Device,
};

use crate::CommandError;

pub async fn run(_options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let playback = spotify.current_playback(None, None::<Vec<&AdditionalType>>)
        .await?
        .ok_or("No current playback")?;

    let device: Device = playback.device;
    let name: String = device.name;
    let id: String = match device.id {
        Some(id) => id,
        None => "N/A".to_string(),
    };
    let active: String = device.is_active.to_string();
    let _type = device._type;
    
    Ok(format!("
        > Device: {name}
        > Id: {id}
        > Active: {active}
        > Type: {_type:?}
    "))
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("info")
        .description("Show playback device info")
}
