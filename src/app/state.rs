use axum::extract::FromRef;

use super::config::Config;

use crate::example::ExampleThing;

#[derive(Clone)]
pub struct AppState {
    // TODO: add state and extractors here
    //  Here's a little example thing for the moment
    example_thing: ExampleThing,
}

impl AppState {
    pub async fn from_config(_config: &Config) -> Result<Self, AppStateSetupError> {
        // TODO: setup state here from the config
        let example_thing = ExampleThing::new();

        Ok(Self { example_thing })
    }
}

impl FromRef<AppState> for ExampleThing {
    fn from_ref(state: &AppState) -> Self {
        state.example_thing.clone()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppStateSetupError {
    // TODO: register state setup errors here
}
