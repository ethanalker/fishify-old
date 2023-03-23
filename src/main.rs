mod commands;

use std::env;

use dotenv;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use log::{Record, Level, Metadata, SetLoggerError, LevelFilter, debug, error, info};
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};

use rspotify::{
    AuthCodeSpotify,
    Config,
    Credentials,
    OAuth,
    clients::OAuthClient,
    scopes,
    ClientError,
    model::enums::types::SearchType,
};

// Logger setup
struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn log_init() -> Result<(), SetLoggerError> {
        log::set_logger(&LOGGER)
                    .map(|()| log::set_max_level(LevelFilter::Info))
}

// Custom error type
#[derive(Debug)]
pub enum CommandError {
    SpotifyError(ClientError),
    SimpleError(String),
}

impl From<&str> for CommandError {
    fn from(string: &str) -> Self {
        CommandError::SimpleError(string.to_string())
    }
}

impl From<ClientError> for CommandError {
    fn from(error: ClientError) -> Self {
        CommandError::SpotifyError(error)
    }
}

impl From<CommandError> for String {
    fn from(command_error: CommandError) -> Self {
        match command_error {
            CommandError::SpotifyError(error) => format!("Error: {}", error.to_string()),
            CommandError::SimpleError(error) => format!("Error: {}", error),
        }
    }
}

trait TypeFromStr: Sized {
    fn parse(string: &str) -> Result<Self, CommandError>;
}

impl TypeFromStr for SearchType {
    fn parse(string: &str) -> Result<Self, CommandError> {
        match string {
            "track" => Ok(SearchType::Track),
            "album" => Ok(SearchType::Album),
            "playlist" => Ok(SearchType::Playlist),
            "artist" => Ok(SearchType::Artist),
            _ => return Err(CommandError::from("Unexpected search choice")),
        }
    }
}

pub fn values_from_options(options: &[CommandDataOption]) -> Result<Vec<&CommandDataOptionValue>, CommandError> {
    let mut values: Vec<&CommandDataOptionValue> = vec![];
    for option in options {
        let value = option.resolved.as_ref().ok_or("Failed to resolve value")?;
        values.push(value);
    }
    Ok(values)
}

pub fn search_type_from_value(
    values: &Vec<&CommandDataOptionValue>, 
    index: usize
) -> Result<SearchType, CommandError> {
    match values.get(index) {
        Some(CommandDataOptionValue::String(value)) => Ok(SearchType::parse(&value)?),
        None => Ok(SearchType::Track),
        _ => Err(CommandError::from("Invalid data option value")),
    }
}

pub fn str_from_value<'a>(
    values: &'a Vec<&'a CommandDataOptionValue>, 
    index: usize, 
    default: Option<&'a str>
) -> Result<&'a str, CommandError> {
    match (values.get(index), default) {
        (Some(CommandDataOptionValue::String(value)), _) => Ok(&value),
        (None, Some(value)) => Ok(value),
        _ => Err(CommandError::from("Invalid data option value")),
    }
}

pub fn int_from_value(
    values: &Vec<&CommandDataOptionValue>, 
    index: usize, 
    default: Option<i64>
) -> Result<i64, CommandError> {
    match (values.get(index), default) {
        (Some(CommandDataOptionValue::Integer(value)), _) => Ok(*value),
        (None, Some(value)) => Ok(value),
        _ => Err(CommandError::from("Invalid data option value")),
    }
}

struct Handler {
    spotify: AuthCodeSpotify,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!("Received command interaction: {:#?}", command);

            let content_result = match command.data.name.as_str() {
                "search" => commands::search::run(&command.data.options, &self.spotify).await,
                "play" => commands::play::run(&command.data.options, &self.spotify).await,
                "queue" => commands::queue::run(&command.data.options, &self.spotify).await,
                "set" => commands::set::run(&command.data.options, &self.spotify).await,
                "skip" => commands::skip::run(&command.data.options, &self.spotify).await,
                "list" => commands::list::run(&command.data.options, &self.spotify).await,
                "pause" => commands::pause::run(&command.data.options, &self.spotify).await,
                "resume" => commands::resume::run(&command.data.options, &self.spotify).await,
                "connect" => commands::connect::run(&command.data.options, &self.spotify).await,
                "status" => commands::status::run(&command.data.options, &self.spotify).await,
                "info" => commands::info::run(&command.data.options, &self.spotify).await,
                "devices" => commands::devices::run(&command.data.options, &self.spotify).await,
                _ => Err(CommandError::SimpleError("not implemented :(".to_string())),
            };

            let content = match content_result {
                Ok(msg) => msg,
                Err(why) => String::from(why),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                error!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::search::register(command))
                .create_application_command(|command| commands::play::register(command))
                .create_application_command(|command| commands::queue::register(command))
                .create_application_command(|command| commands::set::register(command))
                .create_application_command(|command| commands::skip::register(command))
                .create_application_command(|command| commands::list::register(command))
                .create_application_command(|command| commands::pause::register(command))
                .create_application_command(|command| commands::resume::register(command))
                .create_application_command(|command| commands::connect::register(command))
                .create_application_command(|command| commands::status::register(command))
                .create_application_command(|command| commands::info::register(command))
                .create_application_command(|command| commands::devices::register(command))
        })
        .await;

        info!("I now have the following guild slash commands: {:#?}", commands);

//        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
//            commands::wonderful_command::register(command)
//        })
//        .await;
//
//        info!("I created the following global slash command: {:#?}", guild_command);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    log_init();

    // Spotify auth
    let config = Config {
        token_refreshing: true,
        ..Default::default()
    };

    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-read-playback-state", "user-modify-playback-state")).unwrap();

    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);
    let url = spotify.get_authorize_url(false).unwrap();

    spotify
        .prompt_for_token(&url)
        .await
        .expect("auth failed");

    // Discord auth
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler { spotify: spotify, })
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
