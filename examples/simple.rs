extern crate parking_lot;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use parking_lot::RwLock;
use reqwest::header::{ContentType, Headers};
use serde_json::Value;
use std::error::Error;
use std::sync::Arc;
use std::{thread, time};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageList {
    pub messages: Vec<Message>,
}

pub fn main() -> Result<(), Box<Error>> {
    let address = String::from("http://127.0.0.1:3140");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let client = Arc::new(RwLock::new(
        reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap(),
    ));

    let client1 = Arc::clone(&client);
    let address1 = address.clone();
    let child1 = thread::spawn(move || {
        client1
            .read()
            .put(&format!("{}/api/v0/topics/topic", address1))
            .body(r#"{ "message_ttl": 3600, "ttl": 0 }"#)
            .send()
            .unwrap();
        loop {
            client1
                .read()
                .post(&format!("{}/api/v0/topics/topic/publish", address1))
                .body(r#"{ "messages": [ { "data": "simple" } ] }"#)
                .send()
                .unwrap();
            thread::sleep(time::Duration::from_millis(2000));
        }
    });

    let client2 = Arc::clone(&client);
    let address2 = address.clone();
    let child2 = thread::spawn(move || {
        client2
            .read()
            .put(&format!("{}/api/v0/subscriptions/sub1", address2))
            .body(r#"{ "topic": "topic", "ack_deadline": 60, "ttl": 0, "historical": true }"#)
            .send()
            .unwrap();
        loop {
            let message_list: Value = client2
                .read()
                .post(&format!("{}/api/v0/subscriptions/sub1/pull", address2))
                .body(r#"{ "max_message": 1 }"#)
                .send()
                .unwrap()
                .json()
                .unwrap();
            println!("{:?}", json);
            thread::sleep(time::Duration::from_millis(1000));
        }
    });

    // let client3 = Arc::clone(&client);
    // let address3 = address.clone();
    // let child3 = thread::spawn(move || {
    //     client3
    //         .read()
    //         .put(&format!("{}/api/v0/subscriptions/sub2", address3))
    //         .body(r#"{ "topic": "1s", "ack_deadline": 60, "ttl": 0, "historical": true }"#)
    //         .send()
    //         .unwrap();
    //     loop {
    //         client3
    //             .read()
    //             .post(&format!("{}/api/v0/subscriptions/sub2/pull", address3))
    //             .body(r#"{ "max_message": 1 }"#)
    //             .send()
    //             .unwrap();
    //         thread::sleep(time::Duration::from_millis(2000));
    //     }
    // });

    // let client4 = Arc::clone(&client);
    // let address4 = address.clone();
    // let child4 = thread::spawn(move || {
    //     client4
    //         .read()
    //         .put(&format!("{}/api/v0/subscriptions/sub3", address4))
    //         .body(r#"{ "topic": "1s", "ack_deadline": 60, "ttl": 0, "historical": true }"#)
    //         .send()
    //         .unwrap();
    //     loop {
    //         client4
    //             .read()
    //             .post(&format!("{}/api/v0/subscriptions/sub3/pull", address4))
    //             .body(r#"{ "max_message": 1 }"#)
    //             .send()
    //             .unwrap();
    //         thread::sleep(time::Duration::from_millis(3000));
    //     }
    // });

    let _ = child1.join();
    let _ = child2.join();
    // let _ = child3.join();
    // let _ = child4.join();

    return Ok(());
}
