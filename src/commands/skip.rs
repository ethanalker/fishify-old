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

// struct SkipData {
//     votes: u8,
//     id: String,
// }

// make this better, maybe global maybe do something smarter idk
// fix the ?????? shit
pub async fn run(options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let option = options.get(0).ok_or("Missing subcommand")?;

    match option.name.as_str() {
        "vote" => {
            // let id = spotify
            //     .current_user_playing_item()
            //     .await?
            //     .ok_or("No currently playing context")?
            //     .item
            //     .ok_or("No currently playing item")?
            //     .id()
            //     .ok_or("Current item does not have id")?
            //     .id();

            // let votes = fs::read_to_string("/tmp/fishify_skip_data/votes");
            // let skip_id = fs::read_to_string("/tmp/fishify_skip_data/id");

            // if skip.id == id {
            //     skip.votes = skip.votes + 1;
            // } else {
            //     skip.id = id.to_string();
            //     skip.votes = 1;
            // }

            // let result = if skip.votes >= 3 {
            //     match spotify.next_track(None).await {
            //         Ok(()) => "Successfuly skipped track".to_string(),
            //         Err(why) => format!("Failed to skip track: {}", why),
            //     }
            // } else {
            //     "Vote recorded".to_string()
            // };

            // match (fs::write("/tmp/fishify_skip_data/votes", skip.votes.to_string()),
            //     fs::write("/tmp/fishify_skip_data/id", skip.id))
            // {
            //     (Ok(()), Ok(())) => (),
            //     _ => warn!("Failed to write skip data to file"),
            // }

            // Ok(result)
            Ok("Not implemented Fuck You".to_string())
        }
        "force" => { 
            spotify.next_track(None).await?;
            Ok("Skipped track".to_string())
        }
        _ => Err(CommandError::from("Invalid subcommad"))
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("skip")
        .description("Skip currently playing song")
        .create_option(|option| {
            option
                .name("vote")
                .description("vote to skip")
                .kind(CommandOptionType::SubCommand)
        })
        .create_option(|option| {
            option
                .name("force")
                .description("immediately skip")
                .kind(CommandOptionType::SubCommand)
        })
}
