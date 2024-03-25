use figment::providers::Format;
use figment::{providers, Figment};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AobaConfig {
    pub arkalis_url: String,
}

impl AobaConfig {
    pub fn new() -> Self {
        Figment::new()
            .merge(providers::Json::file("config.json"))
            .merge(providers::Env::prefixed("AOBA_"))
            .extract()
            .expect("Failed to load config")
    }
}
