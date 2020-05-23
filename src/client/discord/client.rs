use super::SerenityHandler;
use crate::framework::{Channel, Client, ClientEvent, Context, Message};

use serenity::http::Http;
use serenity::model::channel::Message as SerenityMessage;
use serenity::model::id::ChannelId;
use serenity::Client as SerenityClient;
use std::env;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::{mpsc, Arc, Mutex};

pub struct DiscordClient {
    token: String,
    client: SerenityClient,
}

impl Client for DiscordClient {
    type Context = DiscordContext;

    fn new(channel: mpsc::Sender<ClientEvent<Self>>) -> Self {
        let token = env::var("DISCORD_TOKEN")
            .expect("Make sure you set \"DISCORD_TOKEN\" environment variable.");

        let handler = SerenityHandler {
            send_event_channel: Mutex::new(channel),
        };

        let client = SerenityClient::new(&token, handler).unwrap();
        Self { token, client }
    }

    fn start(&mut self) {
        self.client.start().unwrap();
    }
}

impl Debug for DiscordClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "DiscordClient: Token: {}", &self.token)
    }
}

pub struct DiscordContext {
    http: Arc<Http>,
}

impl DiscordContext {
    pub fn new(a: Arc<Http>) -> Self {
        Self { http: a }
    }
}

impl Context for DiscordContext {
    fn send_message(&self, channel_id: u64, content: &str) -> Result<Message, String> {
        ChannelId(channel_id)
            .say(&self.http, content)
            .map(|m| m.into())
            .map_err(|e| e.to_string())
    }
}

impl From<SerenityMessage> for Message {
    fn from(m: SerenityMessage) -> Self {
        Message {
            content: m.content,
            id: m.id.0,
            channel: Channel { id: m.channel_id.0 },
        }
    }
}
