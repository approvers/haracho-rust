use crate::framework::launch_arg::{OnCommandCall, OnMessageMatch};
use crate::framework::launch_type::{LaunchOnCommandCall, LaunchOnMessageMatch};
use crate::framework::service::{Client, Controller, Service, ServiceFactory};
use crate::framework::service_info::{ServiceInfo, ServiceInfoBuilder};

pub struct PingServiceFactory;

impl<T: Client> ServiceFactory<T> for PingServiceFactory {
    fn info() -> ServiceInfo<T> {
        ServiceInfoBuilder::<T>::new()
            .name("PingService")
            .description("!ping コマンドに反応して pong と返します。Botのテスト用です。")
            .timing(LaunchOnCommandCall("ping".into()))
            .callback(|arg: OnCommandCall| PingService {
                channel: arg.message.channel.id,
            })
            .timing(LaunchOnMessageMatch("g!ping".into()))
            .callback(|arg: OnMessageMatch| PingService {
                channel: arg.message.channel.id,
            })
            .build()
    }
}

#[derive(Debug)]
pub struct PingService {
    channel: u64,
}

impl<T: Client> Service<T> for PingService {
    fn launch(&mut self, t: &T::Controller) -> Result<(), String> {
        t.send_message(self.channel, "pong!").map(|_| ())
    }
}
