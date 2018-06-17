#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use actix_web::{FromRequest, HttpRequest, HttpResponse, State};
use serde_json;
use std::clone::Clone;
use std::sync::Arc;

use http_protocol::HttpState;

// // static BULMA_CSS: &'static str = include_str!("../../web/css/bulma.css");
// // static DOCUMENTATION_HTML: &'static str = include_str!("../../web/documentation.html");

// pub fn documentation() -> Response<'static> {
//     Response::build()
//         .header(ContentType::HTML)
//         .sized_body(Cursor::new(""))
//         .finalize()
// }

// // #[get("/bulma.css")]
// // pub fn bulma() -> Response<'static> {
// //     Response::build()
// //         .header(ContentType::CSS)
// //         .sized_body(Cursor::new(""))
// //         .finalize()
// // }

// pub fn ui(_: HttpRequest) -> String {
//     String::from("ui")
// }

pub fn heartbeat(_: HttpRequest<HttpState>) -> &'static str {
    "heartbeat"
}

pub fn metrics(req: HttpRequest<HttpState>) -> HttpResponse {
    let state = State::extract(&req);
    let reg = Arc::clone(&state.registry);
    let metrics_wrapper = reg.metrics();
    let metrics = metrics_wrapper.read().unwrap();
    let json = serde_json::to_string(&*metrics).unwrap_or_else(|_| String::from("{}"));
    HttpResponse::Ok().body(json)
}
