#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use actix_web::{FromRequest, HttpRequest, HttpResponse, State};
use serde_json;
use std::clone::Clone;
use std::sync::Arc;

use crate::http_protocol::HttpState;

pub fn heartbeat(_: HttpRequest<HttpState>) -> &'static str {
    "heartbeat"
}

pub fn metrics(req: HttpRequest<HttpState>) -> HttpResponse {
    let state = State::extract(&req);
    let reg = Arc::clone(&state.registry);
    let metrics_wrapper = reg.metrics();
    let metrics = metrics_wrapper.read();
    let json = serde_json::to_string(&*metrics).unwrap_or_else(|_| String::from("{}"));
    HttpResponse::Ok().body(json)
}
