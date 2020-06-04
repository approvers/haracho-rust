use crate::framework::launch_arg;
use crate::framework::launch_type;
use crate::framework::service::TextMessage;
use crate::framework::service::{Client, Controller, Service, ServiceFactory};
use crate::framework::service_info::{ArgType, ServiceInfo, ServiceInfoBuilder};
use regex::Regex;

pub struct ArgtestFactory;

impl<T: Client> ServiceFactory<T> for ArgtestFactory {
    fn info() -> ServiceInfo<T> {
        let regex = Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$").unwrap();

        let timing = launch_type::OnCommandCall::new("role")
            .arg("役職名", "作成する役職名", ArgType::String)
            .arg("付ける相手", "作成した役職を付与する相手", ArgType::User)
            .optional_arg(
                "カラーコード",
                "役職のカラーコードを指定できます",
                ArgType::Regex(regex),
            )
            .callback(|arg: launch_arg::OnCommandCall<T>| ArgTest {
                channel: arg.message.channel(),
            })
            .build()
            .unwrap();

        ServiceInfoBuilder::new()
            .name("role")
            .description("役職を自動作成して指定のユーザーに割り当てます")
            .timing(timing)
            .build()
            .unwrap()
    }
}

#[derive(Debug)]
pub struct ArgTest<T: Client> {
    channel: T::TextChannel,
}

impl<T: Client> Service<T> for ArgTest<T> {
    fn launch(&self, t: &T::Controller) -> Result<(), String> {
        t.send_message(&self.channel, "pong!").map(|_| ())
    }
}
