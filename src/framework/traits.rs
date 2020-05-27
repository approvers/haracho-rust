use crate::framework::ServiceInfo;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::mpsc;

pub trait ServiceFactory<T: Client> {
    fn info() -> ServiceInfo;
    fn make(_: LaunchArg) -> Box<dyn Service<T>>;
}

pub trait Service<T: Client>: Debug {
    fn launch(&self, _: &T::Context) -> Result<(), String>;
}

pub trait StatefulServiceFactory<T: Client> {
    //Botの起動中だけ維持される。再起動されたときには消える。
    type State: State;
    //Botが再起動しても保存される。
    type Config: Config;

    fn info() -> ServiceInfo;
    fn make(
        _: LaunchArg,
        _: &StateUnion<Self::State, Self::Config>,
    ) -> Box<dyn StatefulService<T, State = Self::State, Config = Self::Config>>;
}

pub trait StatefulService<T: Client> {
    type State: State;
    type Config: Config;

    // 新しいStateを返す。 更新しなくていいなら、NoneでOK
    fn launch(
        &self,
        _: &T::Context,
    ) -> Result<Option<StateUnion<Self::State, Self::Config>>, String>;
}

pub struct StateUnion<TState: State, TConfig: Config> {
    pub state: TState,
    pub config: TConfig,
}

pub trait State {
    // StateによってBotの起動タイミングを変更したい場合、これで返せばOK
    fn launch_timings(&self) -> Option<Vec<LaunchTiming>>;
}

pub struct NoState;

impl State for NoState {
    fn launch_timings(&self) -> Option<Vec<LaunchTiming>> {
        None
    }
}

pub trait Config: Serialize + Deserialize<'static> {
    fn name() -> &'static str;

    // ConfigによってBotの起動タイミングを変更したい場合、これで返せばOK
    fn launch_timings(&self) -> Option<Vec<LaunchTiming>>;
}

#[derive(Serialize, Deserialize)]
pub struct NoConfig;

impl Config for NoConfig {
    fn name() -> &'static str {
        "__no_config__"
    }

    fn launch_timings(&self) -> Option<Vec<LaunchTiming>> {
        None
    }
}

pub trait Context {
    fn send_message(&self, channel_id: u64, content: &str) -> Result<Message, String>;
}

pub trait Client: Sized + Debug + Send + 'static {
    type Context: Context;

    fn new(_: mpsc::Sender<ClientEvent<Self>>) -> Self;

    fn start(&mut self);
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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
    OnReady(T::Context),
    OnMessage(Message),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Message {
    pub content: String,
    pub id: u64,
    pub channel: Channel,
    pub author: User,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Channel {
    pub id: u64,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct User {
    pub id: u64,
}
