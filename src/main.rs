#![feature(plugin, proc_macro, decl_macro)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate uuid;

extern crate courier;

mod http_protocol;
mod registry;

use chrono::Duration;

pub fn main() {
    let config = http_protocol::Config {
        default_message_ttl: Duration::seconds(3600),
        default_ack_deadline: Duration::seconds(60),
        default_return_immediately: false,
        default_max_messages: 1,
        cleanup_interval: Duration::seconds(1),
        max_pull_wait: Duration::seconds(5),
    };
    http_protocol::rocket(config).launch();
}
