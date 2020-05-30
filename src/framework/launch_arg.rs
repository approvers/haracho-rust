use crate::framework::service::{Client};

pub struct OnMessageMatch<T: Client> {
    pub matches_to: String,
    pub message: T::TextMessage,
}

pub struct OnCommandCall<T: Client> {
    pub command_name: String,
    pub message: T::TextMessage,
}

pub trait LaunchArg: 'static {}

impl<T: Client> LaunchArg for OnMessageMatch<T> {}

impl<T: Client> LaunchArg for OnCommandCall<T> {}
