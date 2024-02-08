use crate::{Context, Error};

/// Returns Pong! to check if the bot is still alive.
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong!").await?;
    Ok(())
}
