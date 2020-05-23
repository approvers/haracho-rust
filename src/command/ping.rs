use crate::framework::{
    Client, Context, LaunchArg, LaunchTiming, Service, ServiceFactory, ServiceInfo,
    ServiceInfoBuilder,
};

pub struct PingServiceFactory;

impl<T: Client> ServiceFactory<T> for PingServiceFactory {
    fn info() -> ServiceInfo {
        ServiceInfoBuilder::new()
            .name("PingService")
            .description("!ping コマンドに反応して pong と返します。Botのテスト用です。")
            .timing(LaunchTiming::OnCommandCall("ping"))
            .build()
    }

    fn make(t: LaunchArg) -> Box<dyn Service<T>> {
        let a = match t {
            LaunchArg::OnCommandCall(message) => PingService {
                channel: message.channel.id,
            },

            _ => unreachable!(),
        };
        Box::new(a)
    }
}

#[derive(Debug)]
pub struct PingService {
    channel: u64,
}

impl<T: Client> Service<T> for PingService {
    fn launch(&mut self, t: &T::Context) -> Result<(), String> {
        t.send_message(self.channel, "pong!").map(|_| ())
    }
}
