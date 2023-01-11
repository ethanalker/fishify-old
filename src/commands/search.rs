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
use crate::ParseOptionValues;
use crate::ParseTypeFromStr;

pub async fn run(options: &[CommandDataOption], spotify: &AuthCodeSpotify) -> Result<String, CommandError> {
    let values: Vec<&CommandDataOptionValue> = options.values()?;

    if let (CommandDataOptionValue::String(search_type), CommandDataOptionValue::String(search_term)) = 
        (values[0], values[1])  
    {
        let spotify_type = SearchType::parse(&search_type)?;

        let result = spotify.search(&search_term, spotify_type, None, None, Some(5), None).await?;
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
    } else {
        Err(CommandError::from("Invalid search arguments"))
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
                .add_string_choice("artist", "artist")
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
