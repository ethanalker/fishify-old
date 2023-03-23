use log::warn;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
};

use rspotify::{
    AuthCodeSpotify,
    clients::OAuthClient,
    model::device::Device,
};

use crate::CommandError;

pub async fn run(_options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let devices: Vec<Device> = spotify.device().await?;

    if devices.len() == 0 {
        return Err(CommandError::from("No available devices"));
    }

    let mut response = String::new();

    for device in devices {
        let name = &device.name;
        let _type = &device._type;
        let id = match &device.id {
            Some(val) => val,
            None => "NA",
        };

        response.push_str(format!("> {_type:?} {name}\n> Id: {id}\n").as_ref());
    }

    Ok(response)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("devices")
        .description("List devices")
}
