mod general_handlers;
mod state;
mod subscription_handlers;
#[cfg(test)]
mod tests;
mod topic_handlers;
mod types;

use actix;
use actix_web::http::Method;
use actix_web::middleware::{cors, Logger};
use actix_web::{server, App};
use std::sync::Arc;
use std::thread;
use std::time;

pub use self::state::{Config, HttpState};
use registry::Registry;

static LOGGER_FORMAT: &'static str = "%t %Ts \"%r\" (%s %bB)";

pub fn create(config: Config) -> impl Fn() -> Vec<Box<server::HttpHandler>> {
    let registry = Registry::new();
    let registry_cleanup = Arc::clone(&registry);
    thread::spawn(move || loop {
        registry_cleanup.cleanup();
        thread::sleep(time::Duration::from_secs(1));
    });

    move || {
        vec![
            App::new()
                .prefix("/web")
                .route("/ui", Method::GET, general_handlers::html)
                .route("/src.036c3682.css", Method::GET, general_handlers::css)
                .route("/src.b423b4bf.js", Method::GET, general_handlers::js)
                .middleware(Logger::new(LOGGER_FORMAT))
                .middleware(cors::Cors::build().finish())
                .boxed(),
            App::with_state(HttpState::new(&registry, &config))
                .prefix("/api/v0")
                .route("/heartbeat", Method::GET, general_handlers::heartbeat)
                .route("/metrics", Method::GET, general_handlers::metrics)
                .scope("/topics", |scope| {
                    scope
                        .route("/{name}", Method::PUT, topic_handlers::create_with_name)
                        .route("/", Method::PUT, topic_handlers::create_without_name)
                        .route("/{name}", Method::PATCH, topic_handlers::update)
                        .route("/{name}", Method::DELETE, topic_handlers::delete)
                        .route("/{name}", Method::GET, topic_handlers::get)
                        .route("/", Method::GET, topic_handlers::list)
                        .route(
                            "/{name}/subscriptions",
                            Method::GET,
                            topic_handlers::subscriptions,
                        )
                        .route("/{name}/publish", Method::POST, topic_handlers::publish)
                })
                .scope("/subscriptions", |scope| {
                    scope
                        .route(
                            "/{name}",
                            Method::PUT,
                            subscription_handlers::create_with_name,
                        )
                        .route("/", Method::PUT, subscription_handlers::create_without_name)
                        .route("/{name}", Method::PATCH, subscription_handlers::update)
                        .route("/{name}", Method::DELETE, subscription_handlers::delete)
                        .route("/{name}", Method::GET, subscription_handlers::get)
                        .route("/", Method::GET, subscription_handlers::list)
                        .route("/{name}/pull", Method::POST, subscription_handlers::pull)
                        .route("/{name}/ack", Method::POST, subscription_handlers::ack)
                })
                .middleware(Logger::new(LOGGER_FORMAT))
                .middleware(cors::Cors::build().finish())
                .boxed(),
        ]
    }
}

pub fn start(config: Config) {
    let sys = actix::System::new("courier");
    let address = format!("{}:{}", &config.host, &config.port);
    server::new(create(config))
        .bind(address.clone())
        .expect(&format!("Can not bind to '{}'!", address))
        .start();
    sys.run();
}
