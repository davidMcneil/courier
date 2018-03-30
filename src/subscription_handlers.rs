use chrono::Duration;
use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_contrib::Json;
use uuid::Uuid;

use qorier::{Message, SubscriptionMeta};

use registry::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionConfig {
    topic: String,
    ack_deadline: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionList {
    subscriptions: Vec<SubscriptionMeta>,
}

impl SubscriptionList {
    pub fn new(subscriptions: Vec<SubscriptionMeta>) -> Self {
        Self { subscriptions }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageList {
    messages: Vec<Message>,
}

impl MessageList {
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct MessageIdList {
    pub message_ids: Vec<Uuid>,
}

#[put("/<subscription>", data = "<subscription_config>")]
fn create(
    reg: State<Reg>,
    subscription: String,
    subscription_config: Json<SubscriptionConfig>,
) -> Option<Custom<Json<SubscriptionMeta>>> {
    let config = subscription_config.into_inner();
    let subscribe = reg.write().unwrap().create_subscription(
        &subscription,
        &config.topic,
        &Duration::seconds(config.ack_deadline),
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
fn list(reg: State<Reg>) -> Json<SubscriptionList> {
    Json(SubscriptionList::new(
        reg.read().unwrap().list_subscriptions(),
    ))
}

#[post("/<subscription>/pull")]
fn pull(reg: State<Reg>, subscription: String) -> Option<Json<MessageList>> {
    reg.write()
        .unwrap()
        .pull(&subscription)
        .map(|messages| Json(MessageList::new(messages)))
}

#[post("/<subscription>/ack", data = "<ids>")]
fn ack(reg: State<Reg>, subscription: String, ids: Json<MessageIdList>) -> Custom<()> {
    if reg.write()
        .unwrap()
        .ack(&subscription, &ids.into_inner().message_ids)
    {
        Custom(Status::Ok, ())
    } else {
        Custom(Status::NotFound, ())
    }
}
