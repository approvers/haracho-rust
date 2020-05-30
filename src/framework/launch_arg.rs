use crate::framework::service::Message;

pub struct OnMessageMatch {
    pub matches_to: String,
    pub message: Message,
}

pub struct OnCommandCall {
    pub command_name: String,
    pub message: Message,
}

pub trait LaunchArg: 'static {}

impl LaunchArg for OnMessageMatch {}

impl LaunchArg for OnCommandCall {}
