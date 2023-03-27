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
    model::search::SearchResult,
};

use crate::CommandError;
use crate::values_from_options;
use crate::search_type_from_value;
use crate::str_from_value;

pub async fn run(options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let values: Vec<&CommandDataOptionValue> = values_from_options(options)?;

    let search_term: &str = str_from_value(&values, 0, Some("track"))?;

    let search_type: SearchType = search_type_from_value(&values, 1, Some(SearchType::Track))?; 

    let result = spotify.search(search_term, search_type, None, None, Some(5), None).await?;

    let mut result_string: String = format!("Search results for '{}'\n", search_term);

    match result {
        SearchResult::Tracks(page) => {
            let items = page.items;

            for item in items {
                result_string.push_str(format!("{} \u{2014} {} \n", item.artists[0].name, item.name).as_str());
            }
            Ok(result_string)
        }
        SearchResult::Albums(page) => {
            let items = page.items;

            for item in items {
                result_string.push_str(format!("{} \u{2014} {} \n", item.artists[0].name, item.name).as_str());
            }
            Ok(result_string)
        }
        SearchResult::Playlists(page) => {
            let items = page.items;

            for item in items {
                result_string.push_str(format!("{} \n", item.name).as_str());
            }
            Ok(result_string)
        }
        SearchResult::Artists(page) => {
            let items = page.items;

            for item in items {
                result_string.push_str(format!("{} \n", item.name).as_str());
            }
            Ok(result_string)
        }
        _ => Err(CommandError::from("Unexpected search result type")),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("search")
        .description("Search for tracks, albums, and playlists through spotify")
        .create_option(|option| {
            option
                .name("name")
                .description("name of music to search for")
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
                .add_string_choice("artist", "artist")
                .required(false)
        })
}
