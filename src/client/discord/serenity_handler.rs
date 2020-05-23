use crate::client::discord::{DiscordClient, DiscordContext};
use crate::framework::{Channel, ClientEvent, Message};
use serenity::client::EventHandler as SerenityEventHandler;
use serenity::model::gateway::Ready;
use serenity::{client::Context, model::channel::Message as SMessage};
use std::sync::{mpsc::Sender, Arc, Mutex};

pub(super) struct SerenityHandler {
    pub send_event_channel: Mutex<Sender<ClientEvent<DiscordClient>>>,
}

impl SerenityEventHandler for SerenityHandler {
    fn message(&self, _ctx: Context, msg: SMessage) {
        let channel = Channel {
            id: msg.channel_id.0,
        };

        let message = Message {
            id: msg.id.0,
            content: msg.content,
            channel,
        };

        let arg = ClientEvent::OnMessage(message);

        self.send_event_channel.lock().unwrap().send(arg).unwrap();
    }

    fn ready(&self, ctx: Context, _: Ready) {
        self.send_event_channel
            .lock()
            .unwrap()
            .send(ClientEvent::OnReady(DiscordContext::new(Arc::clone(
                &ctx.http,
            ))))
            .unwrap();
    }
}
