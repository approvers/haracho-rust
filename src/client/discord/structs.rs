use crate::client::discord::DiscordClient;
use crate::framework;
use crate::framework::Client;

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

impl framework::Message<DiscordClient> for Message {}

impl framework::Message<DiscordClient> for TextMessage {}
impl From<serenity::model::channel::Message> for TextMessage {
    fn from(m: serenity::model::channel::Message) -> Self {
        Self {
            content: m.content,
            channel: TextChannel { id: m.channel_id.0 },
        }
    }
}

impl framework::TextMessage<DiscordClient> for TextMessage {
    fn content(&self) -> &str {
        &self.content
    }

    fn channel(&self) -> <DiscordClient as Client>::TextChannel {
        self.channel
    }
}

impl framework::Channel for Channel {}

impl framework::Channel for TextChannel {}

impl framework::TextChannel for TextChannel {}

impl framework::Channel for VoiceChannel {}

impl framework::VoiceChannel for VoiceChannel {}
