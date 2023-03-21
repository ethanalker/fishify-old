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
    let id = "01d8e617c3382c198f44d9418eb65fdef0cc368a".to_string();
    spotify.transfer_playback(&id, Some(false)).await?;
    Ok("Playback transfered to speaker".to_string())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("reconnect")
        .description("Reconnect to speaker")
}
