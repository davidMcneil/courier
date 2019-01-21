#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use crate::http_protocol::state::HttpState;
use crate::http_protocol::types;
use crate::http_protocol::Config;
use actix_web::dev::HttpResponseBuilder;
use actix_web::{HttpResponse, Json, Path, State};
use chrono::Duration;
use courier::SharedRegistry;
use courier::TopicMeta;
use uuid::Uuid;

fn create(
    name: &str,
    config: &types::TopicCreateConfig,
    reg: &SharedRegistry,
    cfg: &Config,
) -> HttpResponse {
    let message_ttl = config
        .message_ttl
        .map(|ttl| Duration::seconds(i64::from(ttl)))
        .unwrap_or(cfg.default_message_ttl);
    let ttl = config
        .ttl
        .map(|ttl| Duration::seconds(i64::from(ttl)))
        .unwrap_or(cfg.default_topic_ttl);
    let (created, topic) = reg.create_topic(&name, message_ttl, ttl);
    let mut response = if created {
        HttpResponse::Created()
    } else {
        HttpResponse::Conflict()
    };
    response.json(topic)
}

pub fn create_with_name(
    (name, config, state): (
        Path<String>,
        Json<types::TopicCreateConfig>,
        State<HttpState>,
    ),
) -> HttpResponse {
    create(
        &name.into_inner(),
        &config.into_inner(),
        &state.registry,
        &state.config,
    )
}

pub fn create_without_name(
    (config, state): (Json<types::TopicCreateConfig>, State<HttpState>),
) -> HttpResponse {
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
        Json<types::TopicUpdateConfig>,
        State<HttpState>,
    ),
) -> Option<Json<TopicMeta>> {
    let config = config.into_inner();
    let message_ttl = config
        .message_ttl
        .map(|message_ttl| Duration::seconds(i64::from(message_ttl)));
    let ttl = config.ttl.map(|ttl| Duration::seconds(i64::from(ttl)));
    state
        .registry
        .update_topic(&name, message_ttl, ttl)
        .map(Json)
}

pub fn delete((name, state): (Path<String>, State<HttpState>)) -> HttpResponseBuilder {
    if state.registry.delete_topic(&name) {
        HttpResponse::Ok()
    } else {
        HttpResponse::NotFound()
    }
}

pub fn get((name, state): (Path<String>, State<HttpState>)) -> Option<Json<TopicMeta>> {
    state.registry.get_topic(&name).map(Json)
}

pub fn list(state: State<HttpState>) -> Json<types::TopicList> {
    Json(types::TopicList::new(state.registry.list_topics()))
}

pub fn subscriptions(
    (name, state): (Path<String>, State<HttpState>),
) -> Option<Json<types::SubscriptionNameList>> {
    let reg = &state.registry;
    reg.list_topic_subscriptions(&name)
        .map(|l| Json(types::SubscriptionNameList::new(l)))
}

pub fn publish(
    (name, messages, state): (Path<String>, Json<types::RawMessageList>, State<HttpState>),
) -> Option<Json<types::MessageIdList>> {
    let reg = &state.registry;
    let data = messages
        .into_inner()
        .raw_messages
        .into_iter()
        .map(|m| m.data)
        .collect();
    reg.publish(&name, data)
        .map(|m| Json(types::MessageIdList::new(m)))
}
