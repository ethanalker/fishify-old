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

pub async fn run(options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> String {
    let option = options
        .get(0)
        .expect("expected subcommand");

    let option_value = option
        .options
        .get(0)
        .expect("expected option")
        .resolved
        .as_ref()
        .expect("expected value");
    
    match option.name.as_str() {
        "repeat" => {
            if let CommandDataOptionValue::Boolean(value) = option_value {
                if *value {
                    spotify.repeat(&RepeatState::Context, None).await.expect("request failed");
                    format!("Set repeat to on")
                } else {
                    spotify.repeat(&RepeatState::Off, None).await.expect("request failed");
                    format!("Set repeat to off")
                }
            } else {
                "Failed: expected boolean value".to_string()
            }
        }
        "shuffle" => {
            if let CommandDataOptionValue::Boolean(value) = option_value {
                spotify.shuffle(*value, None).await.expect("request failed");
                format!("Set shuffle to {}", value)
            } else {
                "Failed: expected boolean value".to_string()
            }
        }
        "volume" => {
            if let CommandDataOptionValue::Integer(value) = option_value {
                spotify.volume((*value).try_into().unwrap(), None).await.expect("request failed");
                format!("Set volume to {}", value)
            } else {
                "Failed: expected integer value".to_string()
            }
        }
        _ => "Failed: unknown subcommand".to_string(),
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
