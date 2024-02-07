use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn create_user(ctx: Context<'_>) -> Result<(), Error> {

}