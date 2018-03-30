use chrono::Duration;
use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_contrib::Json;
use uuid::Uuid;

use qorier::{RawMessage, TopicMeta};

use registry::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TopicConfig {
    message_ttl: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TopicList {
    topics: Vec<TopicMeta>,
}

impl TopicList {
    pub fn new(topics: Vec<TopicMeta>) -> Self {
        Self { topics }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawMessageList {
    messages: Vec<RawMessage>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageIdList {
    message_ids: Vec<Uuid>,
}

impl MessageIdList {
    pub fn new(message_ids: Vec<Uuid>) -> Self {
        Self { message_ids }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionList {
    subscriptions: Vec<String>,
}

impl SubscriptionList {
    pub fn new(subscriptions: Vec<String>) -> Self {
        Self { subscriptions }
    }
}

#[put("/<topic>", data = "<topic_config>")]
fn create(
    reg: State<Reg>,
    topic: String,
    topic_config: Json<TopicConfig>,
) -> Custom<Json<TopicMeta>> {
    let config = topic_config.into_inner();
    let (created, topic) = reg.write()
        .unwrap()
        .create_topic(&topic, &Duration::seconds(config.message_ttl));
    let json = Json(topic);
    if created {
        Custom(Status::Created, json)
    } else {
        Custom(Status::AlreadyReported, json)
    }
}

#[delete("/<topic>")]
fn delete(reg: State<Reg>, topic: String) -> Custom<()> {
    if reg.write().unwrap().delete_topic(&topic) {
        Custom(Status::Ok, ())
    } else {
        Custom(Status::NotFound, ())
    }
}

#[get("/<topic>")]
fn get(reg: State<Reg>, topic: String) -> Option<Json<TopicMeta>> {
    reg.write().unwrap().get_topic(&topic).map(Json)
}

#[get("/")]
fn list(reg: State<Reg>) -> Json<TopicList> {
    Json(TopicList::new(reg.read().unwrap().list_topics()))
}

#[post("/<topic>/publish", data = "<messages>")]
fn publish(
    reg: State<Reg>,
    topic: String,
    messages: Json<RawMessageList>,
) -> Option<Json<MessageIdList>> {
    reg.write()
        .unwrap()
        .publish(&topic, messages.into_inner().messages)
        .map(|m| Json(MessageIdList::new(m)))
}

#[get("/<topic>/subscriptions")]
fn subscriptions(reg: State<Reg>, topic: String) -> Option<Json<SubscriptionList>> {
    reg.write()
        .unwrap()
        .list_topic_subscriptions(&topic)
        .map(|l| Json(SubscriptionList::new(l)))
}
