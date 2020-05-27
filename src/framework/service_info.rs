use super::LaunchTiming;
use derive_builder::Builder;
use log::info;

#[derive(Hash, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct ServiceInfo {
    pub name: String,
    pub description: String,
    pub initial_timings: Vec<LaunchTiming>,

    #[builder(setter(strip_option), default)]
    pub args_description: Option<Vec<String>>,
}

impl ServiceInfoBuilder {
    pub fn timing(mut self, t: LaunchTiming) -> Self {
        if self.initial_timings.is_none() {
            self.initial_timings = Some(vec![]);
        }

        self.initial_timings.as_mut().unwrap().push(t);
        self
    }
}
