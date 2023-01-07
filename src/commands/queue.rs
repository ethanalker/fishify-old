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
    model::PlayableItem,
    clients::BaseClient,
    clients::OAuthClient,
    model::search::SearchResult,
    prelude::PlayableId,
};

pub async fn run(options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> String {
    let mut values: Vec<&CommandDataOptionValue> = vec![];
    for option in options {
        values.push(option.resolved.as_ref().expect("expected option value"));
    }

    if let (CommandDataOptionValue::String(search_type), CommandDataOptionValue::String(search_term)) = 
        (values[0], values[1])  
    {
        let spotify_type = match search_type.as_str() {
            "album" => SearchType::Album,
            "playlist" => SearchType::Playlist,
            _ => SearchType::Track,
        };

        let future_result = spotify.search(search_term, &spotify_type, None, None, Some(1), None);
        match future_result.await {
            Ok(result) => {
                let name: String;
                let mut track_ids: Vec<TrackId> = vec![];
                match result {
                    SearchResult::Tracks(page) => {
                        let track = page.items[0].clone();
                        name = track.name;
                        track_ids.push(
                            track
                            .id
                            .expect("expected track id")
                            );
                    }
                    SearchResult::Albums(page) => {
                        let album = &page.items[0];
                        name = album.name.clone();
                        let tracks = spotify
                            .album(
                                album
                                .id
                                .as_ref()
                                .expect("expected album id")
                            ).await
                            .expect("expected album")
                            .tracks
                            .items;

                        for track in tracks {
                            let id = track.id.expect("expected track id");
                            track_ids.push(id);
                        }
                    }
                    SearchResult::Playlists(page) => {
                        let playlist = &page.items[0];
                        name = playlist.name.clone();
                        let items = spotify
                            .playlist(&playlist.id, None, None).await
                            .expect("expected full playlist")
                            .tracks
                            .items;

                        for item in items {
                            if let PlayableItem::Track(track) = item
                                .track
                                .expect("expected playable item")
                            {
                                track_ids.push(track.id.expect("expected track id"));
                            }
                        }
                    }
                    _ => return "Search Failed: Expected track, album, or playlist".to_string(),
                }
                for id in track_ids {
                    spotify.add_item_to_queue(&id, None).await.expect("queue failed");
                }
                format!("Successfully queued {} {}", search_type, name)
            }
            Err(why) => format!("Search Failed: {:?}", why),
        }
    } else {
        "Search Failed: Expected search term and type to be strings".to_string()
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
