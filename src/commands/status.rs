use log::warn;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
};

use rspotify::{
    AuthCodeSpotify,
    clients::OAuthClient,
    model::enums::types::AdditionalType,
};

use crate::CommandError;

pub async fn run(_options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let additional_type = AdditionalType::Track;
    let playback = spotify.current_playback(None, Some(vec![&additional_type]))
        .await?
        .ok_or("No current playback")?;

    let name = playback.device.name;
    let id = match playback.device.id {
        Some(id) => id,
        None => { warn!("No device id"); "None".to_string() }
    };
    let response: String = format!("Currently playing on {name} (id: {id})");
    Ok(response)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("status")
        .description("Playback status")
}
