/// handlers.rs
///
/// Serenity handler. If receives message, add one taiyaki to user if exist in
/// database.
use poise::serenity_prelude as serenity;
use serenity::{
    EventHandler,
    async_trait,
    model::channel::Message,
    client::Context,
};
use crate::model::user;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler{

    async fn message(&self, _ctx: Context, msg: Message) {
        user::add_taiyaki_by_id(msg.author.id.get());
    }
}