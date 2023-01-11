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
    model::enums::misc::RepeatState,
};

use crate::CommandError;

pub async fn run(options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let option = options
        .get(0)
        .ok_or("No argument")?;

    let option_value = option
        .options
        .get(0)
        .ok_or("No argument")?
        .resolved    
        .as_ref()
        .ok_or("No argument value")?;

    match option.name.as_str() {
        "repeat" => {
            if let CommandDataOptionValue::Boolean(value) = *option_value {
                if value {
                    spotify.repeat(RepeatState::Context, None).await?;
                    Ok("Set repeat to on".to_string())
                } else {
                    spotify.repeat(RepeatState::Off, None).await?;
                    Ok("Set repeat to off".to_string())
                }
            } else {
                Err(CommandError::from("Expected boolean value"))
            }
        }
        "shuffle" => {
            if let CommandDataOptionValue::Boolean(value) = *option_value {
                spotify.shuffle(value, None).await?;
                Ok(format!("Set shuffle to {}", value))
            } else {
                Err(CommandError::from("Expected boolean value"))
            }
        }
        "volume" => {
            if let CommandDataOptionValue::Integer(value) = *option_value {
                spotify.volume(value.try_into().unwrap(), None).await?;
                Ok(format!("Set volume to {}", value))
            } else {
                Err(CommandError::from("Expected integer value"))
            }
        }
        _ => Err(CommandError::from("Unknown subcommand"))
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("set")
        .description("Change settings for spotify music")
        .create_option(|option| {
            option
                .name("repeat")
                .description("enable or disable repeat")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("bool")
                        .description("true/false")
                        .kind(CommandOptionType::Boolean)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("shuffle")
                .description("enable or disable shuffle")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("bool")
                        .description("true/false")
                        .kind(CommandOptionType::Boolean)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("volume")
                .description("set volume")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("level")
                        .description("0 to 100")
                        .kind(CommandOptionType::Integer)
                        .min_int_value(0)
                        .max_int_value(100)
                        .required(true)
                })
        })
}
