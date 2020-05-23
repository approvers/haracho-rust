use super::LaunchTiming;
use log::info;

#[derive(Hash, PartialEq, Eq)]
pub struct ServiceInfo {
    pub name: String,
    pub description: String,
    pub initial_timings: Vec<LaunchTiming>,

    pub args_description: Option<Vec<String>>,
}

pub struct ServiceInfoBuilder<'a> {
    name: Option<&'a str>,
    description: Option<&'a str>,
    initial_timings: Vec<LaunchTiming>,

    args_descriptions: Option<&'a [&'a str]>,
}

impl<'instance> ServiceInfoBuilder<'instance> {
    pub fn new() -> Self {
        ServiceInfoBuilder {
            name: None,
            description: None,
            initial_timings: Vec::new(),

            args_descriptions: None,
        }
    }

    pub fn name<'a: 'instance>(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn description<'a: 'instance>(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn args_descriptions<'a: 'instance>(mut self, args_desc: &'a [&str]) -> Self {
        self.args_descriptions = Some(args_desc);
        self
    }

    pub fn timing(mut self, new_timing: LaunchTiming) -> Self {
        self.initial_timings.push(new_timing);
        self
    }

    pub fn build(self) -> ServiceInfo {
        if self.name.is_none() {
            panic!("Building service info failed: name is empty.");
        }

        if self.description.is_none() {
            panic!("Building service info failed: description is empty.");
        }

        if self.args_descriptions.is_none() {
            info!(
                "\"{}\" service's args descriptions is empty.",
                self.name.unwrap()
            );
        }

        if self.initial_timings.is_empty() {
            panic!(
                "\"{}\" service's timing is empty: There is no way to call this service!",
                self.name.unwrap()
            )
        }

        let args_desc = {
            if self.args_descriptions.is_some() {
                Some(
                    self.args_descriptions
                        .unwrap()
                        .iter()
                        .map(|x| String::from(*x))
                        .collect(),
                )
            } else {
                None
            }
        };

        ServiceInfo {
            name: String::from(self.name.unwrap()),
            description: String::from(self.description.unwrap()),
            initial_timings: self.initial_timings,

            args_description: args_desc,
        }
    }
}
