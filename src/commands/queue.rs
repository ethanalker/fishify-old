use log::warn;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};

use rspotify::{
    AuthCodeSpotify,
    model::enums::types::SearchType,
    clients::BaseClient,
    clients::OAuthClient,
    model::search::SearchResult,
    prelude::PlayableId,
};

use crate::CommandError;
use crate::values_from_options;
use crate::search_type_from_value;
use crate::str_from_value;

pub async fn run(options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let values: Vec<&CommandDataOptionValue> = values_from_options(options)?;

    let search_term: &str = str_from_value(&values, 0, Some("track"))?;

    let search_type: SearchType = search_type_from_value(&values, 1)?; 

    let result = spotify.search(search_term, search_type, None, None, Some(1), None).await?;

    match result {
        SearchResult::Tracks(mut page) => {
            let track = page.items.remove(0);
            spotify.add_item_to_queue(PlayableId::Track(track.id.ok_or("No track id")?), None).await?;
            Ok(format!("Successfully queued {} {}", <SearchType as Into<&'static str>>::into(search_type), track.name))
        }
        SearchResult::Albums(mut page) => {
            let album = page.items.remove(0);
            let tracks = spotify
                .album(album.id.ok_or("No track id")?).await?
                .tracks
                .items;

            for track in tracks {
                spotify.add_item_to_queue(PlayableId::Track(track.id.ok_or("No track id")?), None).await?;
            }
            Ok(format!("Successfully queued {} {}", <SearchType as Into<&'static str>>::into(search_type), album.name))
        }
        SearchResult::Playlists(mut page) => {
            let playlist = page.items.remove(0);
            let items = spotify
                .playlist(playlist.id, None, None).await?
                .tracks
                .items;

            for item in items {
                spotify.add_item_to_queue(
                    item.track.ok_or("No playable track")?.id().ok_or("No track id")?, None
                    ).await?;
            }
            Ok(format!("Successfully queued {} {}", <SearchType as Into<&'static str>>::into(search_type), playlist.name))
        }
        _ => Err(CommandError::from("Unexpected search result type")),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("queue")
        .description("Queue spotify music")
        .create_option(|option| {
            option
                .name("name")
                .description("name of music to add to queue")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("type")
                .description("track, album, or playlist")
                .kind(CommandOptionType::String)
                .add_string_choice("track", "track")
                .add_string_choice("album", "album")
                .add_string_choice("playlist", "playlist")
                .required(false)
        })
}
