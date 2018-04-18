use chrono::Duration;
use chrono::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time;
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

pub struct Registry {
    topics: HashMap<String, TopicStore>,
    subscriptions: HashMap<String, Subscription>,
}

pub type SharedRegistry = Arc<RwLock<Registry>>;

impl Registry {
    pub fn new() -> SharedRegistry {
        Arc::new(RwLock::new(Self {
            topics: HashMap::new(),
            subscriptions: HashMap::new(),
        }))
    }

    pub fn create_topic(&mut self, topic_name: &str, message_ttl: Duration) -> (bool, TopicMeta) {
        let created = !self.topics.contains_key(topic_name);
        let topic = self.topics
            .entry(String::from(topic_name))
            .or_insert_with(|| TopicStore::new(topic_name, message_ttl));
        (created, TopicMeta::from(&topic.topic))
    }

    pub fn update_topic(
        &mut self,
        topic_name: &str,
        message_ttl: Option<Duration>,
    ) -> Option<TopicMeta> {
        self.topics.get_mut(topic_name).map(|t| {
            let old_ttl = t.topic.message_ttl;
            t.topic.set_message_ttl(message_ttl.unwrap_or(old_ttl));
            TopicMeta::from(&t.topic)
        })
    }

    pub fn delete_topic(&mut self, topic_name: &str) -> bool {
        self.topics.remove(topic_name).is_some()
    }

    pub fn get_topic(&self, topic_name: &str) -> Option<TopicMeta> {
        self.topics
            .get(topic_name)
            .map(|t| TopicMeta::from(&t.topic))
    }

    pub fn list_topics(&self) -> Vec<TopicMeta> {
        self.topics
            .values()
            .map(|t| TopicMeta::from(&t.topic))
            .collect()
    }

    pub fn publish(
        &mut self,
        topic_name: &str,
        raw_messages: Vec<RawMessage>,
    ) -> Option<Vec<Uuid>> {
        self.topics.get_mut(topic_name).map(|topic_store| {
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

    pub fn list_topic_subscriptions(&mut self, topic_name: &str) -> Option<Vec<String>> {
        self.topics.get_mut(topic_name).map(|topic_store| {
            let subscriptions = &topic_store.subscriptions;
            subscriptions.into_iter().cloned().collect()
        })
    }

    pub fn create_subscription(
        &mut self,
        subscription_name: &str,
        topic_name: &str,
        ack_deadline: Duration,
    ) -> Option<(bool, SubscriptionMeta)> {
        let topic_store = self.topics.get_mut(topic_name)?;
        let topic = &topic_store.topic;
        let created = !self.subscriptions.contains_key(subscription_name);
        let subscription = self.subscriptions
            .entry(String::from(subscription_name))
            .or_insert_with(|| Subscription::new_head(subscription_name, topic, ack_deadline));
        topic_store
            .subscriptions
            .insert(String::from(subscription_name));
        Some((created, SubscriptionMeta::from(&*subscription)))
    }

    pub fn update_subscription(
        &mut self,
        subscription_name: &str,
        ack_deadline: Option<Duration>,
    ) -> Option<SubscriptionMeta> {
        self.subscriptions.get_mut(subscription_name).map(|s| {
            let old_deadline = s.ack_deadline;
            s.set_ack_deadline(ack_deadline.unwrap_or(old_deadline));
            SubscriptionMeta::from(&*s)
        })
    }

    pub fn delete_subscription(&mut self, subscription_name: &str) -> bool {
        self.subscriptions.remove(subscription_name).is_some()
    }

    pub fn get_subscription(&self, subscription_name: &str) -> Option<SubscriptionMeta> {
        self.subscriptions
            .get(subscription_name)
            .map(SubscriptionMeta::from)
    }

    pub fn list_subscriptions(&self) -> Vec<SubscriptionMeta> {
        self.subscriptions
            .values()
            .map(SubscriptionMeta::from)
            .collect()
    }

    pub fn pull_immediate(
        &mut self,
        subscription_name: &str,
        max_messages: usize,
    ) -> Option<Vec<Message>> {
        self.subscriptions
            .get_mut(subscription_name)
            .map(|subscription| {
                let mut messages = Vec::with_capacity(max_messages);
                while let Some(message) = subscription.pull() {
                    messages.push(message);
                    if messages.len() >= max_messages {
                        break;
                    }
                }
                messages
            })
    }

    pub fn pull_wait(
        &mut self,
        subscription_name: &str,
        max_messages: usize,
        wait: Duration,
    ) -> Option<Vec<Message>> {
        let start = Utc::now();
        self.subscriptions
            .get_mut(subscription_name)
            .map(|subscription| {
                let mut messages = Vec::with_capacity(max_messages);
                loop {
                    while let Some(message) = subscription.pull() {
                        messages.push(message);
                        if messages.len() >= max_messages {
                            break;
                        }
                    }
                    if messages.len() >= max_messages
                        || Utc::now().signed_duration_since(start) > wait
                    {
                        break;
                    }
                    thread::sleep(time::Duration::from_millis(10));
                }
                messages
            })
    }

    pub fn ack(&mut self, subscription_name: &str, ids: &[Uuid]) -> bool {
        self.subscriptions
            .get_mut(subscription_name)
            .map(|subscription| {
                subscription.ack_many(ids);
                true
            })
            .unwrap_or(false)
    }

    pub fn cleanup(&mut self) {
        for mut topic in self.topics.values_mut() {
            topic.topic.cleanup();
        }
    }
}
