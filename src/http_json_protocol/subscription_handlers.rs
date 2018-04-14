#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use super::types;
use chrono::Duration;
use courier::SubscriptionMeta;
use registry::Reg;
use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_contrib::Json;

#[put("/<subscription>", data = "<subscription_config>")]
fn create(
    reg: State<Reg>,
    subscription: String,
    subscription_config: Json<types::SubscriptionConfig>,
) -> Option<Custom<Json<SubscriptionMeta>>> {
    let config = subscription_config.into_inner();
    let subscribe = reg.write().unwrap().create_subscription(
        &subscription,
        &config.topic,
        Duration::seconds(config.ack_deadline),
    );
    subscribe.map(|(created, subscription)| {
        let json = Json(subscription);
        if created {
            Custom(Status::Created, json)
        } else {
            Custom(Status::AlreadyReported, json)
        }
    })
}

#[delete("/<subscription>")]
fn delete(reg: State<Reg>, subscription: String) -> Custom<()> {
    if reg.write().unwrap().delete_subscription(&subscription) {
        Custom(Status::Ok, ())
    } else {
        Custom(Status::NotFound, ())
    }
}

#[get("/<subscription>")]
fn get(reg: State<Reg>, subscription: String) -> Option<Json<SubscriptionMeta>> {
    reg.write()
        .unwrap()
        .get_subscription(&subscription)
        .map(Json)
}

#[get("/")]
fn list(reg: State<Reg>) -> Json<types::SubscriptionList> {
    Json(types::SubscriptionList::new(
        reg.read().unwrap().list_subscriptions(),
    ))
}

#[post("/<subscription>/pull")]
fn pull(reg: State<Reg>, subscription: String) -> Option<Json<types::MessageList>> {
    reg.write()
        .unwrap()
        .pull(&subscription)
        .map(|messages| Json(types::MessageList::new(messages)))
}

#[post("/<subscription>/ack", data = "<ids>")]
fn ack(reg: State<Reg>, subscription: String, ids: Json<types::MessageIdList>) -> Custom<()> {
    if reg.write()
        .unwrap()
        .ack(&subscription, &ids.into_inner().message_ids)
    {
        Custom(Status::Ok, ())
    } else {
        Custom(Status::NotFound, ())
    }
}
