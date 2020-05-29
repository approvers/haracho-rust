use super::SerenityHandler;
use crate::framework::{Channel, Client, ClientError, ClientEvent, Controller, Message};

use log::error;
use serenity::http::Http;
use serenity::model::channel::Message as SerenityMessage;
use serenity::model::id::ChannelId;
use serenity::Client as SerenityClient;
use std::env;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::{mpsc, Arc, Mutex};

const TOKEN_ERROR_TEXT: &str = "Make sure you set DISCORD_TOKEN environment variable.";

pub struct DiscordClient {
    token: String,
    client: SerenityClient,
}

impl Client for DiscordClient {
    type Controller = DiscordController;

    fn new(channel: mpsc::Sender<ClientEvent<Self>>) -> Result<Self, ClientError> {
        let token = env::var("DISCORD_TOKEN")
            .map_err(|_| ClientError::InitializeClientError(TOKEN_ERROR_TEXT.into()))?;

        let handler = SerenityHandler::new(Mutex::new(channel));

        let client = SerenityClient::new(&token, handler)
            .map_err(|x| ClientError::InitializeClientError(x.to_string()))?;

        Ok(Self { token, client })
    }

    fn start(&mut self) -> Result<(), ClientError> {
        self.client
            .start()
            .map_err(|x| ClientError::StartingClientError(x.to_string()))
    }
}

impl Debug for DiscordClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "DiscordClient: Token: {}", &self.token)
    }
}

pub struct DiscordController {
    http: Arc<Http>,
}

impl DiscordController {
    pub fn new(a: Arc<Http>) -> Self {
        Self { http: a }
    }
}

impl Controller for DiscordController {
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
