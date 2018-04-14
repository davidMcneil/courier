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

mod http_json_protocol;
mod registry;

pub fn main() {
    http_json_protocol::rocket().launch();
}
