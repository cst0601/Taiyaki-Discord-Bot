use poise::serenity_prelude as serenity;
use poise::CreateReply;
use serenity::{
    MessageBuilder,
    CreateEmbed,
    model::Timestamp
};
use log::warn;
use crate::{Context, Error};
use crate::model::user;

#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn create_user(ctx: Context<'_>) -> Result<(), Error> {
    user::User::new(ctx.author().id.get(), ctx.author().name.clone());
    let response = MessageBuilder::new()
        .push("Created user for ")
        .mention(ctx.author())
        .build();

    let _ = ctx.reply(response).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    match user::get_user_by_id(ctx.author().id.get()) {
        Ok(Some(user)) => { ctx.send(create_status_response(user)).await?; },
        Ok(None) => { ctx.reply("You are not registered as a user.").await?; },
        Err(error) => { warn!("Error connecting to db: {}", error); }
    };

    Ok(())
}

fn create_status_response(user: user::User) -> CreateReply {
    let embed = CreateEmbed::new()
        .title("Status")
        .colour(serenity::model::Colour::from_rgb(255, 183, 197))
        .fields(vec![
            ("Level", user.level.to_string(), true),
            ("Taiyaki", user.taiyaki_count.to_string(), true),
        ])
        .timestamp(Timestamp::now());
    poise::CreateReply::default()
        .embed(embed)
        .reply(true)
}