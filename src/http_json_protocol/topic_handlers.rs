#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use chrono::Duration;
use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_contrib::Json;

use courier::TopicMeta;

use super::types;
use registry::Reg;

#[put("/<topic>", data = "<topic_config>")]
fn create(
    reg: State<Reg>,
    topic: String,
    topic_config: Json<types::TopicConfig>,
) -> Custom<Json<TopicMeta>> {
    let config = topic_config.into_inner();
    let (created, topic) = reg.write()
        .unwrap()
        .create_topic(&topic, Duration::seconds(config.message_ttl));
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
fn list(reg: State<Reg>) -> Json<types::TopicList> {
    Json(types::TopicList::new(reg.read().unwrap().list_topics()))
}

#[post("/<topic>/publish", data = "<messages>")]
fn publish(
    reg: State<Reg>,
    topic: String,
    messages: Json<types::RawMessageList>,
) -> Option<Json<types::MessageIdList>> {
    reg.write()
        .unwrap()
        .publish(&topic, messages.into_inner().messages)
        .map(|m| Json(types::MessageIdList::new(m)))
}

#[get("/<topic>/subscriptions")]
fn subscriptions(reg: State<Reg>, topic: String) -> Option<Json<types::SubscriptionNameList>> {
    reg.write()
        .unwrap()
        .list_topic_subscriptions(&topic)
        .map(|l| Json(types::SubscriptionNameList::new(l)))
}
