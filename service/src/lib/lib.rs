pub use crate::core::{Message, Subscription, SubscriptionMeta, Topic, TopicMeta};
use chrono::prelude::*;
use chrono::Duration;
use parking_lot::RwLock;
use psutil;
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

pub mod commit_log;
mod core;

struct TopicStore {
    topic: Topic,
    subscriptions: HashSet<String>,
}

impl TopicStore {
    fn new(name: &str, message_ttl: Duration, ttl: Duration) -> Self {
        TopicStore {
            topic: Topic::new(name, message_ttl, ttl),
            subscriptions: HashSet::new(),
        }
    }
}

/// Metrics on a topic.
#[derive(Debug, Deserialize, Serialize)]
pub struct TopicMetrics {
    /// Number of messages in the topic.
    pub messages: usize,
    /// Number of messages published all time.
    pub messages_all_time: u64,
    /// Number of messages expired all time.
    pub expired_all_time: u64,
    /// Message time to live.
    pub message_ttl: i64,
    /// Time to live.
    pub ttl: i64,
    /// When the topic was created.
    pub created: DateTime<Utc>,
    /// When the topic was last updated.
    pub updated: DateTime<Utc>,
}

impl TopicMetrics {
    /// Create a new topic metrics.
    pub fn new(topic: &Topic) -> Self {
        Self {
            messages: 0,
            messages_all_time: 0,
            expired_all_time: 0,
            message_ttl: topic.message_ttl.num_seconds(),
            ttl: topic.ttl.num_seconds(),
            created: topic.created,
            updated: topic.updated,
        }
    }
}

/// Metrics on a subscription.
#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriptionMetrics {
    /// Number of currently pending messages.
    pub pending: usize,
    /// Number of messages pulled all time.
    pub pulled_all_time: u64,
    /// Number of pulls of an already pulled message all time.
    pub pulled_retries_all_time: u64,
    /// Number of messages tried to ack all time.
    pub acks_all_time: u64,
    /// Number of messages successfully acked all time.
    pub acked_all_time: u64,
    /// Topic name.
    pub topic: String,
    /// Index into a topic.
    pub message_index: usize,
    /// Ack deadline.
    pub ack_deadline: i64,
    /// Time to live.
    pub ttl: i64,
    /// When the subscription was created.
    pub created: DateTime<Utc>,
    /// When the subscription was last updated.
    updated: DateTime<Utc>,
}

impl SubscriptionMetrics {
    /// Create a new subscription metrics.
    pub fn new(subscription: &Subscription) -> Self {
        Self {
            pending: 0,
            pulled_all_time: 0,
            pulled_retries_all_time: 0,
            acks_all_time: 0,
            acked_all_time: 0,
            topic: subscription.topic.clone(),
            message_index: subscription.next_index(),
            ack_deadline: subscription.ack_deadline.num_seconds(),
            ttl: subscription.ttl.num_seconds(),
            created: subscription.created,
            updated: subscription.updated,
        }
    }
}

/// Courier metrics.
#[derive(Debug, Deserialize, Serialize)]
pub struct Metrics {
    /// Number of topics all time.
    pub topics_all_time: u64,
    /// Number of subscriptions all time.
    pub subscriptions_all_time: u64,
    /// The memory resident set size usage.
    pub memory_resident_set_size: i64,
    /// When the service was started.
    pub start_time: DateTime<Utc>,
    /// Topic metrics.
    pub topics: HashMap<String, TopicMetrics>,
    /// Subscription metrics.
    pub subscriptions: HashMap<String, SubscriptionMetrics>,
}

impl Metrics {
    /// Create a new metrics.
    pub fn new() -> Self {
        Self {
            topics_all_time: 0,
            subscriptions_all_time: 0,
            memory_resident_set_size: 0,
            start_time: Utc::now(),
            topics: HashMap::new(),
            subscriptions: HashMap::new(),
        }
    }
}

