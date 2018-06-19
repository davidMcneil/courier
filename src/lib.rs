extern crate chrono;
extern crate core;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use chrono::prelude::*;
use chrono::Duration;
use commit_log::{CommitLog, Cursor, Index};
use std::collections::HashSet;
use std::collections::VecDeque;
use uuid::Uuid;

mod commit_log;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RawMessage {
    pub data: String,
}

impl RawMessage {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
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
    message_id: Uuid,
    index: Index<Message>,
}

impl PendingMessage {
    fn new(message_id: Uuid, index: Index<Message>) -> Self {
        Self {
            time_sent: Utc::now(),
            message_id,
            index,
        }
    }
}

#[derive(Debug)]
pub struct Subscription {
    pub name: String,
    pub topic: String,
    pub ack_deadline: Duration,
    pub ttl: Duration,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    cursor: Cursor<Message>,
    pending: VecDeque<PendingMessage>,
    pending_ids: HashSet<Uuid>,
    acked: HashSet<Uuid>,
}

impl Subscription {
    pub fn new_head(name: &str, topic: &Topic, ack_deadline: Duration, ttl: Duration) -> Self {
        let now = Utc::now();
        Self {
            name: String::from(name),
            topic: topic.name.clone(),
            ack_deadline,
            ttl,
            created: now,
            updated: now,
            cursor: Cursor::new_head(&topic.log),
            pending: VecDeque::new(),
            pending_ids: HashSet::new(),
            acked: HashSet::new(),
        }
    }

    pub fn new_tail(name: &str, topic: &Topic, ack_deadline: Duration, ttl: Duration) -> Self {
        let now = Utc::now();
        Self {
            name: String::from(name),
            topic: topic.name.clone(),
            ack_deadline,
            ttl,
            created: now,
            updated: now,
            cursor: Cursor::new_tail(&topic.log),
            pending: VecDeque::new(),
            pending_ids: HashSet::new(),
            acked: HashSet::new(),
        }
    }

    pub fn pull(&mut self) -> Option<Message> {
        // Update updated time
        self.update();

        let (message, index) = self
            .check_pending()
            .unwrap_or_else(|| (self.cursor.next(), Index::new(&self.cursor)));
        if let Some(m) = message.as_ref() {
            self.pending_ids.insert(m.id);
            self.pending.push_back(PendingMessage::new(m.id, index));
        }
        message
    }

    pub fn ack(&mut self, id: Uuid) -> bool {
        // Update updated time
        self.update();

        if self.pending_ids.remove(&id) {
            self.acked.insert(id);
            return true;
        }
        false
    }

    pub fn ack_many(&mut self, ids: &[Uuid]) -> usize {
        let mut count = 0;
        for id in ids {
            if self.ack(*id) {
                count += 1;
            }
        }
        count
    }

    pub fn set_ack_deadline(&mut self, ack_deadline: Duration) {
        // Update updated time
        self.update();

        self.ack_deadline = ack_deadline;
    }

    pub fn set_ttl(&mut self, ttl: Duration) {
        // Update updated time
        self.update();

        self.ttl = ttl;
    }

    pub fn update(&mut self) {
        // Update updated time
        self.updated = Utc::now();
    }

    pub fn next_index(&self) -> usize {
        self.cursor.next_index()
    }

    pub fn num_pending(&self) -> usize {
        self.pending_ids.len()
    }

    fn check_pending(&mut self) -> Option<(Option<Message>, Index<Message>)> {
        while let Some(pending) = self.pending.pop_front() {
            match pending.index.get() {
                Some(message) => {
                    // Check to see if this message was acked
                    if self.acked.remove(&message.id) {
                        continue;
                    }
                    if Utc::now().signed_duration_since(pending.time_sent) < self.ack_deadline {
                        // If the message has not timed out yet put the message back on the front
                        // of the queue
                        self.pending.push_front(pending);
                        return None;
                    }
                    // The message ack deadline timed out so return that message to be resent
                    return Some((Some(message), pending.index));
                }
                // The message has timed out in the topic
                None => {
                    // Cleanup the message from pending and acked
                    self.pending_ids.remove(&pending.message_id);
                    self.acked.remove(&pending.message_id);
                    continue;
                }
            }
        }
        // There are no pending messages
        None
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SubscriptionMeta {
    pub name: String,
    pub topic: String,
    pub ack_deadline: i64,
    pub ttl: i64,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl<'a> From<&'a Subscription> for SubscriptionMeta {
    fn from(subscription: &'a Subscription) -> Self {
        Self {
            name: subscription.name.clone(),
            topic: subscription.topic.clone(),
            ack_deadline: subscription.ack_deadline.num_seconds(),
            ttl: subscription.ttl.num_seconds(),
            created: subscription.created,
            updated: subscription.updated,
        }
    }
}

#[derive(Debug)]
pub struct Topic {
    pub name: String,
    pub message_ttl: Duration,
    pub ttl: Duration,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    log: CommitLog<Message>,
}

impl Topic {
    pub fn new(name: &str, message_ttl: Duration, ttl: Duration) -> Topic {
        let now = Utc::now();
        Topic {
            name: String::from(name),
            message_ttl,
            ttl,
            created: now,
            updated: now,
            log: CommitLog::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.log.len()
    }

    pub fn publish(&mut self, message: Message) {
        // Update updated time
        self.update();

        self.log.append(message);
    }

    pub fn cleanup(&mut self) -> usize {
        let ttl = self.message_ttl;
        if ttl != Duration::seconds(0) {
            self.log
                .cleanup(&|m| Utc::now().signed_duration_since(m.time) > ttl)
        } else {
            0
        }
    }

    pub fn set_message_ttl(&mut self, message_ttl: Duration) {
        // Update updated time
        self.update();

        self.message_ttl = message_ttl;
    }

    pub fn set_ttl(&mut self, ttl: Duration) {
        // Update updated time
        self.update();

        self.ttl = ttl;
    }

    pub fn update(&mut self) {
        // Update updated time
        self.updated = Utc::now();
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TopicMeta {
    pub name: String,
    pub message_ttl: i64,
    pub ttl: i64,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl<'a> From<&'a Topic> for TopicMeta {
    fn from(topic: &'a Topic) -> Self {
        Self {
            name: topic.name.clone(),
            message_ttl: topic.message_ttl.num_seconds(),
            ttl: topic.ttl.num_seconds(),
            created: topic.created,
            updated: topic.updated,
        }
    }
}
