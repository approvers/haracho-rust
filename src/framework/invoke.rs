use super::{Client, LaunchArg, LaunchTiming, Message, Service, ServiceFactory, ServiceInfo};

use crate::framework::{ClientEvent, StatefulServiceFactory};

use std::{collections::HashMap, sync::mpsc, thread};

use log::info;

type ServiceStore<T> = HashMap<ServiceInfo, Box<dyn Fn(LaunchArg) -> Box<dyn Service<T>>>>;

pub struct Bot<T: Client> {
    client: T,
    prefix: &'static str,
    channel: mpsc::Receiver<ClientEvent<T>>,
    stateless_services: ServiceStore<T>,
}

impl<T: Client> Bot<T> {
    pub fn new(prefix: &'static str) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            client: T::new(tx),
            prefix,
            channel: rx,
            stateless_services: HashMap::new(),
        }
    }

    pub fn add_service<F>(&mut self)
    where
        F: ServiceFactory<T>,
    {
        let info = F::info();
        let generator = |x: LaunchArg<'_>| F::make(x);

        self.stateless_services.insert(info, Box::new(generator));
    }

    pub fn add_configured_service<F>(&mut self)
    where
        F: StatefulServiceFactory<T>,
    {
        unimplemented!()
    }

    pub fn start(self) {
        let mut client = self.client;
        thread::spawn(move || client.start());

        let mut context = None;
        let channel = self.channel;
        for event in channel {
            match event {
                ClientEvent::OnReady(ctx) => {
                    info!("Bot is ready!");
                    context = Some(ctx);
                }

                ClientEvent::OnMessage(message) => {
                    info!("ClientEvent::OnMessage fired: {:?}", message);

                    let ctx = context.as_ref().expect("Event was called before ready");

                    Self::on_message(self.prefix, &self.stateless_services, message, ctx);
                }
            }
        }
    }

    fn on_message(prefix: &str, store: &ServiceStore<T>, m: Message, ctx: &T::Context) {
        let content = m.content.trim();
        if content.is_empty() {
            return;
        }

        let command_name = {
            if m.content.starts_with(prefix) {
                let c = content
                    .split(" ")
                    .nth(0)
                    .unwrap()
                    .chars()
                    .skip(prefix.len())
                    .collect::<String>();
                Some(c)
            } else {
                None
            }
        };

        store.iter().for_each(|(info, generator)| {
            info.initial_timings
                .iter()
                .filter(|timing| match timing {
                    LaunchTiming::OnCommandCall(target_command_name) => {
                        command_name.is_some()
                            && command_name.as_ref().unwrap() == *target_command_name
                    }

                    LaunchTiming::OnMessageMatch(target_message_content) => {
                        content == *target_message_content
                    }
                })
                .map(|timing| match timing {
                    LaunchTiming::OnCommandCall(command_name) => LaunchArg::OnCommandCall {
                        command_name,
                        message: m.clone(),
                    },
                    LaunchTiming::OnMessageMatch(matched_message) => LaunchArg::OnMessageMatch {
                        matches_to: matched_message,
                        message: m.clone(),
                    },
                })
                .for_each(|arg| {
                    generator(arg)
                        .launch(ctx)
                        .expect(&format!("Failed at launching {}", &info.name));
                })
        })
    }
}
