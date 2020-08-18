use crate::framework::service::Client;
use crate::framework::service_info::ArgResult;

pub struct OnMessageMatch<T: Client> {
    pub matches_to: String,
    pub message: T::TextMessage,
    pub args: Option<Vec<ArgResult<T>>>,
}

pub struct OnCommandCall<T: Client> {
    pub command_name: String,
    pub message: T::TextMessage,
    pub args: Option<Vec<ArgResult<T>>>,
}

pub trait LaunchArg: 'static {}

impl<T: Client> LaunchArg for OnMessageMatch<T> {}

impl<T: Client> LaunchArg for OnCommandCall<T> {}
