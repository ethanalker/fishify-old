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

pub async fn run(options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> String {
    let mut values: Vec<&CommandDataOptionValue> = vec![];
    for option in options {
        values.push(option.resolved.as_ref().expect("Expected option value"));
    }

    if let (CommandDataOptionValue::String(search_type), CommandDataOptionValue::String(search_term)) = 
        (values[0], values[1])  
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
                    _ => result_string = "Search Failed: Expected track, album, or playlist".to_string(),
                }
                result_string
            },
            Err(why) => format!("Search Failed: {:?}", why),
        }
    } else {
        "Search Failed: Expected search term and type to be strings".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("search")
        .description("Search for tracks, albums, and playlists through spotify")
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
                .description("name of music to search for")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
