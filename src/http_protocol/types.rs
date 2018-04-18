use uuid::Uuid;

use courier::{Message, RawMessage, SubscriptionMeta, TopicMeta};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TopicCreateConfig {
    pub message_ttl: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TopicUpdateConfig {
    pub message_ttl: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TopicList {
    pub topics: Vec<TopicMeta>,
}

impl TopicList {
    pub fn new(topics: Vec<TopicMeta>) -> Self {
        Self { topics }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RawMessageList {
    pub messages: Vec<RawMessage>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageIdList {
    pub message_ids: Vec<Uuid>,
}

impl MessageIdList {
    pub fn new(message_ids: Vec<Uuid>) -> Self {
        Self { message_ids }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SubscriptionNameList {
    pub subscriptions: Vec<String>,
}

impl SubscriptionNameList {
    pub fn new(subscriptions: Vec<String>) -> Self {
        Self { subscriptions }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SubscriptionCreateConfig {
    pub topic: String,
    pub ack_deadline: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SubscriptionUpdateConfig {
    pub ack_deadline: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SubscriptionList {
    pub subscriptions: Vec<SubscriptionMeta>,
}

impl SubscriptionList {
    pub fn new(subscriptions: Vec<SubscriptionMeta>) -> Self {
        Self { subscriptions }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageList {
    pub messages: Vec<Message>,
}

impl MessageList {
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PullConfig {
    pub return_immediately: bool,
    pub max_messages: usize,
}