/// A registry mapping names to topics and subscriptions and the relevant metrics.
pub struct Registry {
    topics: RwLock<HashMap<String, TopicStore>>,
    subscriptions: RwLock<HashMap<String, Subscription>>,
    metrics: Arc<RwLock<Metrics>>,
}

/// A [Registry](struct.Registry.html) which can be shared between threads.
pub type SharedRegistry = Arc<Registry>;

impl Registry {
    /// Create a new shared registry.
    pub fn new() -> SharedRegistry {
        Arc::new(Self {
            topics: RwLock::new(HashMap::new()),
            subscriptions: RwLock::new(HashMap::new()),
            metrics: Arc::new(RwLock::new(Metrics::new())),
        })
    }

    /// Create a new topic returning true if the operation resulted in a new topic and the topic meta data.
    pub fn create_topic(
        &self,
        topic_name: &str,
        message_ttl: Duration,
        ttl: Duration,
    ) -> (bool, TopicMeta) {
        let mut topics = self.topics.write();

        let created = !topics.contains_key(topic_name);
        let topic_store = topics
            .entry(String::from(topic_name))
            .or_insert_with(|| TopicStore::new(topic_name, message_ttl, ttl));

        // Update metrics
        if created {
            let mut metrics = self.metrics.write();
            metrics.topics_all_time += 1;
            metrics.topics.insert(
                String::from(topic_name),
                TopicMetrics::new(&topic_store.topic),
            );
        }

        (created, TopicMeta::from(&topic_store.topic))
    }

    /// Update a topic and return the topic meta or None if the topic does not exist.
    pub fn update_topic(
        &self,
        topic_name: &str,
        message_ttl: Option<Duration>,
        ttl: Option<Duration>,
    ) -> Option<TopicMeta> {
        let mut topics = self.topics.write();

        topics.get_mut(topic_name).map(|topic_store| {
            let topic = &mut topic_store.topic;

            if let Some(v) = message_ttl {
                topic.set_message_ttl(v);
            }
            if let Some(v) = ttl {
                topic.set_ttl(v);
            }

            // Ensure that updated was updated
            topic.update();

            // Update metrics
            let mut metrics = self.metrics.write();
            if let Some(m) = metrics.topics.get_mut(topic_name) {
                m.message_ttl = topic.message_ttl.num_seconds();
                m.ttl = topic.ttl.num_seconds();
                m.updated = topic.updated;
            };

            TopicMeta::from(&*topic)
        })
    }

    /// Delete a topic return false if the topic does not exist.
    pub fn delete_topic(&self, topic_name: &str) -> bool {
        // Update metrics
        {
            let mut metrics = self.metrics.write();
            metrics.topics.remove(topic_name);
        }

        let topic_store = {
            let mut topics = self.topics.write();
            topics.remove(topic_name)
        };

        // Delete all subscriptions
        if let Some(ts) = topic_store {
            for subscription in &ts.subscriptions {
                self.delete_subscription(&subscription);
            }
            true
        } else {
            false
        }
    }

    /// Get the topic meta data or None if the topic does not exist.
    pub fn get_topic(&self, topic_name: &str) -> Option<TopicMeta> {
        let topics = self.topics.read();
        topics.get(topic_name).map(|ts| TopicMeta::from(&ts.topic))
    }

    /// Get a list of all topic meta data.
    pub fn list_topics(&self) -> Vec<TopicMeta> {
        let topics = self.topics.read();
        topics
            .values()
            .map(|ts| TopicMeta::from(&ts.topic))
            .collect()
    }

    /// Publish a list of data as messages to a topic return a list of published message ids or None
    /// if the topic does not exist.
    pub fn publish(&self, topic_name: &str, data: Vec<String>) -> Option<Vec<Uuid>> {
        let mut topics = self.topics.write();

        topics.get_mut(topic_name).map(|topic_store| {
            let topic = &mut topic_store.topic;
            let count = data.len();
            let mut ids = Vec::with_capacity(count);
            for d in data {
                ids.push(topic.publish(d));
            }

            // Update metrics
            let mut metrics = self.metrics.write();
            if let Some(m) = metrics.topics.get_mut(topic_name) {
                m.messages = topic.len();
                m.messages_all_time += count as u64;
                m.updated = topic.updated;
            }

            ids
        })
    }

