extern crate chrono;
extern crate courier_client;
extern crate rand;

use chrono::Utc;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::error::Error;
use std::thread;
use std::time;

use courier_client::{Client, MessageList, SubscriptionCreateConfig, TopicCreateConfig};

static ADDRESS: &str = "http://127.0.0.1:3140";

pub fn main() -> Result<(), Box<dyn Error>> {
    let child1 = thread::spawn(|| {
        let mut rng = thread_rng();
        let uniform = Uniform::new(100, 1000);

        let client = Client::new(&ADDRESS).unwrap();
        let _ = client.delete_topic("topic");
        let _ = client.create_topic("topic", &TopicCreateConfig::new());

        loop {
            client
                .publish_one("topic", Utc::now().to_rfc3339())
                .unwrap();
            thread::sleep(time::Duration::from_millis(rng.sample(uniform)));
        }
    });

    let child2 = thread::spawn(|| {
        let mut rng = thread_rng();
        let uniform1 = Uniform::new(1000, 3000);
        let uniform2 = Uniform::new(1, 10);

        let client = Client::new(&ADDRESS).unwrap();
        let _ = client.delete_subscription("sub1");
        let _ = client.create_subscription("sub1", &SubscriptionCreateConfig::new("topic"));

        loop {
            let MessageList { mut messages } = client.pull("sub1", rng.sample(uniform2)).unwrap();
            thread_rng().shuffle(&mut messages);
            let to_ack = messages
                .iter()
                .take(rng.sample(uniform2))
                .map(|m| m.id.clone())
                .collect();
            client.ack("sub1", to_ack).unwrap();
            thread::sleep(time::Duration::from_millis(rng.sample(uniform1)));
        }
    });

    let _ = child1.join();
    let _ = child2.join();
    // let _ = child3.join();
    // let _ = child4.join();

    return Ok(());
}
