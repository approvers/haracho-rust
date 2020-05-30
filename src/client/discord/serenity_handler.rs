use crate::client::discord::{DiscordClient, DiscordController};
use crate::framework::service::ClientEvent;
use serenity::client::EventHandler as SerenityEventHandler;
use serenity::model::gateway::Ready;
use serenity::{client::Context, model::channel::Message as SMessage};
use std::sync::{mpsc::Sender, Arc, Mutex};

pub(super) struct SerenityHandler {
    pub send_event_channel: Mutex<Sender<ClientEvent<DiscordClient>>>,
}

impl SerenityHandler {
    pub fn new(e: Mutex<Sender<ClientEvent<DiscordClient>>>) -> Self {
        Self {
            send_event_channel: e,
        }
    }

    fn send_event(&self, ev: ClientEvent<DiscordClient>) {
        self.send_event_channel.lock().unwrap().send(ev).unwrap();
    }
}

impl SerenityEventHandler for SerenityHandler {
    fn message(&self, _ctx: Context, msg: SMessage) {
        let event = ClientEvent::OnMessage(msg.into());

        self.send_event(event)
    }

    fn ready(&self, ctx: Context, _: Ready) {
        let http = Arc::clone(&ctx.http);
        let controller = DiscordController::new(http);
        let event = ClientEvent::OnReady(controller);

        self.send_event(event);
    }
}
