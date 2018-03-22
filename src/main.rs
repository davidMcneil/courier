extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;

extern crate qorier;

use hyper::{Response, StatusCode};
use gotham::http::response::create_response;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;

fn documentation(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("documentation").into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );
    (state, response)
}

fn ui(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("ui").into_bytes(), mime::TEXT_PLAIN)),
    );
    (state, response)
}

fn metrics(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("metrics").into_bytes(), mime::TEXT_PLAIN)),
    );
    (state, response)
}

fn put_topic(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("put_topic").into_bytes(), mime::TEXT_PLAIN)),
    );
    (state, response)
}

fn delete_topic(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("delete_topic").into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );
    (state, response)
}

fn get_topic(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("get_topic").into_bytes(), mime::TEXT_PLAIN)),
    );
    (state, response)
}

fn list_topics(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("list_topics").into_bytes(), mime::TEXT_PLAIN)),
    );
    (state, response)
}

fn list_topic_subscriptions(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("list_topic_subscriptions").into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );
    (state, response)
}

fn put_subscription(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("put_subscription").into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );
    (state, response)
}

fn delete_subscription(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("delete_subscription").into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );
    (state, response)
}

fn get_subscription(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("get_subscription").into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );
    (state, response)
}

fn list_subscriptions(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("list_subscriptions").into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );
    (state, response)
}

fn pull_subscription(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("pull_subscription").into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );
    (state, response)
}

fn acknowledge_subscription(state: State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("acknowledge_subscription").into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );
    (state, response)
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/ui").to(ui);

        route.scope("/api/v0", |route| {
            route.get("/metrics").to(metrics);

            route.scope("/topics", |route| {
                route.put("/:topic").to(put_topic);
                route.delete("/:topic").to(delete_topic);
                route.get("/:topic").to(get_topic);
                route.get("/").to(list_topics);
                route
                    .get("/:topic/subscriptions")
                    .to(list_topic_subscriptions);
            });

            route.scope("/subscriptions", |route| {
                route.put("/:subscription").to(put_subscription);
                route.delete("/:subscription").to(delete_subscription);
                route.get("/:subscription").to(get_subscription);
                route.get("/").to(list_subscriptions);
                route.get("/:subscription/pull").to(pull_subscription);
                route
                    .get("/:subscription/acknowledge")
                    .to(acknowledge_subscription);
            });
        });

        route.get("*").to(documentation);
    })
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