    /// Return a list of subscription names that are subscribed to this topic or None if the topic
    /// does not exist.
    pub fn list_topic_subscriptions(&self, topic_name: &str) -> Option<Vec<String>> {
        let mut topics = self.topics.write();
        topics.get_mut(topic_name).map(|topic_store| {
            let subscriptions = &topic_store.subscriptions;
            subscriptions.into_iter().cloned().collect()
        })
    }

    /// Create a new subscription returning true if the operation resulted in a new subscription and
    /// the subscription meta data or None if the topic does not exist.
    pub fn create_subscription(
        &self,
        subscription_name: &str,
        topic_name: &str,
        ack_deadline: Duration,
        ttl: Duration,
        historical: bool,
    ) -> Option<(bool, SubscriptionMeta)> {
        let mut topics = self.topics.write();
        let topic_store = topics.get_mut(topic_name)?;
        let topic = &topic_store.topic;

        let mut subscriptions = self.subscriptions.write();
        let created = !subscriptions.contains_key(subscription_name);
        let subscription = subscriptions
            .entry(String::from(subscription_name))
            .or_insert_with(|| {
                if historical {
                    Subscription::new_head(subscription_name, topic, ack_deadline, ttl)
                } else {
                    Subscription::new_tail(subscription_name, topic, ack_deadline, ttl)
                }
            });

        topic_store
            .subscriptions
            .insert(String::from(subscription_name));

        // Update metrics
        if created {
            let mut metrics = self.metrics.write();
            metrics.subscriptions_all_time += 1;
            metrics.subscriptions.insert(
                String::from(subscription_name),
                SubscriptionMetrics::new(subscription),
            );
        }

        Some((created, SubscriptionMeta::from(&*subscription)))
    }

    /// Update a subscription and return the subscription meta data or None if the subscription does
    /// not exist.
    pub fn update_subscription(
        &self,
        subscription_name: &str,
        ack_deadline: Option<Duration>,
        ttl: Option<Duration>,
    ) -> Option<SubscriptionMeta> {
        let mut subscriptions = self.subscriptions.write();
        subscriptions
            .get_mut(subscription_name)
            .map(|subscription| {
                if let Some(v) = ack_deadline {
                    subscription.set_ack_deadline(v);
                }
                if let Some(v) = ttl {
                    subscription.set_ttl(v);
                }

                // Ensure that updated was updated
                subscription.update();

                // Update metrics
                let mut metrics = self.metrics.write();
                if let Some(m) = metrics.subscriptions.get_mut(subscription_name) {
                    m.ack_deadline = subscription.ack_deadline.num_seconds();
                    m.ttl = subscription.ttl.num_seconds();
                    m.updated = subscription.updated;
                }

                SubscriptionMeta::from(&*subscription)
            })
    }

    /// Delete a subscription return false if the subscription does not exist.
    pub fn delete_subscription(&self, subscription_name: &str) -> bool {
        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.subscriptions.remove(subscription_name);

        let mut subscriptions = self.subscriptions.write();
        let subscription = subscriptions.remove(subscription_name);

        if let Some(s) = subscription {
            // Remove the subscription from the topic if it exists
            let mut topics = self.topics.write();
            if let Some(topic_store) = topics.get_mut(&s.topic) {
                topic_store.subscriptions.remove(&s.name);
            }
            true
        } else {
            false
        }
    }

    /// Get the subscription meta data or None if the subscription does not exist.
    pub fn get_subscription(&self, subscription_name: &str) -> Option<SubscriptionMeta> {
        let subscriptions = self.subscriptions.read();
        subscriptions
            .get(subscription_name)
            .map(SubscriptionMeta::from)
    }

