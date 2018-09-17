mod general_handlers;
mod state;
mod subscription_handlers;
#[cfg(test)]
mod tests;
mod topic_handlers;
mod types;

use actix;
use actix_web::http::{header, Method, NormalizePath};
use actix_web::middleware::{cors, Logger};
use actix_web::{server, App, HttpRequest, HttpResponse};
use include_dir::Dir;
use mime_guess::guess_mime_type;
use std::sync::Arc;
use std::thread;
use std::time;

pub use self::state::{Config, HttpState};
use courier::Registry;

const WEB: Dir = include_dir!("../web/dist");

static LOGGER_FORMAT: &'static str = "%a \"%r\" (%s %Ts %bB)";

pub fn create(
    config: Config,
) -> impl Fn() -> Vec<Box<server::HttpHandler<Task = Box<server::HttpHandlerTask>>>> {
    let registry = Registry::new();
    let registry_cleanup = Arc::clone(&registry);

    let cleanup_interval = match config.cleanup_interval.to_std() {
        Ok(duration) => duration,
        _ => time::Duration::from_secs(0),
    };

    thread::spawn(move || loop {
        let (topics_removed, subscriptions_removed, messages_removed) = registry_cleanup.cleanup();
        debug!(
            "Removed '{}' topics, '{}' subscriptions, '{}' messages ",
            topics_removed, subscriptions_removed, messages_removed
        );

        thread::sleep(cleanup_interval)
    });

    move || {
        let mut web_app = App::new().prefix("/ui");
        // Add the static files to the web app
        for file in WEB.files() {
            let path = file.path();
            let path_str = path.to_string_lossy();
            let handler = move |_: HttpRequest| {
                let mime = guess_mime_type(path);
                let mime_str = format!("{}/{}", mime.type_(), mime.subtype());
                HttpResponse::Ok()
                    .header(header::CONTENT_TYPE, mime_str)
                    .body(file.contents())
            };
            web_app = web_app.route(&path_str, Method::GET, handler);
        }
        // Register the index route last
        let file = WEB.get_file("index.html")
            .expect("no index.html file found");
        web_app = web_app
            .route("{tail:.*}", Method::GET, move |_: HttpRequest| {
                HttpResponse::Ok()
                    .header(header::CONTENT_TYPE, "text/html")
                    .body(file.contents())
            })
            .default_resource(|r| r.method(Method::GET).h(NormalizePath::default()));
        vec![
            web_app
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
        .shutdown_timeout(30)
        .start();
    sys.run();
}
