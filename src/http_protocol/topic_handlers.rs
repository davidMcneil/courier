#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use chrono::Duration;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::State;
use rocket_contrib::Json;
use uuid::Uuid;

use courier::TopicMeta;

use super::types;
use super::Config;
use registry::SharedRegistry;

fn create(
    cfg: State<Config>,
    reg: State<SharedRegistry>,
    name: String,
    config: Json<types::TopicCreateConfig>,
) -> Custom<Json<TopicMeta>> {
    let config = config.into_inner();
    let ttl = config
        .message_ttl
        .map(|ttl| Duration::seconds(i64::from(ttl)))
        .unwrap_or(cfg.default_message_ttl);
    let (created, topic) = reg.create_topic(&name, ttl);
    let json = Json(topic);
    if created {
        Custom(Status::Created, json)
    } else {
        Custom(Status::Conflict, json)
    }
}

#[put("/<name>", data = "<config>")]
pub fn create_with_name(
    cfg: State<Config>,
    reg: State<SharedRegistry>,
    name: String,
    config: Json<types::TopicCreateConfig>,
) -> Custom<Json<TopicMeta>> {
    create(cfg, reg, name, config)
}

#[put("/", data = "<config>")]
pub fn create_without_name(
    cfg: State<Config>,
    reg: State<SharedRegistry>,
    config: Json<types::TopicCreateConfig>,
) -> Custom<Json<TopicMeta>> {
    create(cfg, reg, Uuid::new_v4().to_string(), config)
}

#[patch("/<name>", data = "<config>")]
pub fn update(
    reg: State<SharedRegistry>,
    name: String,
    config: Json<types::TopicUpdateConfig>,
) -> Option<Json<TopicMeta>> {
    let config = config.into_inner();
    let ttl = config
        .message_ttl
        .map(|ttl| Duration::seconds(i64::from(ttl)));
    reg.update_topic(&name, ttl).map(Json)
}

#[delete("/<name>")]
pub fn delete(reg: State<SharedRegistry>, name: String) -> Custom<()> {
    if reg.delete_topic(&name) {
        Custom(Status::Ok, ())
    } else {
        Custom(Status::NotFound, ())
    }
}

#[get("/<name>")]
pub fn get(reg: State<SharedRegistry>, name: String) -> Option<Json<TopicMeta>> {
    reg.get_topic(&name).map(Json)
}

#[get("/")]
pub fn list(reg: State<SharedRegistry>) -> Json<types::TopicList> {
    Json(types::TopicList::new(reg.list_topics()))
}

#[get("/<name>/subscriptions")]
pub fn subscriptions(
    reg: State<SharedRegistry>,
    name: String,
) -> Option<Json<types::SubscriptionNameList>> {
    reg.list_topic_subscriptions(&name)
        .map(|l| Json(types::SubscriptionNameList::new(l)))
}

#[post("/<name>/publish", data = "<messages>")]
pub fn publish(
    reg: State<SharedRegistry>,
    name: String,
    messages: Json<types::RawMessageList>,
) -> Option<Json<types::MessageIdList>> {
    reg.publish(&name, messages.into_inner().messages)
        .map(|m| Json(types::MessageIdList::new(m)))
}
