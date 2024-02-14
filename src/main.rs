mod commands;
mod model;
mod handler;

use log::{info, warn, debug};
use poise::serenity_prelude as serenity;
use serenity::GatewayIntents;
use std::{
    sync::Arc,
    time::Duration
};

use commands::{ping, user};

struct Data{}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();

    // Poise framework configuration
    let options = poise::FrameworkOptions {
        commands: vec![
            ping::ping(),
            user::create_user(),
            user::status(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(">".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            ..Default::default()
        },
        pre_command: |ctx| {
            Box::pin(async move {
                debug!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                debug!("Executed commnad {}!", ctx.command().qualified_name);
            })
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 000000000 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                debug!(
                    "Got an event in event handler: {:?}",
                    event.snake_case_name()
                );
                Ok(())
            })
        },
        ..Default::default()
    };

    info!("Reading credentials and setting up connection...");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                info!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    use crate::model::config;
    let mut client =
        serenity::ClientBuilder::new(
            config::get_credential_data().discord_token.clone(),
            intents
        )
            .event_handler(handler::Handler)
            .framework(framework)
            .await
            .expect("Err creating client");

    if let Err(e) = client.start().await {
        warn!("Client error: {:?}", e);
    }
}
