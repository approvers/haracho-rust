use crate::framework::launch_arg;
use crate::framework::service::TextMessage;
use crate::framework::service::{Client, ClientEvent, LaunchTiming, Service, ServiceFactory};
use crate::framework::service_info::ServiceInfo;
use log::info;
use std::{sync::mpsc, thread};

type ServiceStore<T> = Vec<ServiceInfo<T>>;

pub struct Bot<T: Client> {
    client: T,
    prefix: &'static str,
    channel: mpsc::Receiver<ClientEvent<T>>,
    services: ServiceStore<T>,
    history: Vec<Box<dyn Service<T>>>,
}

impl<T: Client> Bot<T> {
    pub fn new(prefix: &'static str) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            client: T::new(tx).unwrap(),
            prefix,
            channel: rx,
            services: vec![],
            history: vec![],
        }
    }

    pub fn add_service<F>(&mut self)
    where
        F: ServiceFactory<T>,
    {
        self.services.push(F::info());
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
                    info!("{:?}", message);

                    let ctx = context.as_ref().expect("Event was called before ready");

                    Self::on_message(self.prefix, &self.services, message, ctx);
                }
            }
        }
    }

    fn on_message(
        prefix: &str,
        store: &ServiceStore<T>,
        m: T::TextMessage,
        controller: &T::Controller,
    ) {
        let content = m.content().trim();
        if content.is_empty() {
            return;
        }

        let parsed_command_name = {
            if content.split(" ").nth(0).unwrap().starts_with(prefix) {
                let c = content
                    .split(" ")
                    .nth(0)
                    .unwrap()
                    .trim()
                    .chars()
                    .skip(prefix.len())
                    .collect::<String>();
                Some(c)
            } else {
                None
            }
        };

        for info in store {
            let matches = info.initial_timings.iter().filter(|timing| match timing {
                LaunchTiming::OnCommandCall { command_name, .. } => {
                    parsed_command_name.is_some()
                        && parsed_command_name.as_ref().unwrap() == command_name
                }

                LaunchTiming::OnMessageMatch { target_content, .. } => content == *target_content,
            });

            for timing in matches {
                match timing {
                    LaunchTiming::OnCommandCall {
                        command_name,
                        generator,
                    } => {
                        let arg = launch_arg::OnCommandCall {
                            command_name: command_name.clone(),
                            message: m.clone(),
                        };

                        generator(arg).launch(controller).unwrap();
                    }

                    LaunchTiming::OnMessageMatch {
                        target_content,
                        generator,
                    } => {
                        let arg = launch_arg::OnMessageMatch {
                            matches_to: target_content.clone(),
                            message: m.clone(),
                        };

                        generator(arg).launch(controller).unwrap();
                    }
                }
            }
        }
    }
}
