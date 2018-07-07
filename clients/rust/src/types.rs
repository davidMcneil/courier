use chrono::prelude::*;
use uuid::Uuid;

/// A message which can be published to a [Topic](struct.Topic.html).
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Message {
    /// Unique identifier for this message.
    pub id: Uuid,
    /// Time the message was published.
    pub time: DateTime<Utc>,
    /// Number of times the message has been tried (pulled).
    pub tries: u32,
    /// Actual message data.
    pub data: String,
}

/// A subscription meta type used for serialization.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Subscription {
    /// Unique name for this subscription.
    pub name: String,
    /// Topic name the subscription is subscribed to.
    pub topic: String,
    /// Amount of time given to ack a message in seconds.
    pub ack_deadline: i64,
    /// Time to live of the subscription in seconds.
    pub ttl: i64,
    /// Time the subscription was created.
    pub created: DateTime<Utc>,
    /// Time the subscription was last updated.
    pub updated: DateTime<Utc>,
}

/// A topic meta type used for serialization.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Topic {
    /// Unique name of the topic.
    pub name: String,
    /// Message time to live in seconds.
    pub message_ttl: i64,
    /// Time to live of the topic in seconds.
    pub ttl: i64,
    /// Time the topic was created.
    pub created: DateTime<Utc>,
    /// Time the topic was updated.
    pub updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TopicCreateConfig {
    pub message_ttl: Option<u32>,
    pub ttl: Option<u32>,
}

impl TopicCreateConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TopicUpdateConfig {
    pub message_ttl: Option<u32>,
    pub ttl: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TopicList {
    pub topics: Vec<Topic>,
}

impl TopicList {
    pub fn new(topics: Vec<Topic>) -> Self {
        Self { topics }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RawMessage {
    pub data: String,
}

impl RawMessage {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RawMessageList {
    pub raw_messages: Vec<RawMessage>,
}

impl RawMessageList {
    pub fn new(raw_messages: Vec<RawMessage>) -> Self {
        Self { raw_messages }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MessageIdList {
    pub message_ids: Vec<Uuid>,
}

impl MessageIdList {
    pub fn new(message_ids: Vec<Uuid>) -> Self {
        Self { message_ids }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SubscriptionNameList {
    pub subscription_names: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SubscriptionCreateConfig {
    pub topic: String,
    pub ack_deadline: Option<u32>,
    pub ttl: Option<u32>,
    pub historical: Option<bool>,
}

impl SubscriptionCreateConfig {
    pub fn new(topic: &str) -> Self {
        SubscriptionCreateConfig {
            topic: String::from(topic),
            ack_deadline: None,
            ttl: None,
            historical: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SubscriptionUpdateConfig {
    pub ack_deadline: Option<u32>,
    pub ttl: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SubscriptionList {
    pub subscriptions: Vec<Subscription>,
}

impl SubscriptionList {
    pub fn new(subscriptions: Vec<Subscription>) -> Self {
        Self { subscriptions }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MessageList {
    pub messages: Vec<Message>,
}

impl MessageList {
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PullConfig {
    pub max_messages: Option<usize>,
}

impl PullConfig {
    pub fn new(max_messages: usize) -> Self {
        Self {
            max_messages: Some(max_messages),
        }
    }
}
