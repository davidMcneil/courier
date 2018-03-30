#![feature(custom_attribute)]

extern crate chrono;
extern crate core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

mod commit_log;

use chrono::Duration;
use chrono::prelude::*;
use std::collections::HashSet;
use std::collections::VecDeque;
use uuid::Uuid;

use commit_log::{CommitLog, Cursor, Index};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawMessage {
    pub data: String,
}

impl RawMessage {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub id: Uuid,
    pub time: DateTime<Utc>,
    pub data: String,
}

impl Message {
    pub fn new(data: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            time: Utc::now(),
            data,
        }
    }
}

impl From<RawMessage> for Message {
    fn from(raw: RawMessage) -> Self {
        Self::new(raw.data)
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            id: Default::default(),
            time: Utc::now(),
            data: Default::default(),
        }
    }
}

#[derive(Debug)]
struct PendingMessage {
    time_sent: DateTime<Utc>,
    index: Index<Message>,
}

impl PendingMessage {
    fn new(index: Index<Message>) -> Self {
        Self {
            time_sent: Utc::now(),
            index,
        }
    }
}

#[derive(Debug)]
pub struct Subscription {
    pub name: String,
    pub topic: String,
    pub ack_deadline: Duration,
    cursor: Cursor<Message>,
    pending: VecDeque<PendingMessage>,
    acked: HashSet<Uuid>,
}

impl Subscription {
    pub fn new(name: &str, topic: &Topic, ack_deadline: &Duration) -> Self {
        Self {
            name: String::from(name),
            ack_deadline: *ack_deadline,
            topic: topic.name.clone(),
            cursor: Cursor::new_head(&topic.log),
            pending: VecDeque::new(),
            acked: HashSet::new(),
        }
    }

    pub fn pull(&mut self) -> Option<Message> {
        let (message, index) = self.check_pending()
            .unwrap_or_else(|| (self.cursor.get_copy(), self.cursor.new_index()));
        if message.is_some() {
            self.pending.push_back(PendingMessage::new(index));
        }
        message
    }

    pub fn ack(&mut self, ids: &[Uuid]) {
        for id in ids {
            self.acked.insert(*id);
        }
    }

    fn check_pending(&mut self) -> Option<(Option<Message>, Index<Message>)> {
        while let Some(pending) = self.pending.pop_front() {
            match pending.index.get_copy() {
                Some(message) => {
                    if self.acked.contains(&message.id) {
                        self.acked.remove(&message.id);
                        continue;
                    }
                    if Utc::now().signed_duration_since(pending.time_sent) < self.ack_deadline {
                        // If the message has not timed out yet put the message back on the front
                        // of the queue
                        self.pending.push_front(pending);
                        return None;
                    }
                    return Some((Some(message), pending.index));
                }
                None => continue,
            }
        }
        None
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionMeta {
    pub name: String,
    pub topic: String,
    pub ack_deadline: i64,
}

impl<'a> From<&'a Subscription> for SubscriptionMeta {
    fn from(subscription: &'a Subscription) -> Self {
        Self {
            name: subscription.name.clone(),
            topic: subscription.topic.clone(),
            ack_deadline: subscription.ack_deadline.num_seconds(),
        }
    }
}

#[derive(Debug)]
pub struct Topic {
    pub name: String,
    pub message_ttl: Duration,
    log: CommitLog<Message>,
}

impl Topic {
    pub fn new(name: &str, message_ttl: &Duration) -> Topic {
        Topic {
            name: String::from(name),
            message_ttl: *message_ttl,
            log: CommitLog::new(),
        }
    }

    pub fn publish(&mut self, message: Message) {
        self.log.append(message);
    }

    pub fn subscribe(&self, name: &str, ack_deadline: &Duration) -> Subscription {
        Subscription::new(name, self, ack_deadline)
    }

    pub fn cleanup(&mut self) {
        let ttl = self.message_ttl;
        self.log.cleanup(&|m| {
            println!(
                "{}, {}",
                Utc::now().signed_duration_since(m.time).num_seconds(),
                ttl.num_seconds()
            );
            Utc::now().signed_duration_since(m.time) > ttl
        });
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TopicMeta {
    pub name: String,
    pub message_ttl: i64,
}

impl<'a> From<&'a Topic> for TopicMeta {
    fn from(topic: &'a Topic) -> Self {
        Self {
            name: topic.name.clone(),
            message_ttl: topic.message_ttl.num_seconds(),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test00() {
        assert_eq!(2 + 2, 4);
    }
}
