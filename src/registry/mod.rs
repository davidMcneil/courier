use chrono::prelude::*;
use chrono::Duration;
use psutil;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use courier::{Message, RawMessage, Subscription, SubscriptionMeta, Topic, TopicMeta};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct TopicMetrics {
    messages: usize,
    messages_all_time: u64,
    expired_all_time: u64,
    message_ttl: i64,
    ttl: i64,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl TopicMetrics {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriptionMetrics {
    pending: usize,
    pulled_all_time: u64,
    acked_all_time: u64,
    topic: String,
    topic_message_index: usize,
    ack_deadline: i64,
    ttl: i64,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl SubscriptionMetrics {
    pub fn new(subscription: &Subscription) -> Self {
        Self {
            pending: 0,
            pulled_all_time: 0,
            acked_all_time: 0,
            topic: subscription.topic.clone(),
            topic_message_index: subscription.next_index(),
            ack_deadline: subscription.ack_deadline.num_seconds(),
            ttl: subscription.ttl.num_seconds(),
            created: subscription.created,
            updated: subscription.updated,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metrics {
    topics_all_time: u64,
    subscriptions_all_time: u64,
    memory_resident_set_size: i64,
    start_time: DateTime<Utc>,
    topics: HashMap<String, TopicMetrics>,
    subscriptions: HashMap<String, SubscriptionMetrics>,
}

impl Metrics {
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

pub struct Registry {
    topics: RwLock<HashMap<String, TopicStore>>,
    subscriptions: RwLock<HashMap<String, Subscription>>,
    metrics: Arc<RwLock<Metrics>>,
}

pub type SharedRegistry = Arc<Registry>;

impl Registry {
    pub fn new() -> SharedRegistry {
        Arc::new(Self {
            topics: RwLock::new(HashMap::new()),
            subscriptions: RwLock::new(HashMap::new()),
            metrics: Arc::new(RwLock::new(Metrics::new())),
        })
    }

    pub fn create_topic(
        &self,
        topic_name: &str,
        message_ttl: Duration,
        ttl: Duration,
    ) -> (bool, TopicMeta) {
        let mut topics = self.topics.write().unwrap();
        let created = !topics.contains_key(topic_name);
        let topic = topics
            .entry(String::from(topic_name))
            .or_insert_with(|| TopicStore::new(topic_name, message_ttl, ttl));

        // Update metrics
        if created {
            let mut metrics = self.metrics.write().unwrap();
            metrics.topics_all_time += 1;
            metrics
                .topics
                .insert(String::from(topic_name), TopicMetrics::new(&topic.topic));
        }

        (created, TopicMeta::from(&topic.topic))
    }

    pub fn update_topic(
        &self,
        topic_name: &str,
        message_ttl: Option<Duration>,
        ttl: Option<Duration>,
    ) -> Option<TopicMeta> {
        let mut topics = self.topics.write().unwrap();
        topics.get_mut(topic_name).map(|t| {
            if let Some(v) = message_ttl {
                t.topic.set_message_ttl(v);
            }
            if let Some(v) = ttl {
                t.topic.set_ttl(v);
            }

            // Ensure that updated was updated
            t.topic.update();

            // Update metrics
            let mut metrics = self.metrics.write().unwrap();
            if let Some(m) = metrics.topics.get_mut(topic_name) {
                m.message_ttl = t.topic.message_ttl.num_seconds();
                m.ttl = t.topic.ttl.num_seconds();
            };

            TopicMeta::from(&t.topic)
        })
    }

    pub fn delete_topic(&self, topic_name: &str) -> bool {
        // Update metrics
        let mut metrics = self.metrics.write().unwrap();
        metrics.topics.remove(topic_name);

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
            let count = raw_messages.len();
            let mut ids = Vec::with_capacity(count);
            for raw_message in raw_messages {
                let message = Message::from(raw_message);
                ids.push(message.id);
                topic.publish(message)
            }

            // Update metrics
            let mut metrics = self.metrics.write().unwrap();
            if let Some(m) = metrics.topics.get_mut(topic_name) {
                m.messages = topic.len();
                m.messages_all_time += count as u64;
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
        ttl: Duration,
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
            let mut metrics = self.metrics.write().unwrap();
            metrics.subscriptions_all_time += 1;
            metrics.subscriptions.insert(
                String::from(subscription_name),
                SubscriptionMetrics::new(subscription),
            );
        }

        Some((created, SubscriptionMeta::from(&*subscription)))
    }

    pub fn update_subscription(
        &self,
        subscription_name: &str,
        ack_deadline: Option<Duration>,
        ttl: Option<Duration>,
    ) -> Option<SubscriptionMeta> {
        let mut subscriptions = self.subscriptions.write().unwrap();
        subscriptions.get_mut(subscription_name).map(|s| {
            if let Some(v) = ack_deadline {
                s.set_ack_deadline(v);
            }
            if let Some(v) = ttl {
                s.set_ttl(v);
            }

            // Ensure that updated was updated
            s.update();

            // Update metrics
            let mut metrics = self.metrics.write().unwrap();
            if let Some(m) = metrics.subscriptions.get_mut(subscription_name) {
                m.ack_deadline = s.ack_deadline.num_seconds();
                m.ttl = s.ttl.num_seconds();
            }

            SubscriptionMeta::from(&*s)
        })
    }

    pub fn delete_subscription(&self, subscription_name: &str) -> bool {
        // Update metrics
        let mut metrics = self.metrics.write().unwrap();
        metrics.subscriptions.remove(subscription_name);

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
        subscriptions.get_mut(subscription_name).map(|s| {
            let mut messages = Vec::with_capacity(max_messages);
            while let Some(message) = s.pull() {
                messages.push(message);
                if messages.len() >= max_messages {
                    break;
                }
            }

            // Update metrics
            let mut metrics = self.metrics.write().unwrap();
            if let Some(m) = metrics.subscriptions.get_mut(subscription_name) {
                m.pending = s.num_pending();
                m.pulled_all_time += messages.len() as u64;
                m.topic_message_index = s.next_index();
            }

            messages
        })
    }

    pub fn ack(&self, subscription_name: &str, ids: &[Uuid]) -> bool {
        let mut subscriptions = self.subscriptions.write().unwrap();
        subscriptions
            .get_mut(subscription_name)
            .map(|s| {
                let count = s.ack_many(ids);

                // Update metrics
                let mut metrics = self.metrics.write().unwrap();
                if let Some(m) = metrics.subscriptions.get_mut(subscription_name) {
                    m.pending = s.num_pending();
                    m.acked_all_time += count as u64;
                };

                true
            })
            .unwrap_or(false)
    }

    pub fn metrics(&self) -> Arc<RwLock<Metrics>> {
        Arc::clone(&self.metrics)
    }

    pub fn cleanup(&self) {
        let mut metrics = self.metrics.write().unwrap();

        let mut subscriptions = self.subscriptions.write().unwrap();
        subscriptions.retain(|_, s| {
            if s.ttl == Duration::seconds(0) {
                true
            } else {
                Utc::now().signed_duration_since(s.updated) <= s.ttl
            }
        });

        // Update metrics
        metrics
            .subscriptions
            .retain(|name, _| subscriptions.contains_key(name));

        let mut topics = self.topics.write().unwrap();
        topics.retain(|_, ts| {
            if ts.topic.ttl == Duration::seconds(0) {
                true
            } else {
                Utc::now().signed_duration_since(ts.topic.updated) <= ts.topic.ttl
            }
        });

        // Update metrics
        metrics.topics.retain(|name, _| topics.contains_key(name));

        for (topic_name, mut topic_store) in topics.iter_mut() {
            let count = topic_store.topic.cleanup();

            // Update metrics
            if let Some(m) = metrics.topics.get_mut(topic_name) {
                m.messages = topic_store.topic.len();
                m.expired_all_time += count as u64;
            }
        }

        //Update metrics
        let process = psutil::process::Process::new(psutil::getpid());
        metrics.memory_resident_set_size = if let Ok(process) = process {
            process.rss
        } else {
            -1
        };
    }
}
