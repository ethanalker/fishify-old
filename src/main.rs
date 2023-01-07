mod commands;

use std::env;

use dotenv;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use log::{Record, Level, Metadata, SetLoggerError, LevelFilter, debug, error, info};

use rspotify::{
    AuthCodeSpotify,
    Config,
    Credentials,
    OAuth,
    clients::OAuthClient,
    scopes,
};

struct Handler {
    spotify: AuthCodeSpotify,
}

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


#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "search" => commands::search::run(&command.data.options, &self.spotify).await,
                // "play" => commands::play::run(&command.data.options, self.spotify.clone()).await,
                "queue" => commands::queue::run(&command.data.options, &self.spotify).await,
                _ => "not implemented :(".to_string(),
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
                // .create_application_command(|command| commands::play::register(command))
                .create_application_command(|command| commands::queue::register(command))
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

    let mut spotify = AuthCodeSpotify::with_config(creds, oauth, config);
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
        .event_handler(Handler { spotify: spotify, } )
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
