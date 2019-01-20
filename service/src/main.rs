use chrono::Duration;
use env_logger::fmt::WriteStyle;
use env_logger::Builder;
use log::LevelFilter;
use log::{error, info};
use open;
use structopt;
use structopt::StructOpt;

mod http_protocol;

#[derive(Debug, StructOpt)]
#[structopt()]
struct Opt {
    /// An IP address or host the application will listen on
    #[structopt(
        default_value = "0.0.0.0",
        env = "COURIER_HOST",
        long = "host",
        short = "H"
    )]
    host: String,
    /// A port number to listen on
    #[structopt(
        default_value = "3140",
        env = "COURIER_PORT",
        long = "port",
        short = "P"
    )]
    port: u16,
    /// Log level
    #[structopt(
        default_value = "info",
        long = "log-level",
        raw(possible_values = r#"&["trace", "debug", "info", "warn", "error"]"#)
    )]
    log_level: LevelFilter,
    /// Uncolored log
    #[structopt(long = "uncolored-log")]
    uncolored_log: bool,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt()]
    /// Run the service
    #[structopt(name = "run")]
    Run {
        /// Default time to live (ttl) of a topic (seconds)
        #[structopt(default_value = "0", long = "default-topic-ttl")]
        default_topic_ttl: i64,
        /// Default time to live (ttl) of a subscription (seconds)
        #[structopt(default_value = "0", long = "default-subscription-ttl")]
        default_subscription_ttl: i64,
        /// Default time to live (ttl) of a messages (seconds)
        #[structopt(default_value = "3600", long = "default-message-ttl")]
        default_message_ttl: i64,
        /// Default duration a subscription has to acknowledge a message (seconds)
        #[structopt(default_value = "60", long = "default-ack-deadline")]
        default_ack_deadline: i64,
        /// Default max number of messages pulled by a subscription
        #[structopt(default_value = "1", long = "default-max-messages")]
        default_max_messages: usize,
        /// Duration between running the cleanup thread (seconds)
        #[structopt(default_value = "1", long = "cleanup-interval")]
        cleanup_interval: i64,
    },
    /// Launch the web user interface
    #[structopt(name = "ui")]
    Ui {},
}

pub fn main() {
    let opt = Opt::from_args();

    Builder::new()
        .filter_level(opt.log_level)
        .filter(Some("mio::poll"), LevelFilter::Warn)
        .filter(Some("tokio_core::reactor"), LevelFilter::Warn)
        .filter(Some("tokio_reactor"), LevelFilter::Warn)
        .filter(Some("tokio_reactor::background"), LevelFilter::Warn)
        .filter(Some("tokio_threadpool::builder"), LevelFilter::Warn)
        .filter(Some("tokio_threadpool::pool"), LevelFilter::Warn)
        .write_style(if opt.uncolored_log {
            WriteStyle::Never
        } else {
            WriteStyle::Always
        })
        .init();

    match opt.cmd {
        Command::Run {
            default_topic_ttl,
            default_subscription_ttl,
            default_message_ttl,
            default_ack_deadline,
            default_max_messages,
            cleanup_interval,
        } => {
            let config = http_protocol::Config {
                host: opt.host.clone(),
                port: opt.port,
                default_topic_ttl: Duration::seconds(default_topic_ttl),
                default_subscription_ttl: Duration::seconds(default_subscription_ttl),
                default_message_ttl: Duration::seconds(default_message_ttl),
                default_ack_deadline: Duration::seconds(default_ack_deadline),
                default_max_messages,
                cleanup_interval: Duration::seconds(cleanup_interval),
            };
            http_protocol::start(config)
        }
        Command::Ui {} => {
            let url = format!("http://{}:{}/ui", &opt.host, opt.port);
            match open::that(url.clone()) {
                Ok(exit_status) => {
                    if exit_status.success() {
                        info!("Opened the ui at '{}'.", url);
                    } else if let Some(code) = exit_status.code() {
                        error!(
                            "Opining the ui at '{}' returned a non-zero exit status {}.",
                            url, code
                        );
                    } else {
                        error!(
                            "Opening the ui at '{}' returned an unknown exit status.",
                            url
                        );
                    }
                }
                Err(why) => error!("Failed to open the ui at '{}': {}", url, why),
            }
        }
    }
}
