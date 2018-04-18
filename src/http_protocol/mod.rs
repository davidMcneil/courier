mod general_handlers;
mod subscription_handlers;
#[cfg(test)]
mod tests;
mod topic_handlers;
mod types;

use chrono::Duration;
use rocket;
use std::sync::Arc;
use std::thread;
use std::time;

use registry::Registry;

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub default_message_ttl: Duration,
    pub default_ack_deadline: Duration,
    pub default_return_immediately: bool,
    pub default_max_messages: usize,
    pub cleanup_interval: Duration,
    pub max_pull_wait: Duration,
}

pub fn rocket(config: Config) -> rocket::Rocket {
    let api_root = "/api/v0";
    let registry = Registry::new();
    let registry_cleanup = Arc::clone(&registry);
    thread::spawn(move || loop {
        registry_cleanup.write().unwrap().cleanup();
        thread::sleep(time::Duration::from_secs(1));
    });
    rocket::ignite()
        .mount(
            "/",
            routes![general_handlers::documentation, general_handlers::ui],
        )
        .mount("/css", routes![general_handlers::bulma])
        .mount(&format!("{}", api_root), routes![general_handlers::metrics])
        .mount(
            &format!("{}/topics", api_root),
            routes![
                topic_handlers::create_with_name,
                topic_handlers::create_without_name,
                topic_handlers::update,
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
                subscription_handlers::create_with_name,
                subscription_handlers::create_without_name,
                subscription_handlers::update,
                subscription_handlers::delete,
                subscription_handlers::get,
                subscription_handlers::list,
                subscription_handlers::pull,
                subscription_handlers::ack,
            ],
        )
        .manage(config)
        .manage(registry)
}
