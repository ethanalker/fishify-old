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
    clients::BaseClient,
    model::search::SearchResult,
};

pub async fn run(options: &[CommandDataOption], spotify: AuthCodeSpotify) -> String {
    let search_type_option = options 
        .get(0)
        .expect("Expected music type")
        .resolved
        .as_ref()
        .expect("Expected string object");
    
    let search_term_option = options
        .get(1)
        .expect("Expected search term")
        .resolved
        .as_ref()
        .expect("Expected string object");

    if let (CommandDataOptionValue::String(search_type), CommandDataOptionValue::String(search_term)) = 
        (search_type_option, search_term_option)  
    {
        let spotify_type = match search_type.as_str() {
            "album" => SearchType::Album,
            "playlist" => SearchType::Playlist,
            _ => SearchType::Track,
        };

        let future_result = spotify.search(search_term, &spotify_type, None, None, Some(5), None);
        match future_result.await {
            Ok(result) => {
                let mut result_string: String = "Search Results: \n".to_string();
                match result {
                    SearchResult::Tracks(page) => {
                        let items = page.items;

                        for item in items {
                            result_string.push_str(format!("{} - {} \n", item.artists[0].name, item.name).as_str());
                        }
                    }
                    SearchResult::Albums(page) => {
                        let items = page.items;

                        for item in items {
                            result_string.push_str(format!("{} - {} \n", item.artists[0].name, item.name).as_str());
                        }
                    }
                    SearchResult::Playlists(page) => {
                        let items = page.items;

                        for item in items {
                            result_string.push_str(format!("{} \n", item.name).as_str());
                        }
                    }
                    _ => result_string = "shouldn't be possible".to_string(),
                }
                result_string
            },
            Err(why) => format!("Search Failed: {:?}", why),
        }
    } else {
        "shouldn't be possible".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("search")
        .description("Search for tracks, albums, and playlists through spotify")
        .create_option(|option| {
            option
                .name("music_type")
                .description("track, album, or playlist")
                .kind(CommandOptionType::String)
                .add_string_choice("track", "track")
                .add_string_choice("album", "album")
                .add_string_choice("playlist", "playlist")
                .required(true)
        })
        .create_option(|option| {
            option
                .name("search_term")
                .description("music to search for")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
