//
// handler.rs
//
// Message handler when a user sends a message in discord text channel.
//
use log::{info, warn};

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use super::ping_handler::PingHandler;

#[async_trait]
pub trait MessageListener {
    async fn on_message_received(&self, ctx: &Context, msg: &Message);
}

pub struct Handler {
    listeners: Vec<Box<dyn MessageListener + Sync>>,
}

impl Handler {
    fn new() -> Self {
        let mut handler = Self {
            listeners: Vec::new(),
        };

        handler.listeners.push(Box::new(PingHandler));

        handler
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        for listener in self.listeners.iter_mut() {
            *listener.on_message_received(&ctx, &msg).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}