use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};

pub fn run(options: &[CommandDataOption]) -> String {
    let music_type_option = options 
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

    if let (CommandDataOptionValue::String(music_type), CommandDataOptionValue::String(search_term)) = 
        (music_type_option, search_term_option)  
    {
        format!("Search {} for {}", music_type, search_term)
    } else {
        "Invalid search type".to_string()
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
