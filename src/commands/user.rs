use poise::serenity_prelude::MessageBuilder;
use crate::{Context, Error};
use crate::model::user;

#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn create_user(ctx: Context<'_>) -> Result<(), Error> {
    user::User::new(ctx.author().id.get(), ctx.author().name.clone());
    let response = MessageBuilder::new()
        .push("Created user for ")
        .mention(ctx.author())
        .build();
    let _ = ctx.say(response).await;
    Ok(())
}
