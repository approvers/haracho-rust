use crate::framework::ServiceInfo;
use readonly::*;
use std::fmt::Debug;
use std::sync::mpsc;

pub trait Service<T: Client>: Debug {
    fn launch(&mut self, _: &T::Controller) -> Result<(), String>;
}

pub trait ServiceFactory<T: Client> {
    fn info() -> ServiceInfo;
    fn make(_: LaunchArg) -> Box<dyn Service<T>>;
}

pub trait Controller {
    fn send_message(&self, channel_id: u64, content: &str) -> Result<Message, String>;
}

pub trait Client: Sized + Debug + Send + 'static {
    type Controller: Controller;

    fn new(_: mpsc::Sender<ClientEvent<Self>>) -> Self;

    fn start(&mut self);
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum LaunchTiming {
    OnMessageMatch(&'static str),
    OnCommandCall(&'static str),
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum LaunchArg<'a> {
    OnMessageMatch {
        matches_to: &'a str,
        message: Message,
    },
    OnCommandCall {
        command_name: &'a str,
        message: Message,
    },
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ClientEvent<T: Client> {
    OnReady(T::Controller),
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
