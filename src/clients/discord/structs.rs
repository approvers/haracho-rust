use crate::clients::discord::DiscordClient;
use crate::framework::service;

#[derive(Debug, Copy, Clone)]
pub struct Message;

#[derive(Debug, Clone)]
pub struct TextMessage {
    pub(super) content: String,
    pub(super) channel: TextChannel,
}

#[derive(Debug, Copy, Clone)]
pub struct Channel {
    pub(super) id: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct TextChannel {
    pub(super) id: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct VoiceChannel {
    pub(super) id: u64,
}

impl service::Message<DiscordClient> for Message {}

impl service::Message<DiscordClient> for TextMessage {}
impl From<serenity::model::channel::Message> for TextMessage {
    fn from(m: serenity::model::channel::Message) -> Self {
        Self {
            content: m.content,
            channel: TextChannel { id: m.channel_id.0 },
        }
    }
}

impl service::TextMessage<DiscordClient> for TextMessage {
    fn content(&self) -> &str {
        &self.content
    }

    fn channel(&self) -> <DiscordClient as service::Client>::TextChannel {
        self.channel
    }
}

impl service::Channel for Channel {}

impl service::Channel for TextChannel {}

impl service::TextChannel for TextChannel {}

impl service::Channel for VoiceChannel {}

impl service::VoiceChannel for VoiceChannel {}
