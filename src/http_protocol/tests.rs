use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;
use serde::de::DeserializeOwned;
use serde_json;

use self::types::*;
use courier::{SubscriptionMeta, TopicMeta};
use http_protocol::*;

fn response_as<T: DeserializeOwned>(response: &mut rocket::Response) -> T {
    serde_json::from_str::<T>(&response.body_string().unwrap()).unwrap()
}

fn get_client() -> (Config, Client) {
    let config = Config {
        default_message_ttl: Duration::seconds(3600),
        default_ack_deadline: Duration::seconds(60),
        default_return_immediately: false,
        default_max_messages: 1,
        cleanup_interval: Duration::seconds(1),
        max_pull_wait: Duration::seconds(5),
    };
    let client = Client::new(rocket(config.clone())).expect("valid rocket instance");
    (config, client)
}

#[test]
fn http_protocol_create_topic() {
    let (config, client) = get_client();

    let expected = TopicMeta {
        name: String::from("test"),
        message_ttl: config.default_message_ttl.num_seconds(),
    };
    // Create a new topic
    let topic_config = TopicCreateConfig { message_ttl: None };
    let mut response = client
        .put("api/v0/topics/test")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();
    assert_eq!(Status::Created, response.status());
    assert_eq!(expected, response_as::<TopicMeta>(&mut response));
    // Try and create a topic that already exists
    let topic_config = TopicCreateConfig {
        message_ttl: Some(30),
    };
    let mut response = client
        .put("api/v0/topics/test")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();
    assert_eq!(Status::Conflict, response.status());
    assert_eq!(expected, response_as::<TopicMeta>(&mut response));
    // Create a topic with no name
    let topic_config = TopicCreateConfig {
        message_ttl: Some(12),
    };
    let mut response = client
        .put("api/v0/topics")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();
    let meta = response_as::<TopicMeta>(&mut response);
    let expected = TopicMeta {
        name: meta.name.clone(),
        message_ttl: 12,
    };
    assert_eq!(Status::Created, response.status());
    assert_eq!(expected, meta);
}

#[test]
fn http_protocol_update_topic() {
    let (_, client) = get_client();

    // Create a new topic
    let topic_config = TopicCreateConfig { message_ttl: None };
    client
        .put("api/v0/topics/test_topic")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();
    // Update the topic
    let topic_config = TopicUpdateConfig {
        message_ttl: Some(60),
    };
    let mut response = client
        .patch("api/v0/topics/test_topic")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();
    let expected = TopicMeta {
        name: String::from("test_topic"),
        message_ttl: 60,
    };
    assert_eq!(Status::Ok, response.status());
    assert_eq!(expected, response_as::<TopicMeta>(&mut response));
    // Update the topic with no ttl
    let topic_config = TopicUpdateConfig { message_ttl: None };
    let mut response = client
        .patch("api/v0/topics/test_topic")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(expected, response_as::<TopicMeta>(&mut response));
    // Update a non existent topic
    let response = client
        .patch("api/v0/topics/does_not_exist")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();
    assert_eq!(Status::NotFound, response.status());
}

#[test]
fn http_protocol_delete_and_get_topic() {
    let (config, client) = get_client();

    let expected = TopicMeta {
        name: String::from("test"),
        message_ttl: config.default_message_ttl.num_seconds(),
    };
    // Create a new topic
    let topic_config = TopicCreateConfig { message_ttl: None };
    client
        .put("api/v0/topics/test")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();
    // Get the topic
    let mut response = client
        .get("api/v0/topics/test")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(expected, response_as::<TopicMeta>(&mut response));
    // Delete the topic
    let response = client
        .delete("api/v0/topics/test")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(Status::Ok, response.status());
    // Delete a topic that does not exist
    let response = client
        .delete("api/v0/topics/does_not_exist")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(Status::NotFound, response.status());
    // Get a topic that does not exist
    let response = client
        .get("api/v0/topics/test")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(Status::NotFound, response.status());
}

#[test]
fn http_protocol_create_subscription() {
    let (config, client) = get_client();

    // Create a new topic
    let topic_config = TopicCreateConfig { message_ttl: None };
    client
        .put("api/v0/topics/test_topic")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();

    let expected = SubscriptionMeta {
        name: String::from("test"),
        topic: String::from("test_topic"),
        ack_deadline: config.default_ack_deadline.num_seconds(),
    };
    // Create a new subscription
    let subscription_config = SubscriptionCreateConfig {
        topic: String::from("test_topic"),
        ack_deadline: None,
    };
    let mut response = client
        .put("api/v0/subscriptions/test")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Created);
    assert_eq!(expected, response_as::<SubscriptionMeta>(&mut response));
    // Try and create a subscription that already exists
    let subscription_config = SubscriptionCreateConfig {
        topic: String::from("test_topic"),
        ack_deadline: Some(45),
    };
    let mut response = client
        .put("api/v0/subscriptions/test")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Conflict);
    assert_eq!(expected, response_as::<SubscriptionMeta>(&mut response));
    // Create a subscription with no name
    let subscription_config = SubscriptionCreateConfig {
        topic: String::from("test_topic"),
        ack_deadline: Some(67),
    };
    let mut response = client
        .put("api/v0/subscriptions")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    let meta = response_as::<SubscriptionMeta>(&mut response);
    let expected = SubscriptionMeta {
        name: meta.name.clone(),
        topic: String::from("test_topic"),
        ack_deadline: 67,
    };
    assert_eq!(Status::Created, response.status());
    assert_eq!(expected, meta);
}

