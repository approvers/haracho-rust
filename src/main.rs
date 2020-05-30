#![allow(dead_code)]
mod client;
mod command;
mod framework;

use crate::command::ping::PingServiceFactory;
use client::discord::DiscordClient;

use crate::framework::bot::Bot;

const PREFIX: &'static str = "g!";
fn main() {
    let mut client = Bot::<DiscordClient>::new(PREFIX);

    client.add_service::<PingServiceFactory>();

    client.start();
}
