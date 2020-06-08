use crate::framework::launch_arg;
use crate::framework::service::{Client, LaunchTiming, Service};
use crate::framework::service_info::{ArgEntry, ArgType};
use std::marker::PhantomData;
use std::str::FromStr;

type NoArgBuilder<TClient, TLaunchType> =
    LaunchTimingBuilder<TLaunchType, Option<Box<dyn Fn() -> Box<dyn Service<TClient>>>>, Empty>;

type WithArgBuilder<TClient, TLaunchType: LaunchTypeWithArg<TClient>> = LaunchTimingBuilder<
    TLaunchType,
    Option<Box<dyn Fn(TLaunchType::Arg) -> Box<dyn Service<TClient>>>>,
    Option<Vec<ArgEntry>>,
>;

trait LaunchType<T: Client>: Sized + 'static {
    fn build(self, callback: impl Fn() -> Box<dyn Service<T>> + 'static) -> LaunchTiming<T>;
}

trait LaunchTypeWithArg<T: Client>: Sized + 'static {
    type Arg: launch_arg::LaunchArg;

    fn build(
        self,
        arg_list: Option<Vec<ArgEntry>>,
        callback: impl Fn(Self::Arg) -> Box<dyn Service<T>> + 'static,
    ) -> LaunchTiming<T>;
}

pub struct OnMessageMatch(String);

impl OnMessageMatch {
    pub fn new<TClient, S>(target_content: S) -> WithArgBuilder<TClient, Self>
    where
        TClient: Client,
        S: Into<String>,
    {
        WithArgBuilder::new(OnMessageMatch(target_content.into()))
    }
}

impl<T: Client> LaunchTypeWithArg<T> for OnMessageMatch {
    type Arg = launch_arg::OnMessageMatch<T>;

    fn build(
        self,
        arg_list: Option<Vec<ArgEntry>>,
        callback: impl Fn(Self::Arg) -> Box<dyn Service<T>> + 'static,
    ) -> LaunchTiming<T> {
        LaunchTiming::OnMessageMatch {
            target_content: self.0,
            args: arg_list,
            generator: Box::new(callback),
        }
    }
}

pub struct OnCommandCall(String);

impl OnCommandCall {
    pub fn new<TClient, S>(command_name: S) -> WithArgBuilder<TClient, Self>
    where
        TClient: Client,
        S: Into<String>,
    {
        WithArgBuilder::new(Self(command_name.into()))
    }
}

impl<T: Client> LaunchTypeWithArg<T> for OnCommandCall {
    type Arg = launch_arg::OnCommandCall<T>;

    fn build(
        self,
        arg_list: Option<Vec<ArgEntry>>,
        callback: impl Fn(Self::Arg) -> Box<dyn Service<T>> + 'static,
    ) -> LaunchTiming<T> {
        LaunchTiming::OnCommandCall {
            command_name: self.0,
            args: arg_list,
            generator: Box::new(callback),
        }
    }
}
@kawason0708

#[derive(Debug)]
pub struct BuildLaunchTimingError(pub String);

struct Empty;

struct LaunchTimingBuilder<TLaunchType, TCallback, TArgs> {
    internal: TLaunchType,
    callback: TCallback,
    args: TArgs,
}

impl<TClient, TLaunchType>
    LaunchTimingBuilder<TLaunchType, Option<Box<dyn Fn() -> Box<dyn Service<TClient>>>>, Empty>
where
    TClient: Client,
    TLaunchType: LaunchType<TClient>,
{
    pub fn new(target: TLaunchType) -> Self {
        Self {
            internal: target,
            callback: None,
            args: Empty,
        }
    }

    pub fn callback<TService, TCallback>(mut self, callback: TCallback) -> Self
    where
        TService: Service<TClient> + 'static,
        TCallback: Fn() -> TService + 'static,
    {
        let boxed_callback = || Box::new(callback()) as Box<dyn Service<TClient>>;

        self.callback = Some(Box::new(boxed_callback));
        self
    }

    pub fn build(self) -> Result<LaunchTiming<TClient>, BuildLaunchTimingError> {
        let callback = self.callback.ok_or(BuildLaunchTimingError(
            "callbackが渡されていません。".into(),
        ))?;
        let result = self.internal.build(callback);

        Ok(result)
    }
}

impl<TClient, TLaunchType>
    LaunchTimingBuilder<
        TLaunchType,
        Option<Box<dyn Fn(TLaunchType::Arg) -> Box<dyn Service<TClient>>>>,
        Option<Vec<ArgEntry>>,
    >
where
    TClient: Client,
    TLaunchType: LaunchTypeWithArg<TClient>,
{
    pub fn new(target: TLaunchType) -> Self {
        Self {
            internal: target,
            args: None,
            callback: None,
        }
    }

    fn push_arg(mut self, entry: ArgEntry) {
        if self.args.is_none() {
            self.args = Some(vec![]);
        }
        self.args.as_mut().unwrap().push(entry);
    }

    pub fn arg<A, B>(mut self, name: A, description: B, arg_type: ArgType) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        let entry = ArgEntry {
            name: name.into(),
            description: description.into(),
            arg_type,
            is_optional: false,
        };

        self.push_arg(entry);
        self
    }

    pub fn optional_arg<A, B>(mut self, name: A, description: B, arg_type: ArgType) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        let entry = ArgEntry {
            name: name.into(),
            description: description.into(),
            arg_type,
            is_optional: true,
        };

        self.push_arg(entry);
        self
    }

    pub fn callback<TService, TCallback>(mut self, callback: TCallback) -> Self
    where
        TService: Service<TClient> + 'static,
        TCallback: Fn(TLaunchType::Arg) -> TService + 'static,
    {
        let boxed_callback =
            |arg: TLaunchType::Arg| Box::new(callback(arg)) as Box<dyn Service<TClient>>;

        self.callback = Some(Box::new(boxed_callback));
        self
    }

    pub fn build(self) -> Result<LaunchTiming<TClient>, BuildLaunchTimingError> {
        if self.args.is_some() {
            let mut entered_optional = false;
            for entry in self.args.as_ref().unwrap() {
                if entry.is_optional {
                    entered_optional = true;
                } else {
                    if entered_optional {
                        return Err(BuildLaunchTimingError(
                            "Optionalな引数の後に、そうでない引数を置くことは出来ません".into(),
                        ));
                    }
                }
            }
        }

        let callback = self.callback.ok_or(BuildLaunchTimingError(
            "callbackが渡されていません。".into(),
        ))?;
        let result = self.internal.build(self.args, callback);

        Ok(result)
    }
}
