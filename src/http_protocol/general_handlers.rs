#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use actix_web::{http, FromRequest, HttpRequest, HttpResponse, State};
use serde_json;
use std::clone::Clone;
use std::sync::Arc;

use http_protocol::HttpState;

static HTML: &'static str = include_str!("../../web/dist/index.html");
static CSS: &'static str = include_str!("../../web/dist/src.036c3682.css");
static JS: &'static str = include_str!("../../web/dist/src.b423b4bf.js");

pub fn html(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "text/html")
        .body(HTML)
}

pub fn css(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "text/css")
        .body(CSS)
}

pub fn js(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/javascript")
        .body(JS)
}

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
