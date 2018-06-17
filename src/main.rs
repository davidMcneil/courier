extern crate actix;
extern crate actix_web;
extern crate chrono;
extern crate env_logger;
extern crate futures;
extern crate psutil;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
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
    #[structopt(default_value = "0.0.0.0", env = "COURIER_HOST", long = "host", short = "H")]
    host: String,
    /// A port number to listen on
    #[structopt(default_value = "3140", env = "COURIER_PORT", long = "port", short = "P")]
    port: u16,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt()]
    /// Run the service
    #[structopt(name = "run")]
    Run {
        /// The default amount time to live (ttl) of a topic's messages (seconds)
        #[structopt(default_value = "3600", long = "default-message-ttl")]
        default_message_ttl: i64,
        /// The default amount of time a subscription has to acknowledge a message (seconds)
        #[structopt(default_value = "60", long = "default-ack-deadline")]
        default_ack_deadline: i64,
        /// The default max number of messages pulled by a subscription
        #[structopt(default_value = "1", long = "default-max-messages")]
        default_max_messages: usize,
        /// The amount of time between running the cleanup thread (seconds)
        #[structopt(default_value = "1", long = "cleanup-interval")]
        cleanup_interval: i64,
    },
}

pub fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let opt = Opt::from_args();
    match opt.cmd {
        Command::Run {
            default_message_ttl,
            default_ack_deadline,
            default_max_messages,
            cleanup_interval,
        } => {
            let config = http_protocol::Config {
                host: opt.host.clone(),
                port: opt.port,
                default_message_ttl: Duration::seconds(default_message_ttl),
                default_ack_deadline: Duration::seconds(default_ack_deadline),
                default_max_messages,
                cleanup_interval: Duration::seconds(cleanup_interval),
            };
            http_protocol::start(config)
        }
    }
}
