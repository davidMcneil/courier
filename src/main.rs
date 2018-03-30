#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

extern crate qorier;

mod registry;
mod subscription_handlers;
mod topic_handlers;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use registry::*;

#[get("/")]
fn documentation() -> String {
    String::from("documentation")
}

#[get("/ui")]
fn ui() -> String {
    String::from("ui")
}

#[get("/metrics")]
fn metrics() -> String {
    String::from("metrics")
}

fn rocket() -> rocket::Rocket {
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

pub fn main() {
    rocket().launch();
}
