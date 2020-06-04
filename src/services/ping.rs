use crate::framework::service::TextMessage;
use crate::framework::service::{Client, Controller, Service, ServiceFactory};
use crate::framework::service_info::{ServiceInfo, ServiceInfoBuilder};
use crate::framework::{launch_arg, launch_type};

pub struct PingServiceFactory;

impl<T: Client> ServiceFactory<T> for PingServiceFactory {
    fn info() -> ServiceInfo<T> {
        let timing = launch_type::OnCommandCall::new("ping")
            .callback(|arg: launch_arg::OnCommandCall<T>| PingService {
                channel: arg.message.channel(),
            })
            .build()
            .unwrap();

        ServiceInfoBuilder::<T>::new()
            .name("PingService")
            .description("!ping コマンドに反応して pong と返します。Botのテスト用です。")
            .timing(timing)
            .build()
            .unwrap()
    }
}

#[derive(Debug)]
pub struct PingService<T: Client> {
    channel: T::TextChannel,
}

impl<T: Client> Service<T> for PingService<T> {
    fn launch(&self, t: &T::Controller) -> Result<(), String> {
        t.send_message(&self.channel, "pong!").map(|_| ())
    }
}
