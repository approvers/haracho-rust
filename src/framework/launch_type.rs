use crate::framework::launch_arg::{LaunchArg, OnCommandCall, OnMessageMatch};
use crate::framework::service::{Client, LaunchTiming, Service};

pub struct LaunchOnMessageMatch(pub String);

pub struct LaunchOnCommandCall(pub String);

pub trait LaunchType<T: Client>: Sized + 'static {
    type Arg: LaunchArg;

    fn build<C>(self, callback: C) -> LaunchTiming<T>
    where
        C: Fn(Self::Arg) -> Box<dyn Service<T>> + 'static;
}

impl<T: Client> LaunchType<T> for LaunchOnMessageMatch {
    type Arg = OnMessageMatch;

    fn build<C>(self, callback: C) -> LaunchTiming<T>
    where
        C: Fn(Self::Arg) -> Box<dyn Service<T>> + 'static,
    {
        LaunchTiming::OnMessageMatch {
            target_content: self.0,
            generator: Box::new(callback),
        }
    }
}

impl<T: Client> LaunchType<T> for LaunchOnCommandCall {
    type Arg = OnCommandCall;

    fn build<C>(self, callback: C) -> LaunchTiming<T>
    where
        C: Fn(Self::Arg) -> Box<dyn Service<T>> + 'static,
    {
        LaunchTiming::OnCommandCall {
            command_name: self.0,
            generator: Box::new(callback),
        }
    }
}
