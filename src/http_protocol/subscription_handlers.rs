#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use super::types;
use super::Config;
use chrono::Duration;
use courier::SubscriptionMeta;
use registry::SharedRegistry;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::State;
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
    let subscribe = reg.create_subscription(
        &name,
        &config.topic,
        deadline,
        config.historical.unwrap_or(false),
    );
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
    reg.update_subscription(&name, deadline).map(Json)
}

#[delete("/<name>")]
pub fn delete(reg: State<SharedRegistry>, name: String) -> Custom<()> {
    if reg.delete_subscription(&name) {
        Custom(Status::Ok, ())
    } else {
        Custom(Status::NotFound, ())
    }
}

#[get("/<name>")]
pub fn get(reg: State<SharedRegistry>, name: String) -> Option<Json<SubscriptionMeta>> {
    reg.get_subscription(&name).map(Json)
}

#[get("/")]
pub fn list(reg: State<SharedRegistry>) -> Json<types::SubscriptionList> {
    Json(types::SubscriptionList::new(reg.list_subscriptions()))
}

#[post("/<name>/pull", data = "<config>")]
pub fn pull(
    cfg: State<Config>,
    reg: State<SharedRegistry>,
    name: String,
    config: Json<types::PullConfig>,
) -> Option<Json<types::MessageList>> {
    let config = config.into_inner();
    let max = config.max_messages.unwrap_or(cfg.default_max_messages);
    reg.pull(&name, max)
        .map(|messages| Json(types::MessageList::new(messages)))
}

#[post("/<name>/ack", data = "<ids>")]
pub fn ack(
    reg: State<SharedRegistry>,
    name: String,
    ids: Json<types::MessageIdList>,
) -> Custom<()> {
    if reg.ack(&name, &ids.into_inner().message_ids) {
        Custom(Status::Ok, ())
    } else {
        Custom(Status::NotFound, ())
    }
}
