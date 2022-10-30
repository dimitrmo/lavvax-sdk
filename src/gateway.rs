use std::sync::mpsc::Receiver;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::cluster::{ClusterSize, Provider};
use crate::message::MessageFormat;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GatewayMessage {
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "val")]
    pub value: MessageFormat,
    #[serde(rename = "cat", skip_serializing_if = "String::is_empty")]
    pub category: String,
    #[serde(rename = "siz", skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,
    #[serde(rename = "tms")]
    pub timestamp: u64,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub client_ids: Option<Vec<String>>,
    #[serde(rename = "gid", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct GatewayConfig {
    pub port: u16,
    pub host: String,
    pub size: ClusterSize,
    pub provider: Provider,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        GatewayConfig {
            port: 3333,
            host: "0.0.0.0".to_string(),
            size: ClusterSize::Small,
            provider: Provider::Local,
        }
    }
}

pub type GatewayResult<T> = Result<T, Box<dyn std::error::Error>>;

pub type GatewayMessageReceiver = GatewayResult<Receiver<GatewayMessage>>;

#[async_trait]
pub trait AsyncGateway<S> {
    fn setup(&mut self, config: GatewayConfig, settings: Option<S>) -> GatewayMessageReceiver;
    async fn start(&self);
    fn kind() -> String;
}
