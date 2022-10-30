use serde::{Deserialize, Serialize};

fn default_enabled() -> bool { false }

fn default_timeout() -> u64 {
    5000
}

fn default_method() -> String { "POST".to_string() }

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Webhook {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub endpoint: String,
    #[serde(default = "default_method")]
    pub method: String,
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}
