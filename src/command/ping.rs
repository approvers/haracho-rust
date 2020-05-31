use crate::framework::launch_arg::OnCommandCall;
use crate::framework::launch_type::LaunchOnCommandCall;
use crate::framework::service::{Client, Controller, Service, ServiceFactory};
use crate::framework::service_info::{ServiceInfo, ServiceInfoBuilder};
use crate::framework::TextMessage;

pub struct PingServiceFactory;

impl<T: Client> ServiceFactory<T> for PingServiceFactory {
    fn info() -> ServiceInfo<T> {
        ServiceInfoBuilder::<T>::new()
            .name("PingService")
            .description("!ping コマンドに反応して pong と返します。Botのテスト用です。")
            .timing(LaunchOnCommandCall("ping".into()))
            .callback(|arg: OnCommandCall<T>| PingService {
                channel: arg.message.channel(),
            })
            .build()
    }
}

#[derive(Debug)]
pub struct PingService<T: Client> {
    channel: T::TextChannel,
}

impl<T: Client> Service<T> for PingService<T> {
    fn launch(&mut self, t: &T::Controller) -> Result<(), String> {
        t.send_message(&self.channel, "pong!").map(|_| ())
    }
}
