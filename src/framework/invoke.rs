use super::{Client, LaunchArg, LaunchTiming, Message, Service, ServiceFactory, ServiceInfo};

use crate::framework::ClientEvent;

use std::{collections::HashMap, sync::mpsc, thread};

use log::error;

type ServiceStore<T> = HashMap<ServiceInfo, Box<dyn Fn(LaunchArg) -> Box<dyn Service<T>>>>;

pub struct Bot<T: Client> {
    client: T,
    prefix: &'static str,
    channel: mpsc::Receiver<ClientEvent<T>>,
    services: ServiceStore<T>,
}

impl<T: Client> Bot<T> {
    pub fn new(prefix: &'static str) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            client: T::new(tx).unwrap(),
            prefix,
            channel: rx,
            services: HashMap::new(),
        }
    }

    pub fn add_service<F>(&mut self)
    where
        F: ServiceFactory<T>,
    {
        let info = F::info();
        let generator = |x: LaunchArg<'_>| F::make(x);

        self.services.insert(info, Box::new(generator));
    }

    pub fn start(self) {
        let mut client = self.client;
        thread::spawn(move || client.start());

        let mut context = None;
        let channel = self.channel;
        for event in channel {
            match event {
                ClientEvent::OnReady(ctx) => {
                    println!("Bot is ready!");
                    context = Some(ctx);
                }

                ClientEvent::OnMessage(message) => {
                    println!("{:?}", message);

                    let ctx = context.as_ref().expect("Event was called before ready");

                    Self::on_message(self.prefix, &self.services, message, ctx);
                }
            }
        }
    }

    fn on_message(prefix: &str, store: &ServiceStore<T>, m: Message, ctx: &T::Controller) {
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
                    .trim()
                    .chars()
                    .skip(prefix.len())
                    .collect::<String>();
                Some(c)
            } else {
                None
            }
        };

        for (info, generator) in store {
            info.initial_timings
                .iter()
                .filter(|timing| {
                    use LaunchTiming::*;
                    match timing {
                        OnCommandCall(a) => {
                            command_name.is_some() && command_name.as_ref().unwrap() == *a
                        }

                        OnMessageMatch(a) => content == *a,
                    }
                })
                .map(|timing| match timing {
                    LaunchTiming::OnCommandCall(name) => LaunchArg::OnCommandCall {
                        command_name: name,
                        message: m.clone(),
                    },
                    LaunchTiming::OnMessageMatch(content) => LaunchArg::OnMessageMatch {
                        matches_to: content,
                        message: m.clone(),
                    },
                })
                .for_each(|arg| {
                    let mut instance = generator(arg);
                    let result = instance.launch(ctx);
                    if result.is_err() {
                        error!(
                            "Launching {} failed. error: {}",
                            info.name,
                            result.err().unwrap()
                        );
                    }
                })
        }
    }
}
