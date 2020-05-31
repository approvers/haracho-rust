#![allow(dead_code)]
mod client;
mod command;
mod framework;

use crate::command::ping::PingServiceFactory;
use client::discord::DiscordClient;

use crate::framework::bot::Bot;

use log::Level;

const PREFIX: &'static str = "g!";
fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let mut client = Bot::<DiscordClient>::new(PREFIX);

    client.add_service::<PingServiceFactory>();

    client.start();
}
