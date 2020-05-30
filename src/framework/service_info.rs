use crate::framework::launch_arg::LaunchArg;
use crate::framework::launch_type::LaunchType;
use crate::framework::service::{Client, LaunchTiming, Service};
use log::info;

pub struct ServiceInfo<TClient: Client> {
    pub name: String,
    pub description: String,
    pub initial_timings: Vec<LaunchTiming<TClient>>,

    pub args_description: Option<Vec<String>>,
}

pub struct ServiceInfoBuilder<'a, TClient: Client> {
    name: Option<&'a str>,
    description: Option<&'a str>,
    initial_timings: Vec<LaunchTiming<TClient>>,

    args_descriptions: Option<&'a [&'a str]>,
}

pub struct PartiallyTimedServiceInfoBuilder<
    'a,
    TClient: Client,
    TLaunchArg: LaunchArg,
    TLaunchType: LaunchType<TClient, Arg = TLaunchArg>,
> {
    origin: ServiceInfoBuilder<'a, TClient>,
    timing: TLaunchType,
}

impl<'instance, TClient: Client> ServiceInfoBuilder<'instance, TClient> {
    pub fn new() -> Self {
        ServiceInfoBuilder {
            name: None,
            description: None,
            initial_timings: Vec::new(),

            args_descriptions: None,
        }
    }

    pub fn name<'a: 'instance>(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn description<'a: 'instance>(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn args_descriptions<'a: 'instance>(mut self, args_desc: &'a [&str]) -> Self {
        self.args_descriptions = Some(args_desc);
        self
    }

    pub fn timing<TLaunchType: LaunchType<TClient>>(
        self,
        timing: TLaunchType,
    ) -> PartiallyTimedServiceInfoBuilder<'instance, TClient, TLaunchType::Arg, TLaunchType> {
        PartiallyTimedServiceInfoBuilder {
            origin: self,
            timing,
        }
    }

    pub fn build(self) -> ServiceInfo<TClient> {
        if self.name.is_none() {
            panic!("Building service info failed: name is empty.");
        }

        if self.description.is_none() {
            panic!("Building service info failed: description is empty.");
        }

        if self.args_descriptions.is_none() {
            info!(
                "\"{}\" service's args descriptions is empty.",
                self.name.unwrap()
            );
        }

        if self.initial_timings.is_empty() {
            panic!(
                "\"{}\" service's timing is empty: There is no way to call this service!",
                self.name.unwrap()
            )
        }

        let args_desc = {
            if self.args_descriptions.is_some() {
                let args = self
                    .args_descriptions
                    .unwrap()
                    .iter()
                    .map(|x| String::from(*x))
                    .collect();
                Some(args)
            } else {
                None
            }
        };

        ServiceInfo {
            name: String::from(self.name.unwrap()),
            description: String::from(self.description.unwrap()),
            initial_timings: self.initial_timings,

            args_description: args_desc,
        }
    }
}

impl<
        'a,
        TClient: Client,
        TLaunchArg: LaunchArg,
        TLaunchType: LaunchType<TClient, Arg = TLaunchArg>,
    > PartiallyTimedServiceInfoBuilder<'a, TClient, TLaunchArg, TLaunchType>
{
    pub fn callback<TCallback, TService>(
        mut self,
        callback: TCallback,
    ) -> ServiceInfoBuilder<'a, TClient>
    where
        TService: Service<TClient> + 'static,
        TCallback: Fn(TLaunchArg) -> TService + 'static,
    {
        let boxed_callback = move |arg| Box::new(callback(arg)) as Box<dyn Service<TClient>>;

        let timing = self.timing.build(boxed_callback);
        self.origin.initial_timings.push(timing);

        self.origin
    }
}
