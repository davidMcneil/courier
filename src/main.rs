#![feature(plugin, proc_macro, decl_macro)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate structopt;
extern crate uuid;

extern crate courier;

mod http_protocol;
mod registry;

use chrono::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt()]
struct Opt {
    /// An IP address or host the application will listen on
    #[structopt(default_value = "0.0.0.0", long = "host", short = "H", env = "COURIER_HOST")]
    host: String,
    /// A port number to listen on
    #[structopt(default_value = "3140", long = "port", short = "P", env = "COURIER_PORT")]
    port: u16,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt()]
    /// Run the service
    #[structopt(name = "run")]
    Run {},
}

pub fn main() {
    let opt = Opt::from_args();
    let config = http_protocol::Config {
        host: opt.host.clone(),
        port: opt.port,
        default_message_ttl: Duration::seconds(3600),
        default_ack_deadline: Duration::seconds(60),
        default_return_immediately: false,
        default_max_messages: 1,
        cleanup_interval: Duration::seconds(1),
        max_pull_wait: Duration::seconds(5),
    };
    match opt.cmd {
        Command::Run {} => {
            http_protocol::rocket(config).launch();
        }
    }
}
