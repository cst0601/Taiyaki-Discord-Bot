//
// ping_handler.rs
//
// Does not do anything meaningful, respond ping message with a pong!
//
use log::{info, warn};

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use super::handler::MessageListener;

pub struct PingHandler;

#[async_trait]
impl MessageListener for PingHandler {

    async fn on_message_received(&self, ctx: Context, msg: Message) {
        if msg.content == "ping" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "pong!").await {
                warn!("Error sending message: {:?}", e);
            }
        }
    }

}