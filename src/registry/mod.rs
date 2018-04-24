use chrono::prelude::*;
use chrono::Duration;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use courier::{Message, RawMessage, Subscription, SubscriptionMeta, Topic, TopicMeta};

#[cfg(test)]
mod tests;

struct TopicStore {
    topic: Topic,
    subscriptions: HashSet<String>,
}

impl TopicStore {
    fn new(name: &str, ttl: Duration) -> Self {
        TopicStore {
            topic: Topic::new(name, ttl),
            subscriptions: HashSet::new(),
        }
    }
}

struct TopicMetrics {
    count: u64,
    expired_count: u64,
}

struct SubscriptionMetrics {
    pulled_count: u64,
    acked_count: u64,
}

struct Metrics {
    messages_count: u64,
    expired_messages_count: u64,
    all_time_messages_count: u64,
    all_time_messages_pulled: u64,
    all_time_messages_acked: u64,

    topics_count: u64,
    all_time_topics_count: u64,
    subscriptions_count: u64,
    all_time_subscriptions_count: u64,

    topics: HashMap<String, TopicMetrics>,
    subscriptions: HashMap<String, SubscriptionMetrics>,

    memory_resident_set_size: u64,
    start_time: DateTime<Utc>,
}

pub struct Registry {
    topics: RwLock<HashMap<String, TopicStore>>,
    subscriptions: RwLock<HashMap<String, Subscription>>,
}

pub type SharedRegistry = Arc<Registry>;

impl Registry {
    pub fn new() -> SharedRegistry {
        Arc::new(Self {
            topics: RwLock::new(HashMap::new()),
            subscriptions: RwLock::new(HashMap::new()),
        })
    }

    pub fn create_topic(&self, topic_name: &str, message_ttl: Duration) -> (bool, TopicMeta) {
        let mut topics = self.topics.write().unwrap();
        let created = !topics.contains_key(topic_name);
        let topic = topics
            .entry(String::from(topic_name))
            .or_insert_with(|| TopicStore::new(topic_name, message_ttl));
        (created, TopicMeta::from(&topic.topic))
    }

    pub fn update_topic(
        &self,
        topic_name: &str,
        message_ttl: Option<Duration>,
    ) -> Option<TopicMeta> {
        let mut topics = self.topics.write().unwrap();
        topics.get_mut(topic_name).map(|t| {
            let old_ttl = t.topic.message_ttl;
            t.topic.set_message_ttl(message_ttl.unwrap_or(old_ttl));
            TopicMeta::from(&t.topic)
        })
    }

    pub fn delete_topic(&self, topic_name: &str) -> bool {
        self.topics.write().unwrap().remove(topic_name).is_some()
    }

    pub fn get_topic(&self, topic_name: &str) -> Option<TopicMeta> {
        let topics = self.topics.read().unwrap();
        topics.get(topic_name).map(|t| TopicMeta::from(&t.topic))
    }

    pub fn list_topics(&self) -> Vec<TopicMeta> {
        let topics = self.topics.read().unwrap();
        topics.values().map(|t| TopicMeta::from(&t.topic)).collect()
    }

    pub fn publish(&self, topic_name: &str, raw_messages: Vec<RawMessage>) -> Option<Vec<Uuid>> {
        let mut topics = self.topics.write().unwrap();
        topics.get_mut(topic_name).map(|topic_store| {
            let topic = &mut topic_store.topic;
            let mut ids = Vec::with_capacity(raw_messages.len());
            for raw_message in raw_messages {
                let message = Message::from(raw_message);
                ids.push(message.id);
                topic.publish(message)
            }
            ids
        })
    }

    pub fn list_topic_subscriptions(&self, topic_name: &str) -> Option<Vec<String>> {
        let mut topics = self.topics.write().unwrap();
        topics.get_mut(topic_name).map(|topic_store| {
            let subscriptions = &topic_store.subscriptions;
            subscriptions.into_iter().cloned().collect()
        })
    }

    pub fn create_subscription(
        &self,
        subscription_name: &str,
        topic_name: &str,
        ack_deadline: Duration,
        historical: bool,
    ) -> Option<(bool, SubscriptionMeta)> {
        let mut topics = self.topics.write().unwrap();
        let topic_store = topics.get_mut(topic_name)?;
        let topic = &topic_store.topic;
        let mut subscriptions = self.subscriptions.write().unwrap();
        let created = !subscriptions.contains_key(subscription_name);
        let subscription = subscriptions
            .entry(String::from(subscription_name))
            .or_insert_with(|| {
                if historical {
                    Subscription::new_head(subscription_name, topic, ack_deadline)
                } else {
                    Subscription::new_tail(subscription_name, topic, ack_deadline)
                }
            });
        topic_store
            .subscriptions
            .insert(String::from(subscription_name));
        Some((created, SubscriptionMeta::from(&*subscription)))
    }

    pub fn update_subscription(
        &self,
        subscription_name: &str,
        ack_deadline: Option<Duration>,
    ) -> Option<SubscriptionMeta> {
        let mut subscriptions = self.subscriptions.write().unwrap();
        subscriptions.get_mut(subscription_name).map(|s| {
            let old_deadline = s.ack_deadline;
            s.set_ack_deadline(ack_deadline.unwrap_or(old_deadline));
            SubscriptionMeta::from(&*s)
        })
    }

    pub fn delete_subscription(&self, subscription_name: &str) -> bool {
        let mut subscriptions = self.subscriptions.write().unwrap();
        let subscription = subscriptions.remove(subscription_name);
        match subscription {
            Some(sub) => {
                let mut topics = self.topics.write().unwrap();
                if let Some(topic_store) = topics.get_mut(&sub.topic) {
                    topic_store.subscriptions.remove(&sub.name);
                }
                true
            }
            None => false,
        }
    }

    pub fn get_subscription(&self, subscription_name: &str) -> Option<SubscriptionMeta> {
        let subscriptions = self.subscriptions.read().unwrap();
        subscriptions
            .get(subscription_name)
            .map(SubscriptionMeta::from)
    }

    pub fn list_subscriptions(&self) -> Vec<SubscriptionMeta> {
        let subscriptions = self.subscriptions.read().unwrap();
        subscriptions.values().map(SubscriptionMeta::from).collect()
    }

    pub fn pull(&self, subscription_name: &str, max_messages: usize) -> Option<Vec<Message>> {
        let mut subscriptions = self.subscriptions.write().unwrap();
        subscriptions
            .get_mut(subscription_name)
            .map(|subscription| {
                let mut messages = Vec::with_capacity(max_messages);
                while let Some(message) = subscription.pull() {
                    if messages.len() >= max_messages {
                        break;
                    }
                    messages.push(message);
                }
                messages
            })
    }

    pub fn ack(&self, subscription_name: &str, ids: &[Uuid]) -> bool {
        let mut subscriptions = self.subscriptions.write().unwrap();
        subscriptions
            .get_mut(subscription_name)
            .map(|subscription| {
                subscription.ack_many(ids);
                true
            })
            .unwrap_or(false)
    }

    pub fn cleanup(&self) {
        let mut topics = self.topics.write().unwrap();
        for mut topic in topics.values_mut() {
            topic.topic.cleanup();
        }
    }
}
