use std::sync::mpsc::Receiver;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::message::MessageFormat;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProducerMessage {
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
}

pub type ProducerResult<T> = Result<T, Box<dyn std::error::Error>>;

pub type ProducerMessageReceiver = ProducerResult<Receiver<ProducerMessage>>;

pub trait Producer<S> {
    fn setup(&mut self, settings: Option<S>) -> ProducerMessageReceiver;
    fn start(&self);
    fn kind() -> String;
}

#[async_trait]
pub trait AsyncProducer<S> {
    fn setup(&mut self, settings: Option<S>) -> ProducerMessageReceiver;
    async fn start(&self);
    fn kind() -> String;
}
