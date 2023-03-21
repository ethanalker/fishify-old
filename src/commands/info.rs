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
    model::enums::types::DeviceType,
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
    //   Device: {name}
    //   Id: {id}
    //   Active: {active}
    //   Type: {type}
    //

    // increase capacity when adding more lines
    let mut info: Vec<String> = Vec::with_capacity(4);

    let device: Device = playback.device;
    let name: String = device.name;
    let id: String = match device.id {
        Some(id) => id,
        None => "N/A".to_string(),
    };
    let active: String = device.is_active.to_string();
    let _type: String = match device._type {
        DeviceType::Computer => "Computer".to_string(),
        DeviceType::Tablet => "Tablet".to_string(),
        DeviceType::Smartphone => "Smartphone".to_string(),
        DeviceType::Speaker => "Speaker".to_string(),
        DeviceType::Tv => "Tv".to_string(),
        DeviceType::Avr => "Avr".to_string(),
        DeviceType::Stb => "Stb".to_string(),
        DeviceType::AudioDongle => "AudioDongle".to_string(),
        DeviceType::GameConsole => "GameConsole".to_string(),
        DeviceType::CastVideo => "CastVideo".to_string(),
        DeviceType::CastAudio => "CastAudio".to_string(),
        DeviceType::Automobile => "Automobile".to_string(),
        DeviceType::Unknown => "Unknown".to_string(),
    };

    // first line, has > to quote
    info.push(format!("> Device: {name}"));
    info.push(format!("Id: {id}"));
    info.push(format!("Active: {active}"));
    info.push(format!("Type: {_type}"));

    Ok(info.join("\n> "))
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("info")
        .description("Show playback device info")
}
