use crate::framework::{
    Client, LaunchArg, LaunchTiming, NoConfig, NoState, ServiceInfo, ServiceInfoBuilder, State,
    StateUnion, StatefulService, StatefulServiceFactory,
};
use std::collections::HashMap;

pub struct CustomCommands {
    commands: HashMap<String, String>,
}

impl State for CustomCommands {
    fn launch_timings(&self) -> Option<Vec<LaunchTiming>> {
        unimplemented!()
    }
}

pub struct CustomCommandServiceFactory;

impl<T: Client> StatefulServiceFactory<T> for CustomCommandServiceFactory {
    type State = NoState;
    type Config = NoConfig;

    fn info() -> ServiceInfo {
        ServiceInfoBuilder::default()
            .name("CustomCommandService")
            .description("ポニョみたいに使えるやつ")
            .timing(LaunchTiming::OnCommandCall("commandadd"))
            .build()
            .unwrap()
    }

    fn make(
        arg: LaunchArg,
        state: &StateUnion<Self::State, Self::Config>,
    ) -> Box<dyn StatefulService<T, State = Self::State, Config = Self::Config>> {
        unimplemented!()
    }
}

