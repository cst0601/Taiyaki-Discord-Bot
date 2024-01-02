//
// service.rs
//
use serenity::framework::standard::macros::{command, group, hook};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;


#[group]
#[commands(about)]
struct General;

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "A simple test bot").await?;

    Ok(())
}

#[hook]
async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    msg.reply(
        &ctx.http,
        format!("Error: Command {} unknown.", unknown_command_name)
    ).await;
}

pub fn create_services() -> StandardFramework {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$"))
        .unrecognised_command(unknown_command)
        .group(&GENERAL_GROUP);

    framework
}
