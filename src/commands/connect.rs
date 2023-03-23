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

    let devices: Vec<Device> = spotify.device().await?;

    let device = match str_from_value(&values, 0, None) {
        Ok(target) => devices.iter().find(|device| device.name == target),
        Err(_) => devices.get(0),
    };

    if let Some(dev) = device {
        let id = dev.id.as_ref().ok_or("Missing id")?;
        let name = &dev.name;

        spotify.transfer_playback(&id, None).await?;
        Ok(format!("Playback transfered to {name}"))
    } else {
        Err(CommandError::from("Device not found"))
    }
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
