use crate::framework::ServiceInfo;
use std::fmt::Debug;
use std::sync::mpsc;

pub trait Service<T: Client>: Debug {
    fn launch(&mut self, _: &T::Context) -> Result<(), String>;
}

pub trait ServiceFactory<T: Client> {
    fn info() -> ServiceInfo;
    fn make(_: LaunchArg) -> Box<dyn Service<T>>;
}

pub trait Context {
    fn send_message(&self, channel_id: u64, content: &str) -> Result<Message, String>;
}

pub trait Client: Sized + Debug + Send + 'static {
    type Context: Context;

    fn new(_: mpsc::Sender<ClientEvent<Self>>) -> Self;

    fn start(&mut self);
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum LaunchTiming {
    OnMessageMatch(&'static str),
    OnCommandCall(&'static str),
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum LaunchArg {
    OnMessageMatch(Message),
    OnCommandCall(Message),
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ClientEvent<T: Client> {
    OnReady(T::Context),
    OnMessage(Message),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Message {
    pub content: String,
    pub id: u64,
    pub channel: Channel,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Channel {
    pub id: u64,
}
