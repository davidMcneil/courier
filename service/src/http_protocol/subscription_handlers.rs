#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use crate::http_protocol::state::HttpState;
use crate::http_protocol::types;
use crate::http_protocol::Config;
use actix_web::dev::HttpResponseBuilder;
use actix_web::{HttpResponse, Json, Path, State};
use chrono::Duration;
use uuid::Uuid;

use courier::SharedRegistry;
use courier::SubscriptionMeta;

fn create(
    name: &str,
    config: &types::SubscriptionCreateConfig,
    reg: &SharedRegistry,
    cfg: &Config,
) -> Option<HttpResponse> {
    let ack_deadline = config
        .ack_deadline
        .map(|ack_deadline| Duration::seconds(i64::from(ack_deadline)))
        .unwrap_or(cfg.default_ack_deadline);
    let ttl = config
        .ttl
        .map(|ttl| Duration::seconds(i64::from(ttl)))
        .unwrap_or(cfg.default_subscription_ttl);
    let subscribe = reg.create_subscription(
        &name,
        &config.topic,
        ack_deadline,
        ttl,
        config.historical.unwrap_or(false),
    );
    subscribe.map(|(created, subscription)| {
        let mut response = if created {
            HttpResponse::Created()
        } else {
            HttpResponse::Conflict()
        };
        response.json(subscription)
    })
}

pub fn create_with_name(
    (name, config, state): (
        Path<String>,
        Json<types::SubscriptionCreateConfig>,
        State<HttpState>,
    ),
) -> Option<HttpResponse> {
    create(
        &name.into_inner(),
        &config.into_inner(),
        &state.registry,
        &state.config,
    )
}

pub fn create_without_name(
    (config, state): (Json<types::SubscriptionCreateConfig>, State<HttpState>),
) -> Option<HttpResponse> {
    create(
        &Uuid::new_v4().to_string(),
        &config.into_inner(),
        &state.registry,
        &state.config,
    )
}

pub fn update(
    (name, config, state): (
        Path<String>,
        Json<types::SubscriptionUpdateConfig>,
        State<HttpState>,
    ),
) -> Option<Json<SubscriptionMeta>> {
    let reg = &state.registry;
    let config = config.into_inner();
    let ack_deadline = config
        .ack_deadline
        .map(|ack_deadline| Duration::seconds(i64::from(ack_deadline)));
    let ttl = config.ttl.map(|ttl| Duration::seconds(i64::from(ttl)));
    reg.update_subscription(&name, ack_deadline, ttl).map(Json)
}

pub fn delete((name, state): (Path<String>, State<HttpState>)) -> HttpResponseBuilder {
    if state.registry.delete_subscription(&name) {
        HttpResponse::Ok()
    } else {
        HttpResponse::NotFound()
    }
}

pub fn get((name, state): (Path<String>, State<HttpState>)) -> Option<Json<SubscriptionMeta>> {
    state.registry.get_subscription(&name).map(Json)
}

pub fn list(state: State<HttpState>) -> Json<types::SubscriptionList> {
    Json(types::SubscriptionList::new(
        state.registry.list_subscriptions(),
    ))
}

pub fn pull(
    (name, config, state): (Path<String>, Json<types::PullConfig>, State<HttpState>),
) -> Option<Json<types::MessageList>> {
    let config = config.into_inner();
    let reg = &state.registry;
    let cfg = &state.config;
    let max = config.max_messages.unwrap_or(cfg.default_max_messages);
    reg.pull(&name, max)
        .map(|messages| Json(types::MessageList::new(messages)))
}

pub fn ack(
    (name, ids, state): (Path<String>, Json<types::MessageIdList>, State<HttpState>),
) -> Option<Json<types::MessageIdList>> {
    let reg = &state.registry;
    reg.ack(&name, &ids.into_inner().message_ids)
        .map(|ids| Json(types::MessageIdList::new(ids)))
}
