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
    model::idtypes::TrackId,
    prelude::PlayableId,
    model::PlayableItem,
    clients::BaseClient,
    clients::OAuthClient,
    model::search::SearchResult,
};

use crate::CommandError;
use crate::ParseOptionValues;
use crate::ParseTypeFromStr;

pub async fn run(options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let values: Vec<&CommandDataOptionValue> = options.values()?;

    if let (CommandDataOptionValue::String(search_type), CommandDataOptionValue::String(search_term)) = 
        (values[0], values[1])  
    {
        let spotify_type = SearchType::parse(search_type)?;

        let result = spotify.search(search_term, spotify_type, None, None, Some(1), None).await?;
        let name: String;
        let mut track_ids: Vec<TrackId> = vec![];
        match result {
            SearchResult::Tracks(page) => {
                let track = page.items[0].clone();
                name = track.name;
                track_ids.push(track.id.ok_or("No track id")?);
            }
            SearchResult::Albums(page) => {
                let album = page.items[0].clone();
                name = album.name;
                let tracks = spotify
                    .album(album.id.ok_or("No track id")?).await?
                    .tracks
                    .items;

                for track in tracks {
                    let id = track.id.ok_or("No track id")?;
                    track_ids.push(id);
                }
            }
            SearchResult::Playlists(page) => {
                let playlist = page.items[0].clone();
                name = playlist.name;
                let items = spotify
                    .playlist(playlist.id, None, None).await?
                    .tracks
                    .items;

                for item in items {
                    if let PlayableItem::Track(track) = item.track.ok_or("Playlist item does not have track")?
                    {
                        track_ids.push(track.id.ok_or("No track id")?);
                    }
                }
            }
            _ => return Err(CommandError::from("Unexpected search result type")),
        }
        for id in track_ids {
            spotify.add_item_to_queue(PlayableId::Track(id), None).await?;
        }
        Ok(format!("Successfully queued {} {}", search_type, name))
    } else {
        Err(CommandError::from("Invalid search arguments"))
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("queue")
        .description("Queue spotify music")
        .create_option(|option| {
            option
                .name("type")
                .description("track, album, or playlist")
                .kind(CommandOptionType::String)
                .add_string_choice("track", "track")
                .add_string_choice("album", "album")
                .add_string_choice("playlist", "playlist")
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
