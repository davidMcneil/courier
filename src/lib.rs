extern crate chrono;
extern crate core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use chrono::Duration;
use chrono::prelude::*;
use uuid::Uuid;

mod commit_log;

use commit_log::{CommitLog, Cursor};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    id: Uuid,
    time: DateTime<Utc>,
    data: String,
}

impl Message {
    pub fn new(data: String) -> Self {
        Message {
            id: Uuid::new_v4(),
            time: Utc::now(),
            data: data,
        }
    }
}

impl Default for Message {
    fn default() -> Self {
        Message {
            id: Default::default(),
            time: Utc::now(),
            data: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct Subscription {
    name: String,
    cursor: Cursor<Message>,
}

impl Subscription {
    pub fn new(name: String, topic: &Topic) -> Subscription {
        topic.subscribe(name)
    }

    pub fn pull(&mut self) -> Option<Message> {
        self.cursor.get_copy()
    }
}

#[derive(Debug)]
pub struct Topic {
    name: String,
    ttl: Duration,
    log: CommitLog<Message>,
}

impl Topic {
    pub fn new(name: String, ttl: Duration) -> Topic {
        Topic {
            name: name,
            ttl: ttl,
            log: CommitLog::new(),
        }
    }

    pub fn publish(&mut self, message: Message) {
        self.log.append(message);
    }

    pub fn subscribe(&self, name: String) -> Subscription {
        Subscription {
            name: name,
            cursor: Cursor::new_head(&self.log),
        }
    }

    pub fn cleanup(&mut self) {
        let ttl = self.ttl;
        self.log
            .cleanup(&|m| Utc::now().signed_duration_since(m.time) > ttl);
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
