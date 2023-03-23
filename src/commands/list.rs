use log::warn;
use log::info;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
};

use rspotify::{
    AuthCodeSpotify,
    clients::OAuthClient,
    model::PlayableItem,
};

use crate::CommandError;

pub async fn run(_options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let queue = spotify.current_user_queue().await?;

    let mut result: String = match queue.currently_playing {
        Some(PlayableItem::Track(playable)) => format!(
            "> Currently playing {} by {} \n> Queue: \n", 
            playable.name, 
            playable.artists[0].name,
        ),
        Some(PlayableItem::Episode(playable)) => format!(
            "> Currently playing {} \n> Queue: \n", 
            playable.name, 
        ),
        None => "> Queue: \n".to_string(),
    };

    for item in queue.queue {
        match item {
            PlayableItem::Track(track) => {
                let artist = &track.artists[0].name;
                let name = &track.name;
                result.push_str(format!("> {artist} \u{2014} {name}\n").as_ref());
            }
            PlayableItem::Episode(episode) => {
                let name = &episode.name;
                result.push_str(format!("> {name}\n").as_ref());
            }
        }
    }

    Ok(result)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("list")
        .description("List current queue")
}
