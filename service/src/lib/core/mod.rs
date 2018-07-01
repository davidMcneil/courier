//! The core pubsub module containing [Message](struct.Message.html), [Topic](struct.Topic.html),
//! and [Subscription](struct.Subscription.html).

use chrono::prelude::*;
use chrono::Duration;
use commit_log::{CommitLog, Cursor, Index};
use std::collections::HashSet;
use std::collections::VecDeque;
use uuid::Uuid;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
struct InternalMessage {
    id: Uuid,
    time: DateTime<Utc>,
    data: String,
}

impl InternalMessage {
    fn new(data: String) -> Self {
        InternalMessage {
            id: Uuid::new_v4(),
            time: Utc::now(),
            data,
        }
    }
}

impl From<String> for InternalMessage {
    fn from(data: String) -> Self {
        Self::new(data)
    }
}

impl Default for InternalMessage {
    fn default() -> Self {
        InternalMessage {
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
    tries: u32,
    index: Index<InternalMessage>,
}

impl PendingMessage {
    fn new(message_id: Uuid, tries: u32, index: Index<InternalMessage>) -> Self {
        Self {
            time_sent: Utc::now(),
            message_id,
            tries,
            index,
        }
    }
}

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

impl Message {
    /// Create a new [Message](struct.Message.html)
    pub fn new(data: String) -> Self {
        Message {
            id: Uuid::new_v4(),
            time: Utc::now(),
            tries: 0,
            data,
        }
    }
}

/// A subscription used to subscribe to a [Topic](struct.Topic.html).
#[derive(Debug)]
pub struct Subscription {
    /// Unique name for this subscription.
    pub name: String,
    /// Topic name the subscription is subscribed to.
    pub topic: String,
    /// Amount of time given to ack a message.
    pub ack_deadline: Duration,
    /// Time to live of the subscription.
    pub ttl: Duration,
    /// Time the subscription was created.
    pub created: DateTime<Utc>,
    /// Time the subscription was last updated.
    pub updated: DateTime<Utc>,
    cursor: Cursor<InternalMessage>,
    pending: VecDeque<PendingMessage>,
    pending_ids: HashSet<Uuid>,
    acked: HashSet<Uuid>,
}

impl Subscription {
    /// Create a new subscription at the head (beginning) of the [Topic](struct.Topic.html).
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

    /// Create a new subscription at the tail (end) of the [Topic](struct.Topic.html).
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

    /// Pull a message.
    ///
    /// If `pull` returns `None` there are no [Message](struct.Message.html)s to pull. This will
    /// try and return [Message](struct.Message.html)s that have reached there ack deadline, but
    /// have not been acked before pulling new messages.
    pub fn pull(&mut self) -> Option<Message> {
        self.update();

        // Check if there are any pending messages. If not, try and pull a new one from the cursor.
        let (internal_message, index, tries) = self.check_pending()
            .unwrap_or_else(|| (self.cursor.next(), Index::new(&self.cursor), 1));

        // If there is a message to send add it as a pending message
        if let Some(m) = internal_message.as_ref() {
            self.pending_ids.insert(m.id);
            self.pending
                .push_back(PendingMessage::new(m.id, tries, index));
        }

        internal_message.map(|m| Message {
            id: m.id,
            time: m.time,
            tries,
            data: m.data,
        })
    }

    /// Ack the message with `id`.
    ///
    /// Return true if the ack was successful.
    pub fn ack(&mut self, id: Uuid) -> bool {
        self.update();

        // Check if the id is a pending id. If it is not, the message either reached its ack
        // deadline or it is an invalid id.
        if self.pending_ids.remove(&id) {
            self.acked.insert(id);
            return true;
        }
        false
    }

    /// Ack many message `id`s.
    ///
    /// Returns all ids which were successfully acked.
    pub fn ack_many(&mut self, ids: &[Uuid]) -> Vec<Uuid> {
        let mut acked = Vec::with_capacity(ids.len());
        for id in ids {
            if self.ack(*id) {
                acked.push(*id);
            }
        }
        acked
    }

    /// Set the ack deadline.
    pub fn set_ack_deadline(&mut self, ack_deadline: Duration) {
        self.update();

        self.ack_deadline = ack_deadline;
    }

    /// Set the time to live.
    pub fn set_ttl(&mut self, ttl: Duration) {
        self.update();

        self.ttl = ttl;
    }

    /// Set the updated time to now.
    pub fn update(&mut self) {
        self.updated = Utc::now();
    }

    /// Get the index of the element a pull with no pending elements will retrieve with next.
    ///
    /// This assumes the cursor is pointing to a valid element if it is not the index may be much
    /// larger as the cursor is moved to the [Topic](struct.Topic.html)'s last valid
    /// [Message](struct.Message.html).
    pub fn next_index(&self) -> usize {
        self.cursor.next_index()
    }

    /// Get the number of pending messages
    pub fn num_pending(&self) -> usize {
        self.pending_ids.len()
    }

    fn check_pending(&mut self) -> Option<(Option<InternalMessage>, Index<InternalMessage>, u32)> {
        while let Some(pending) = self.pending.pop_front() {
            match pending.index.get() {
                Some(message) => {
                    // Check to see if this message was acked
                    if self.acked.remove(&message.id) {
                        continue;
                    }
                    if Utc::now().signed_duration_since(pending.time_sent) < self.ack_deadline {
                        // If the message has not yet reached its ack deadline put the message back
                        // on the front of the queue and return None as all other messages will have
                        // not timed out either.
                        self.pending.push_front(pending);
                        return None;
                    }
                    // The message ack deadline has been reached so return the message to be resent
                    // increment the number of times the message has been tried
                    return Some((Some(message), pending.index, pending.tries + 1));
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

/// A subscription meta type used for serialization.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SubscriptionMeta {
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

/// A topic which [Message](struct.Message.html)s can be published and
/// [Subscription](struct.Subscription.html) can subscribe to.
#[derive(Debug)]
pub struct Topic {
    /// Unique name of the topic.
    pub name: String,
    /// Message time to live.
    pub message_ttl: Duration,
    /// Time to live of the topic.
    pub ttl: Duration,
    /// Time the topic was created.
    pub created: DateTime<Utc>,
    /// Time the topic was updated.
    pub updated: DateTime<Utc>,
    log: CommitLog<InternalMessage>,
}

impl Topic {
    /// Create a new topic.
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

    /// Returns `true` if there are no [Message](struct.Message.html)s.
    pub fn empty(&self) -> bool {
        self.log.empty()
    }

    /// Get the number of [Message](struct.Message.html)s.
    pub fn len(&self) -> usize {
        self.log.len()
    }

    /// Publish the provided data.
    ///
    /// Data is converted to a [Message](struct.Message.html) and its id is returned.
    pub fn publish(&mut self, data: String) -> Uuid {
        // Update updated time
        self.update();

        let internal_message = InternalMessage::new(data);
        let id = internal_message.id;
        self.log.append(internal_message);
        id
    }

    /// Cleanup expired messages.
    ///
    /// Returns the number of messages cleaned up
    pub fn cleanup(&mut self) -> usize {
        let ttl = self.message_ttl;
        if ttl != Duration::seconds(0) {
            self.log
                .cleanup(&|m| Utc::now().signed_duration_since(m.time) > ttl)
        } else {
            0
        }
    }

    /// Set the message time to live
    pub fn set_message_ttl(&mut self, message_ttl: Duration) {
        // Update updated time
        self.update();

        self.message_ttl = message_ttl;
    }

    /// Set the topic time to live
    pub fn set_ttl(&mut self, ttl: Duration) {
        // Update updated time
        self.update();

        self.ttl = ttl;
    }

    /// Update the updated time to now
    pub fn update(&mut self) {
        // Update updated time
        self.updated = Utc::now();
    }
}

/// A topic meta type used for serialization.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TopicMeta {
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