    /// Get a list of all subscription meta data.
    pub fn list_subscriptions(&self) -> Vec<SubscriptionMeta> {
        let subscriptions = self.subscriptions.read();
        subscriptions.values().map(SubscriptionMeta::from).collect()
    }

    /// Retrieve messages from a subscription return the list of messages or None if the
    /// subscription does not exist.
    pub fn pull(&self, subscription_name: &str, max_messages: usize) -> Option<Vec<Message>> {
        let mut subscriptions = self.subscriptions.write();
        subscriptions
            .get_mut(subscription_name)
            .map(|subscription| {
                let mut retry_count = 0;
                let mut messages = Vec::with_capacity(max_messages);
                while let Some(message) = subscription.pull() {
                    if message.tries > 1 {
                        retry_count += 1;
                    }
                    messages.push(message);
                    if messages.len() >= max_messages {
                        break;
                    }
                }

                // Update metrics
                let mut metrics = self.metrics.write();
                if let Some(m) = metrics.subscriptions.get_mut(subscription_name) {
                    m.pending = subscription.num_pending();
                    m.pulled_all_time += messages.len() as u64;
                    m.pulled_retries_all_time += retry_count;
                    m.message_index = subscription.next_index();
                    m.updated = subscription.updated;
                }

                messages
            })
    }

    /// Ack message ids returning the list of successfully acked ids or None if the subscription
    /// does not exist.
    pub fn ack(&self, subscription_name: &str, ids: &[Uuid]) -> Option<Vec<Uuid>> {
        let mut subscriptions = self.subscriptions.write();
        subscriptions.get_mut(subscription_name).map(|s| {
            let acked = s.ack_many(ids);

            // Update metrics
            let mut metrics = self.metrics.write();
            if let Some(m) = metrics.subscriptions.get_mut(subscription_name) {
                m.pending = s.num_pending();
                m.acked_all_time += ids.len() as u64;
                m.acks_all_time += acked.len() as u64;
                m.updated = s.updated;
            };

            acked
        })
    }

    /// Get a copy of the metrics.
    pub fn metrics(&self) -> Arc<RwLock<Metrics>> {
        Arc::clone(&self.metrics)
    }

    /// Cleanup the registry removing messages, topics, and subscriptions that meet their ttl.
    pub fn cleanup(&self) -> (usize, usize, usize) {
        let mut metrics = self.metrics.write();

        // Remove timed out subscriptions
        let mut subscriptions = self.subscriptions.write();
        let original_subscriptions_count = subscriptions.len();
        subscriptions.retain(|_, s| {
            if s.ttl == Duration::seconds(0) {
                true
            } else {
                Utc::now().signed_duration_since(s.updated) <= s.ttl
            }
        });
        let subscriptions_removed = original_subscriptions_count - subscriptions.len();

        // Update metrics to match the removed subscriptions
        metrics
            .subscriptions
            .retain(|name, _| subscriptions.contains_key(name));

        // Remove timed out topics
        let mut topics = self.topics.write();
        let original_topics_count = topics.len();
        topics.retain(|_, ts| {
            if ts.topic.ttl == Duration::seconds(0) {
                true
            } else {
                Utc::now().signed_duration_since(ts.topic.updated) <= ts.topic.ttl
            }
        });
        let topics_removed = original_topics_count - topics.len();

        // Update metrics to match the removed subscriptions
        metrics.topics.retain(|name, _| topics.contains_key(name));

        // Cleanup the messages of each topic
        let mut messages_removed = 0;
        for (topic_name, topic_store) in topics.iter_mut() {
            let count = topic_store.topic.cleanup();
            messages_removed += count;

            // Update metrics
            if let Some(m) = metrics.topics.get_mut(topic_name) {
                m.messages = topic_store.topic.len();
                m.expired_all_time += count as u64;
            }
        }

        // Update metrics used memory
        let process = psutil::process::Process::new(psutil::getpid());
        metrics.memory_resident_set_size = if let Ok(process) = process {
            process.rss
        } else {
            0
        };

        (topics_removed, subscriptions_removed, messages_removed)
    }
}
