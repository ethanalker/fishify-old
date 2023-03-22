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

    let mut response: String = "".to_string();

    for device in devices {
        let name = &device.name;
        let id = match &device.id {
            Some(val) => val,
            None => "NA",
        };

        response.push_str("> ");
        response.push_str(name);
        response.push_str(" - ");
        response.push_str(id);
        response.push_str("\n");
    }

    Ok(response)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("devices")
        .description("List devices")
}
