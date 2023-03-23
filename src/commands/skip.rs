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
};

use crate::CommandError;
use crate::values_from_options;
use crate::int_from_value;

pub async fn run(options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let values: Vec<&CommandDataOptionValue> = values_from_options(options)?;

    let repeat = int_from_value(&values, 0, Some(1))?;

    for _ in 0..repeat {
        spotify.next_track(None).await?;
    }
    Ok("Skipped tracks".to_string())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("skip")
        .description("Skip to next song")
        .create_option(|option| {
            option
                .name("count")
                .description("Number of songs to skip")
                .kind(CommandOptionType::Integer)
                .min_int_value(1)
                .max_int_value(10)
                .required(false)
        })
}
