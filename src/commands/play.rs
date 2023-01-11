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
    model::page::Page,
    model::idtypes::TrackId,
    prelude::PlayableId,
    model::PlayableItem,
    clients::BaseClient,
    clients::OAuthClient,
    model::search::SearchResult,
    prelude::PlayContextId,
};

use crate::CommandError;
use crate::ParseOptionValues;
use crate::ParseTypeFromStr;

pub async fn run(options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let values: Vec<&CommandDataOptionValue> = options.values();

    if let (CommandDataOptionValue::String(search_type), CommandDataOptionValue::String(search_term)) = 
        (values[0], values[1])  
    {
        let spotify_type = SearchType::parse(search_type)?;

        let result = spotify.search(search_term, spotify_type, None, None, Some(1), None).await?;
        match result {
            SearchResult::Tracks(page) => {
                let track = page.items[0].clone();
                let id = track.id.ok_or("No track id")?;

                spotify.start_uris_playback([PlayableId::Track(id)], None, None, None).await?;
                Ok(format!("Now playing {} by {}", track.name, track.artists[0].name))
            }
            SearchResult::Albums(page) => {
                let album = page.items[0].clone();
                let id = album.id.ok_or("No album id")?;
                
                spotify.start_context_playback(PlayContextId::Album(id), None, None, None).await?;
                Ok(format!("Now playing {} by {}", album.name, album.artists[0].name))
            }
            SearchResult::Playlists(page) => {
                let playlist = page.items[0].clone();
                let id = playlist.id;

                spotify.start_context_playback(PlayContextId::Playlist(id), None, None, None).await?;
                Ok(format!("Now playing {}", playlist.name))
            }
            SearchResult::Artists(page) => {
                let artist = page.items[0].clone();
                let id = artist.id;

                spotify.start_context_playback(PlayContextId::Artist(id), None, None, None).await?;
                Ok(format!("Now playing from {}", artist.name))
            }
            _ => Err(CommandError::from("Unexpected search result type")),
        }
    } else {
        Err(CommandError::from("Invalid search arguments"))
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
                .required(true)
        })
        .create_option(|option| {
            option
                .name("name")
                .description("name of music to add to queue")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
