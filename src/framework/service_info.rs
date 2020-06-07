use crate::framework::launch_arg::LaunchArg;
use crate::framework::service::{Client, LaunchTiming, Service};
use crate::framework::service::{TextChannel, VoiceChannel};
use log::info;
use regex::Regex;
use std::marker::PhantomData;
use std::str::FromStr;

pub enum ArgType {
    String,
    Int,
    Double,
    User,
    TextChannel,
    VoiceChannel,
    RegexMatch(Regex),
    Custom(Box<dyn Fn(String) -> bool + 'static>),
}

pub enum ArgResult<TClient: Client> {
    String(String),
    Int(i128),
    Double(f64),
    User(TClient::User),
    TextChannel(TClient::TextChannel),
    VoiceChannel(TClient::VoiceChannel),
    RegexMatch(Regex, String),
    Custom(String),
}

pub struct ArgEntry {
    pub name: String,
    pub description: String,
    pub arg_type: ArgType,
    pub is_optional: bool,
}

pub struct ServiceInfo<TClient: Client> {
    pub name: String,
    pub description: String,
    pub initial_timings: Vec<LaunchTiming<TClient>>,
}

pub struct ServiceInfoBuilder<TClient: Client> {
    name: Option<String>,
    description: Option<String>,
    initial_timings: Option<Vec<LaunchTiming<TClient>>>,
}

#[derive(Debug)]
pub struct BuildServiceInfoError(pub String);

impl<TClient: Client> ServiceInfoBuilder<TClient> {
    pub fn new() -> Self {
        ServiceInfoBuilder {
            name: None,
            description: None,
            initial_timings: None,
        }
    }

    pub fn name<A: Into<String>>(mut self, name: A) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description<A: Into<String>>(mut self, description: A) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn timing(mut self, timing: LaunchTiming<TClient>) -> Self {
        if self.initial_timings.is_none() {
            self.initial_timings = Some(vec![]);
        }

        self.initial_timings.as_mut().unwrap().push(timing);
        self
    }

    pub fn build(self) -> Result<ServiceInfo<TClient>, BuildServiceInfoError> {
        let name = self
            .name
            .ok_or(BuildServiceInfoError("名前がありません".into()))?;

        let description = self
            .description
            .ok_or(BuildServiceInfoError("説明がありません".into()))?;

        let timings = self.initial_timings.ok_or(BuildServiceInfoError(
            "起動タイミングが指定されていません：このサービスを起動する方法がありません".into(),
        ))?;

        let result = ServiceInfo {
            name,
            description,
            initial_timings: timings,
        };

        Ok(result)
    }
}
