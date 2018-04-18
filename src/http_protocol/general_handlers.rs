use rocket::http::ContentType;
use rocket::response::Response;
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

#[get("/bulma.css")]
pub fn bulma() -> Response<'static> {
    Response::build()
        .header(ContentType::CSS)
        .sized_body(Cursor::new(""))
        .finalize()
}

#[get("/ui")]
pub fn ui() -> String {
    String::from("ui")
}

#[get("/metrics")]
pub fn metrics() -> String {
    String::from("metrics")
}
