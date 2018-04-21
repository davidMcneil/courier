use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;
use serde::de::DeserializeOwned;
use serde_json;

use self::types::*;
use courier::RawMessage;
use courier::{SubscriptionMeta, TopicMeta};
use http_protocol::*;
use std::thread;
use std::time;

fn response_as<T: DeserializeOwned>(response: &mut rocket::Response) -> T {
    serde_json::from_str::<T>(&response.body_string().unwrap()).unwrap()
}

fn get_client() -> (Config, Client) {
    let config = Config {
        host: String::from("localhost"),
        port: 3140,
        default_message_ttl: Duration::seconds(3600),
        default_ack_deadline: Duration::seconds(60),
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
        historical: Some(false),
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
        historical: None,
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
        historical: Some(false),
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
        historical: Some(true),
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
        historical: Some(true),
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
        historical: Some(true),
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
        historical: Some(false),
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
    // Delete a subscription
    client
        .delete("api/v0/subscriptions/subscription1")
        .header(ContentType::JSON)
        .dispatch();
    // List the topic subscriptions
    let expected = SubscriptionNameList::new(vec![String::from("subscription0")]);
    let mut response = client
        .get("api/v0/topics/topic0/subscriptions")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(Status::Ok, response.status());
    let mut actual = response_as::<SubscriptionNameList>(&mut response);
    actual.subscriptions.sort();
    assert_eq!(expected, actual);
}

#[test]
fn http_protocol_basic() {
    let (_, client) = get_client();

    // Create a new topics
    let topic_config = TopicCreateConfig {
        message_ttl: Some(2),
    };
    client
        .put("api/v0/topics/topic0")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&topic_config).unwrap())
        .dispatch();
    // Publish messages
    let messages = RawMessageList::new(vec![
        RawMessage::new(String::from("first")),
        RawMessage::new(String::from("second")),
        RawMessage::new(String::from("third")),
    ]);
    let mut response = client
        .post("api/v0/topics/topic0/publish")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&messages).unwrap())
        .dispatch();
    let message_ids = response_as::<MessageIdList>(&mut response).message_ids;
    assert_eq!(Status::Ok, response.status());
    assert_eq!(messages.messages.len(), message_ids.len());

    // // Create a new subscriptions
    let mut subscription_config = SubscriptionCreateConfig {
        topic: String::from("topic0"),
        ack_deadline: Some(1),
        historical: Some(true),
    };
    client
        .put("api/v0/subscriptions/subscription0")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();
    subscription_config.historical = None;
    client
        .put("api/v0/subscriptions/subscription1")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&subscription_config).unwrap())
        .dispatch();

    // Pull the first two messages
    let pull_config = PullConfig::new(2);
    let mut response = client
        .post("api/v0/subscriptions/subscription0/pull")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&pull_config).unwrap())
        .dispatch();
    let messages = response_as::<MessageList>(&mut response).messages;
    assert_eq!(Status::Ok, response.status());
    assert_eq!(pull_config.max_messages.unwrap(), messages.len());
    assert_eq!(messages[0].data, String::from("first"));
    assert_eq!(messages[1].data, String::from("second"));
    // Try and pull messages from historical subscription1
    let pull_config = PullConfig::new(100);
    let mut response = client
        .post("api/v0/subscriptions/subscription1/pull")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&pull_config).unwrap())
        .dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!(0, response_as::<MessageList>(&mut response).messages.len());
    // Ack the second one
    let message_ids = MessageIdList::new(vec![messages[1].id]);
    let response = client
        .post("api/v0/subscriptions/subscription0/ack")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&message_ids).unwrap())
        .dispatch();
    assert_eq!(Status::Ok, response.status());
    // Wait for the ack deadline to expire
    thread::sleep(time::Duration::from_millis(1100));
    // Try and pull 5 more message
    let pull_config = PullConfig::new(5);
    let mut response = client
        .post("api/v0/subscriptions/subscription0/pull")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&pull_config).unwrap())
        .dispatch();
    let messages = response_as::<MessageList>(&mut response).messages;
    assert_eq!(Status::Ok, response.status());
    assert_eq!(2, messages.len());
    assert_eq!(String::from("first"), messages[0].data);
    assert_eq!(String::from("third"), messages[1].data);

    // Publish more messages
    let messages = RawMessageList::new(vec![
        RawMessage::new(String::from("fourth")),
        RawMessage::new(String::from("fifth")),
    ]);
    let mut response = client
        .post("api/v0/topics/topic0/publish")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&messages).unwrap())
        .dispatch();
    let message_ids = response_as::<MessageIdList>(&mut response).message_ids;
    assert_eq!(Status::Ok, response.status());
    assert_eq!(messages.messages.len(), message_ids.len());
    // Sleep so the first messages expire
    thread::sleep(time::Duration::from_millis(1100));

    // Pull messages from subscription1
    let pull_config = PullConfig::new(100);
    let mut response = client
        .post("api/v0/subscriptions/subscription1/pull")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&pull_config).unwrap())
        .dispatch();
    let messages = response_as::<MessageList>(&mut response).messages;
    assert_eq!(Status::Ok, response.status());
    assert_eq!(2, messages.len());
    assert_eq!(String::from("fourth"), messages[0].data);
    assert_eq!(String::from("fifth"), messages[1].data);

    // Try and publish to nonexistent topic
    let messages = RawMessageList::new(vec![]);
    let response = client
        .post("api/v0/topics/does_not_exist/publish")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&messages).unwrap())
        .dispatch();
    assert_eq!(Status::NotFound, response.status());

    // Try and pull from a nonexistent subscription
    let pull_config = PullConfig::new(100);
    let response = client
        .post("api/v0/subscriptions/does_not_exist/pull")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&pull_config).unwrap())
        .dispatch();
    assert_eq!(Status::NotFound, response.status());

    // Try and ack to a nonexistent subscription
    let message_ids = MessageIdList::new(vec![]);
    let response = client
        .post("api/v0/subscriptions/does_not_exist/ack")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&message_ids).unwrap())
        .dispatch();
    assert_eq!(Status::NotFound, response.status());
}

#[test]
fn http_protocol_general_handlers() {
    let (_, client) = get_client();

    let response = client.get("").dispatch();
    assert_eq!(Status::Ok, response.status());
    let response = client.get("ui").dispatch();
    assert_eq!(Status::Ok, response.status());
    let response = client.get("/api/v0/metrics").dispatch();
    assert_eq!(Status::Ok, response.status());
}
