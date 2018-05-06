use registry::SharedRegistry;
use rocket::http::ContentType;
use rocket::response::content;
use rocket::response::Response;
use rocket::State;
use serde_json;
use std::io::Cursor;

// static BULMA_CSS: &'static str = include_str!("../../web/css/bulma.css");
// static DOCUMENTATION_HTML: &'static str = include_str!("../../web/documentation.html");

#[get("/")]
pub fn documentation() -> Response<'static> {
    Response::build()
        .header(ContentType::HTML)
        .sized_body(Cursor::new(""))
        .finalize()
}

// #[get("/bulma.css")]
// pub fn bulma() -> Response<'static> {
//     Response::build()
//         .header(ContentType::CSS)
//         .sized_body(Cursor::new(""))
//         .finalize()
// }

#[get("/ui")]
pub fn ui() -> String {
    String::from("ui")
}

#[get("/metrics")]
pub fn metrics(reg: State<SharedRegistry>) -> content::Json<String> {
    let metrics_wrapper = reg.metrics();
    let metrics = metrics_wrapper.read().unwrap();
    let json = serde_json::to_string(&*metrics).unwrap_or(String::from("{}"));
    content::Json(json)
}
