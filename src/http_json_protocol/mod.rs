mod subscription_handlers;
#[cfg(test)]
mod tests;
mod topic_handlers;
mod types;

use rocket;
use rocket::http::ContentType;
use rocket::response::Response;
use std::io::Cursor;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use registry::Registry;

// static BULMA_CSS: &'static str = include_str!("../../web/css/bulma.css");
// static DOCUMENTATION_HTML: &'static str = include_str!("../../web/documentation.html");

#[get("/")]
fn documentation() -> Response<'static> {
    Response::build()
        .header(ContentType::HTML)
        .sized_body(Cursor::new(""))
        .finalize()
}

#[get("/bulma.css")]
fn bulma() -> Response<'static> {
    Response::build()
        .header(ContentType::CSS)
        .sized_body(Cursor::new(""))
        .finalize()
}

#[get("/ui")]
fn ui() -> String {
    String::from("ui")
}

#[get("/metrics")]
fn metrics() -> String {
    String::from("metrics")
}

pub fn rocket() -> rocket::Rocket {
    let api_root = "/api/v0";
    let registry = Registry::new();
    let registry_cleanup = Arc::clone(&registry);
    thread::spawn(move || loop {
        println!("cleanup");
        registry_cleanup.write().unwrap().cleanup();
        thread::sleep(Duration::from_secs(1));
    });
    rocket::ignite()
        .mount("/", routes![documentation, ui])
        .mount("/css", routes![bulma])
        .mount(&format!("{}", api_root), routes![metrics])
        .mount(
            &format!("{}/topics", api_root),
            routes![
                topic_handlers::create,
                topic_handlers::delete,
                topic_handlers::get,
                topic_handlers::list,
                topic_handlers::publish,
                topic_handlers::subscriptions,
            ],
        )
        .mount(
            &format!("{}/subscriptions", api_root),
            routes![
                subscription_handlers::create,
                subscription_handlers::delete,
                subscription_handlers::get,
                subscription_handlers::list,
                subscription_handlers::pull,
                subscription_handlers::ack,
            ],
        )
        .manage(registry)
}
