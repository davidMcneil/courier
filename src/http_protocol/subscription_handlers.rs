#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use super::Config;
use super::types;
use chrono::Duration;
use courier::SubscriptionMeta;
use registry::SharedRegistry;
use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_contrib::Json;
use uuid::Uuid;

fn create(
    cfg: State<Config>,
    reg: State<SharedRegistry>,
    name: String,
    config: Json<types::SubscriptionCreateConfig>,
) -> Option<Custom<Json<SubscriptionMeta>>> {
    let config = config.into_inner();
    let deadline = config
        .ack_deadline
        .map(|deadline| Duration::seconds(i64::from(deadline)))
        .unwrap_or(cfg.default_ack_deadline);
    let subscribe = reg.write()
        .unwrap()
        .create_subscription(&name, &config.topic, deadline);
    subscribe.map(|(created, subscription)| {
        let json = Json(subscription);
        if created {
            Custom(Status::Created, json)
        } else {
            Custom(Status::Conflict, json)
        }
    })
}

#[put("/<name>", data = "<config>")]
pub fn create_with_name(
    cfg: State<Config>,
    reg: State<SharedRegistry>,
    name: String,
    config: Json<types::SubscriptionCreateConfig>,
) -> Option<Custom<Json<SubscriptionMeta>>> {
    create(cfg, reg, name, config)
}

#[put("/", data = "<config>")]
pub fn create_without_name(
    cfg: State<Config>,
    reg: State<SharedRegistry>,
    config: Json<types::SubscriptionCreateConfig>,
) -> Option<Custom<Json<SubscriptionMeta>>> {
    create(cfg, reg, Uuid::new_v4().to_string(), config)
}

#[patch("/<name>", data = "<config>")]
pub fn update(
    reg: State<SharedRegistry>,
    name: String,
    config: Json<types::SubscriptionUpdateConfig>,
) -> Option<Json<SubscriptionMeta>> {
    let config = config.into_inner();
    let deadline = config
        .ack_deadline
        .map(|deadline| Duration::seconds(i64::from(deadline)));
    reg.write()
        .unwrap()
        .update_subscription(&name, deadline)
        .map(Json)
}

#[delete("/<name>")]
pub fn delete(reg: State<SharedRegistry>, name: String) -> Custom<()> {
    if reg.write().unwrap().delete_subscription(&name) {
        Custom(Status::Ok, ())
    } else {
        Custom(Status::NotFound, ())
    }
}

#[get("/<name>")]
pub fn get(reg: State<SharedRegistry>, name: String) -> Option<Json<SubscriptionMeta>> {
    reg.write().unwrap().get_subscription(&name).map(Json)
}

#[get("/")]
pub fn list(reg: State<SharedRegistry>) -> Json<types::SubscriptionList> {
    Json(types::SubscriptionList::new(
        reg.read().unwrap().list_subscriptions(),
    ))
}

#[post("/<name>/pull", data = "<config>")]
pub fn pull(
    reg: State<SharedRegistry>,
    name: String,
    config: Json<types::PullConfig>,
) -> Option<Json<types::MessageList>> {
    let mut reg = reg.write().unwrap();
    let config = config.into_inner();
    let messages = if config.return_immediately {
        reg.pull_immediate(&name, config.max_messages)
    } else {
        reg.pull_wait(&name, config.max_messages, Duration::seconds(5))
    };
    messages.map(|messages| Json(types::MessageList::new(messages)))
}

#[post("/<name>/ack", data = "<ids>")]
pub fn ack(
    reg: State<SharedRegistry>,
    name: String,
    ids: Json<types::MessageIdList>,
) -> Custom<()> {
    if reg.write()
        .unwrap()
        .ack(&name, &ids.into_inner().message_ids)
    {
        Custom(Status::Ok, ())
    } else {
        Custom(Status::NotFound, ())
    }
}
