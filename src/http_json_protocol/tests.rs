use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;
use serde::de::DeserializeOwned;
use serde_json;

use self::types::*;
use courier::{SubscriptionMeta, TopicMeta};
use http_json_protocol::*;

fn response_as_struct<T: DeserializeOwned>(response: &mut rocket::Response) -> T {
    serde_json::from_str::<T>(&response.body_string().unwrap()).unwrap()
}

#[test]
fn basic() {
    let client = Client::new(rocket()).expect("valid rocket instance");

    // Create a topic
    let expected = TopicMeta {
        name: String::from("test_topic"),
        message_ttl: 10,
    };
    let topic_config = TopicConfig { message_ttl: 10 };
    let mut response = client
        .put("api/v0/topics/test_topic")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Created);
    assert_eq!(
        response.body_string(),
        Some(serde_json::to_string(&expected).unwrap())
    );
    // Create another topic
    let expected = TopicMeta {
        name: String::from("another_topic"),
        message_ttl: 5,
    };
    let topic_config = TopicConfig { message_ttl: 5 };
    let mut response = client
        .put("api/v0/topics/another_topic")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Created);
    assert_eq!(
        response.body_string(),
        Some(serde_json::to_string(&expected).unwrap())
    );
    // Try and create another topic with the same name
    let expected = TopicMeta {
        name: String::from("another_topic"),
        message_ttl: 5,
    };
    let topic_config = TopicConfig { message_ttl: 30 };
    let mut response = client
        .put("api/v0/topics/another_topic")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::AlreadyReported);
    assert_eq!(
        response.body_string(),
        Some(serde_json::to_string(&expected).unwrap())
    );

    // Create subscriptions on topic test_topic
    let expected = SubscriptionMeta {
        name: String::from("sub0"),
        topic: String::from("test_topic"),
        ack_deadline: 2,
    };
    let subscription_config = SubscriptionConfig {
        topic: String::from("test_topic"),
        ack_deadline: 2,
    };
    let mut response = client
        .put("api/v0/subscriptions/sub0")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Created);
    assert_eq!(
        response.body_string(),
        Some(serde_json::to_string(&expected).unwrap())
    );
    let expected = SubscriptionMeta {
        name: String::from("sub1"),
        topic: String::from("test_topic"),
        ack_deadline: 4,
    };
    let subscription_config = SubscriptionConfig {
        topic: String::from("test_topic"),
        ack_deadline: 4,
    };
    let mut response = client
        .put("api/v0/subscriptions/sub1")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Created);
    assert_eq!(
        response.body_string(),
        Some(serde_json::to_string(&expected).unwrap())
    );
    // Try and create another topic with the same name
    let expected = SubscriptionMeta {
        name: String::from("sub1"),
        topic: String::from("test_topic"),
        ack_deadline: 4,
    };
    let subscription_config = SubscriptionConfig {
        topic: String::from("test_topic"),
        ack_deadline: 10,
    };
    let mut response = client
        .put("api/v0/subscriptions/sub1")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::AlreadyReported);
    assert_eq!(
        response.body_string(),
        Some(serde_json::to_string(&expected).unwrap())
    );
    // Create subscription on topic another_topic
    let expected = SubscriptionMeta {
        name: String::from("sub2"),
        topic: String::from("another_topic"),
        ack_deadline: 5,
    };
    let subscription_config = SubscriptionConfig {
        topic: String::from("another_topic"),
        ack_deadline: 5,
    };
    let mut response = client
        .put("api/v0/subscriptions/sub2")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Created);
    assert_eq!(
        response.body_string(),
        Some(serde_json::to_string(&expected).unwrap())
    );
    // Try and create a subscription to a topic that does not exist
    let subscription_config = SubscriptionConfig {
        topic: String::from("bad_topic"),
        ack_deadline: 5,
    };
    let response = client
        .put("api/v0/subscriptions/sub3")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);

    // Get a topic
    let expected = TopicMeta {
        name: String::from("test_topic"),
        message_ttl: 10,
    };
    let mut response = client.get("api/v0/topics/test_topic").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some(serde_json::to_string(&expected).unwrap())
    );
    // Try and get a topic that does not exist
    let response = client.get("api/v0/topics/bad_topic").dispatch();
    assert_eq!(response.status(), Status::NotFound);

    // Get a subscription
    let expected = SubscriptionMeta {
        name: String::from("sub0"),
        topic: String::from("test_topic"),
        ack_deadline: 2,
    };
    let mut response = client.get("api/v0/subscriptions/sub0").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some(serde_json::to_string(&expected).unwrap())
    );
    // Try and get a topic that does not exist
    let response = client
        .get("api/v0/subscriptions/bad_subscription")
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);

    // Get a topics list
    let _expected = TopicList {
        topics: vec![
            TopicMeta {
                name: String::from("another_topic"),
                message_ttl: 5,
            },
            TopicMeta {
                name: String::from("test_topic"),
                message_ttl: 10,
            },
        ],
    };
    let mut response = client.get("api/v0/topics").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response_as_struct::<TopicList>(&mut response).topics.len(),
        2
    );

    // Get a subscriptions list
    let _expected = SubscriptionList {
        subscriptions: vec![
            SubscriptionMeta {
                name: String::from("sub2"),
                topic: String::from("another_topic"),
                ack_deadline: 5,
            },
            SubscriptionMeta {
                name: String::from("sub1"),
                topic: String::from("test_topic"),
                ack_deadline: 4,
            },
            SubscriptionMeta {
                name: String::from("sub0"),
                topic: String::from("test_topic"),
                ack_deadline: 2,
            },
        ],
    };
    let mut response = client.get("api/v0/subscriptions").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response_as_struct::<SubscriptionList>(&mut response)
            .subscriptions
            .len(),
        3
    );

    // Get a topics subscriptions list
    let mut response = client
        .get("api/v0/topics/test_topic/subscriptions")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response_as_struct::<SubscriptionNameList>(&mut response)
            .subscriptions
            .len(),
        2
    );
}