#[test]
fn http_protocol_update_subscription() {
    let (_, client) = get_client();

    // Create a new topic
    let topic_config = TopicCreateConfig { message_ttl: None };
    client
        .put("api/v0/topics/test_topic")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();

    // Create a new subscription
    let subscription_config = SubscriptionCreateConfig {
        topic: String::from("test_topic"),
        ack_deadline: None,
    };
    client
        .put("api/v0/subscriptions/test_subscription")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    // Update the subscription
    let subscription_config = SubscriptionUpdateConfig {
        ack_deadline: Some(60),
    };
    let mut response = client
        .patch("api/v0/subscriptions/test_subscription")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    let expected = SubscriptionMeta {
        name: String::from("test_subscription"),
        topic: String::from("test_topic"),
        ack_deadline: 60,
    };
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(expected, response_as::<SubscriptionMeta>(&mut response));
    // Update the subscription with no deadline
    let subscription_config = SubscriptionUpdateConfig { ack_deadline: None };
    let mut response = client
        .patch("api/v0/subscriptions/test_subscription")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(expected, response_as::<SubscriptionMeta>(&mut response));
    // Update a non existent topic
    let response = client
        .patch("api/v0/subscriptions/does_not_exist")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    assert_eq!(Status::NotFound, response.status());
}

#[test]
fn http_protocol_delete_and_get_subscription() {
    let (config, client) = get_client();

    // Create a new topic
    let topic_config = TopicCreateConfig { message_ttl: None };
    client
        .put("api/v0/topics/test_topic")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();

    let expected = SubscriptionMeta {
        name: String::from("test"),
        topic: String::from("test_topic"),
        ack_deadline: config.default_ack_deadline.num_seconds(),
    };
    // Create a new subscription
    let subscription_config = SubscriptionCreateConfig {
        topic: String::from("test_topic"),
        ack_deadline: None,
    };
    client
        .put("api/v0/subscriptions/test")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    // Get the subscription
    let mut response = client
        .get("api/v0/subscriptions/test")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(expected, response_as::<SubscriptionMeta>(&mut response));
    // Delete the subscription
    let response = client
        .delete("api/v0/subscriptions/test")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(Status::Ok, response.status());
    // Delete a subscription that does not exist
    let response = client
        .delete("api/v0/subscriptions/does_not_exist")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(Status::NotFound, response.status());
    // Get a subscription that does not exist
    let response = client
        .get("api/v0/subscriptions/test")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(Status::NotFound, response.status());
}

#[test]
fn http_protocol_lists() {
    let (config, client) = get_client();

    // Create a new topics
    let topic_config = TopicCreateConfig { message_ttl: None };
    client
        .put("api/v0/topics/topic0")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();
    client
        .put("api/v0/topics/topic1")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();

    // Create new subscriptions
    let subscription_config = SubscriptionCreateConfig {
        topic: String::from("topic0"),
        ack_deadline: None,
    };
    client
        .put("api/v0/subscriptions/subscription0")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    client
        .put("api/v0/subscriptions/subscription1")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    let subscription_config = SubscriptionCreateConfig {
        topic: String::from("topic1"),
        ack_deadline: None,
    };
    client
        .put("api/v0/subscriptions/subscription2")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();

    // List the topics
    let expected = TopicList::new(vec![
        TopicMeta {
            name: String::from("topic0"),
            message_ttl: config.default_message_ttl.num_seconds(),
        },
        TopicMeta {
            name: String::from("topic1"),
            message_ttl: config.default_message_ttl.num_seconds(),
        },
    ]);
    let mut response = client
        .get("api/v0/topics")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(Status::Ok, response.status());
    let mut actual = response_as::<TopicList>(&mut response);
    actual.topics.sort();
    assert_eq!(expected, actual);
    // List the subscriptions
    let expected = SubscriptionList::new(vec![
        SubscriptionMeta {
            name: String::from("subscription0"),
            topic: String::from("topic0"),
            ack_deadline: config.default_ack_deadline.num_seconds(),
        },
        SubscriptionMeta {
            name: String::from("subscription1"),
            topic: String::from("topic0"),
            ack_deadline: config.default_ack_deadline.num_seconds(),
        },
        SubscriptionMeta {
            name: String::from("subscription2"),
            topic: String::from("topic1"),
            ack_deadline: config.default_ack_deadline.num_seconds(),
        },
    ]);
    let mut response = client
        .get("api/v0/subscriptions")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(Status::Ok, response.status());
    let mut actual = response_as::<SubscriptionList>(&mut response);
    actual.subscriptions.sort();
    assert_eq!(expected, actual);
    // List the topic subscriptions
    let expected = SubscriptionNameList::new(vec![
        String::from("subscription0"),
        String::from("subscription1"),
    ]);
    let mut response = client
        .get("api/v0/topics/topic0/subscriptions")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(Status::Ok, response.status());
    let mut actual = response_as::<SubscriptionNameList>(&mut response);
    actual.subscriptions.sort();
    assert_eq!(expected, actual);
}
