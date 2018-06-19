use chrono::Duration;
use std::sync::Arc;

use registry::SharedRegistry;

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub default_topic_ttl: Duration,
    pub default_subscription_ttl: Duration,
    pub default_message_ttl: Duration,
    pub default_ack_deadline: Duration,
    pub default_max_messages: usize,
    pub cleanup_interval: Duration,
}

pub struct HttpState {
    pub registry: SharedRegistry,
    pub config: Config,
}

impl HttpState {
    pub fn new(registry: &SharedRegistry, config: &Config) -> Self {
        HttpState {
            registry: Arc::clone(registry),
            config: config.clone(),
        }
    }
}
