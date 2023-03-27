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
    model::idtypes::TrackId,
    model::idtypes::AlbumId,
    model::idtypes::PlaylistId,
    model::idtypes::ArtistId,
    prelude::PlayContextId,
    prelude::PlayableId,
};

use crate::CommandError;
use crate::values_from_options;
use crate::search_type_from_value;
use crate::str_from_value;
use crate::bool_from_value;
use crate::id_from_url;

pub async fn run(options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let values: Vec<&CommandDataOptionValue> = values_from_options(options)?;

    // there is a bug here if link is supplied but type isn't
    let search_term: &str = str_from_value(&values, 0, Some("track"))?;

    let search_type: Option<SearchType> = search_type_from_value(&values, 1, None).ok(); 

    let is_link: bool = bool_from_value(&values, 2, Some(false))?;

    // this is a mess
    // fix it later
    let result = match (is_link, search_type) {
        (true, Some(SearchType::Track)) => {
            let id = TrackId::from_id(id_from_url(search_term)?)?;
            spotify.start_uris_playback([PlayableId::Track(id)], None, None, None).await?;
            return Ok("Now playing".to_string());
        }
        (true, Some(_type)) => {
            let id = id_from_url(search_term)?;
            let context = match _type {
                SearchType::Album => PlayContextId::Album(AlbumId::from_id(id)?),
                SearchType::Playlist => PlayContextId::Playlist(PlaylistId::from_id(id)?),
                SearchType::Artist => PlayContextId::Artist(ArtistId::from_id(id)?),
                _ => return Err("Unsupported context type".into()),
            };
            spotify.start_context_playback(context, None, None, None).await?;
            return Ok("Now playing".to_string());
        }
        (true, None) => return Err("Must specify type when using link".into()),
        (false, Some(_type)) => spotify.search(search_term, _type, None, None, Some(1), None).await?,
        (false, None) => spotify.search(search_term, SearchType::Track, None, None, Some(1), None).await?,
    };

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
                .name("name")
                .description("Name of music search for")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("type")
                .description("Type of music")
                .kind(CommandOptionType::String)
                .add_string_choice("track", "track")
                .add_string_choice("album", "album")
                .add_string_choice("playlist", "playlist")
                .add_string_choice("artist", "artist")
                .required(false)
        })
        .create_option(|option| {
            option
                .name("link")
                .description("Whether the search term should be interpretted as a link")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })
}
