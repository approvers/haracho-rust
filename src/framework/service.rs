use crate::framework::launch_arg::{OnCommandCall, OnMessageMatch};
use crate::framework::service_info::ServiceInfo;
use std::fmt::Debug;
use std::sync::mpsc;

pub trait Service<T: Client>: Debug {
    fn launch(&self, _: &T::Controller) -> Result<(), String>;
}

pub trait ServiceFactory<T: Client> {
    fn info() -> ServiceInfo<T>;
}

pub trait Controller<T: Client> {
    fn send_message(
        &self,
        channel: &T::TextChannel,
        content: &str,
    ) -> Result<T::TextMessage, String>;
}

#[derive(Debug)]
pub enum ClientError {
    InitializeClientError(String),
    StartingClientError(String),
}

pub trait Client: Sized + Debug + Send + 'static {
    type Controller: Controller<Self>;
    type Message: Message<Self>;
    type TextMessage: TextMessage<Self>;
    type Channel: Channel;
    type TextChannel: TextChannel;
    type VoiceChannel: VoiceChannel;

    fn new(_: mpsc::Sender<ClientEvent<Self>>) -> Result<Self, ClientError>;

    fn start(&mut self) -> Result<(), ClientError>;
}

pub enum LaunchTiming<T: Client> {
    OnMessageMatch {
        target_content: String,
        generator: Box<dyn Fn(OnMessageMatch<T>) -> Box<dyn Service<T>>>,
    },
    OnCommandCall {
        command_name: String,
        generator: Box<dyn Fn(OnCommandCall<T>) -> Box<dyn Service<T>>>,
    },
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ClientEvent<T: Client> {
    OnReady(T::Controller),
    OnMessage(T::TextMessage),
}

pub trait Message<T: Client>: Debug + Clone {}

pub trait TextMessage<T: Client>: Message<T> {
    fn content(&self) -> &str;
    fn channel(&self) -> T::TextChannel;
}

pub trait Channel: Debug + Copy + Clone {}

pub trait TextChannel: Channel {}

pub trait VoiceChannel: Channel {}
