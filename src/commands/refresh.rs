use log::warn;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
};

use rspotify::{
    AuthCodeSpotify,
    clients::OAuthClient,
};

use crate::CommandError;

pub async fn run(_options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    spotify.transfer_playback("fishify", Some(true)).await?;
    Ok("Playback transfered to speaker".to_string())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("refresh")
        .description("Transfer playback to speaker")
}
