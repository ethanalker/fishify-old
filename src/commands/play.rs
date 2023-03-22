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
    prelude::PlayContextId,
    prelude::PlayableId,
};

use crate::CommandError;
use crate::values_from_options;
use crate::search_type_from_value;
use crate::str_from_value;

pub async fn run(options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let values: Vec<Option<&CommandDataOptionValue>> = values_from_options(options);

    let search_type: SearchType = search_type_from_value(values[0])?; 

    let search_term: &str = str_from_value(values[1])?;

    let result = spotify.search(search_term, search_type, None, None, Some(1), None).await?;

    match result {
        SearchResult::Tracks(mut page) => {
            let track = page.items.remove(0);
            let id = track.id.ok_or("No track id")?;

            spotify.start_uris_playback([PlayableId::Track(id)], None, None, None).await?;
            Ok(format!("Now playing {} by {}", track.name, track.artists[0].name))
        }
        SearchResult::Albums(mut page) => {
            let album = page.items.remove(0);
            let id = album.id.ok_or("No album id")?;
            
            spotify.start_context_playback(PlayContextId::Album(id), None, None, None).await?;
            Ok(format!("Now playing {} by {}", album.name, album.artists[0].name))
        }
        SearchResult::Playlists(mut page) => {
            let playlist = page.items.remove(0);
            let id = playlist.id;

            spotify.start_context_playback(PlayContextId::Playlist(id), None, None, None).await?;
            Ok(format!("Now playing {}", playlist.name))
        }
        SearchResult::Artists(mut page) => {
            let artist = page.items.remove(0);
            let id = artist.id;

            spotify.start_context_playback(PlayContextId::Artist(id), None, None, None).await?;
            Ok(format!("Now playing from {}", artist.name))
        }
        _ => Err(CommandError::from("Unexpected search result type")),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play")
        .description("Play spotify music")
        .create_option(|option| {
            option
                .name("type")
                .description("track, album, playlist, or artist")
                .kind(CommandOptionType::String)
                .add_string_choice("track", "track")
                .add_string_choice("album", "album")
                .add_string_choice("playlist", "playlist")
                .add_string_choice("artist", "artist")
                .required(false)
        })
        .create_option(|option| {
            option
                .name("name")
                .description("name of music to add to queue")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
