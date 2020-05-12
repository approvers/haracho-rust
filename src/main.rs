use std::env;
use tokio::time::{self, Duration};
use serenity::{
    model::{
        channel::Message,
        gateway::Ready,
    },
    prelude::{
        Context,
        EventHandler,
        Client
    }
};


struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let content = msg.content;
        let author = msg.author.name;
        println!("{} > {}", content, author);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Make sure you set \"DISCORD_TOKEN\" environment variable.");
    
    let mut client = Client::new(&token, Handler).unwrap();
    client.start().unwrap();
}

