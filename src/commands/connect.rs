use log::warn;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};

use rspotify::{
    AuthCodeSpotify,
    clients::OAuthClient,
    model::device::Device,
};

use crate::CommandError;
use crate::values_from_options;
use crate::str_from_value;

pub async fn run(options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let values: Vec<&CommandDataOptionValue> = values_from_options(options)?;

    let target = str_from_value(&values, 0, Some("fishipi"))?;

    let devices: Vec<Device> = spotify.device().await?;

    for device in devices {
        if device.name == target {
            let id = device.id.ok_or("No device id")?;

            spotify.transfer_playback(&id, Some(false)).await?;
            return Ok(format!("Playback transfered to {target}"));
        }
    }

    Err(CommandError::from("Failed to find device"))
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("connect")
        .description("Connect to speaker")
        .create_option(|option| {
            option
                .name("name")
                .description("Name of device to connect to")
                .kind(CommandOptionType::String)
                .required(false)
        })
}
