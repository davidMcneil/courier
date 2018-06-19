use actix_web::http::{Method, StatusCode};
use actix_web::{test, HttpMessage};
use chrono::Duration;
use futures::Future;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use courier::RawMessage;
use courier::{SubscriptionMeta, TopicMeta};

use http_protocol::types::*;
use http_protocol::*;
use std::thread;
use std::time;

fn get_status_with_prefix<T>(
    server: &mut test::TestServer,
    prefix: &str,
    path: &str,
    method: Method,
    json: T,
) -> StatusCode
where
    T: Serialize,
{
    let request = server
        .client(method, &format!("{}/{}", prefix, path))
        .content_type("application/json")
        .json(json)
        .unwrap();
    let response = server.execute(request.send()).unwrap();
    response.status()
}

fn get_status<T>(server: &mut test::TestServer, path: &str, method: Method, json: T) -> StatusCode
where
    T: Serialize,
{
    get_status_with_prefix(server, "api/v0", path, method, json)
}

fn get_response<T, R>(
    server: &mut test::TestServer,
    path: &str,
    method: Method,
    json: T,
) -> (StatusCode, R)
where
    T: Serialize,
    R: DeserializeOwned + 'static,
{
    let request = server
        .client(method, &format!("api/v0/{}", path))
        .content_type("application/json")
        .json(json)
        .unwrap();
    let response = server.execute(request.send()).unwrap();
    (response.status(), response.json::<R>().wait().unwrap())
}

fn get_server() -> (Config, test::TestServer) {
    let config = Config {
        host: String::from("localhost"),
        port: 3140,
        default_topic_ttl: Duration::seconds(0),
        default_subscription_ttl: Duration::seconds(0),
        default_message_ttl: Duration::seconds(3600),
        default_ack_deadline: Duration::seconds(60),
        default_max_messages: 1,
        cleanup_interval: Duration::seconds(1),
    };
    let server = test::TestServer::with_factory(create(config.clone()));
    (config, server)
}

#[test]
fn http_protocol_create_topic() {
    let (config, mut server) = get_server();

    // Create a new topic
    let topic_config = TopicCreateConfig {
        message_ttl: None,
        ttl: None,
    };
    let (status, body): (_, TopicMeta) =
        get_response(&mut server, "topics/test", Method::PUT, topic_config);
    let expected = TopicMeta {
        name: String::from("test"),
        message_ttl: config.default_message_ttl.num_seconds(),
        ttl: 0,
        created: body.created,
        updated: body.updated,
    };
    assert_eq!(StatusCode::CREATED, status);
    assert_eq!(expected, body);
    // Try and create a topic that already exists
    let topic_config = TopicCreateConfig {
        message_ttl: Some(30),
        ttl: None,
    };
    let (status, body) = get_response(&mut server, "topics/test", Method::PUT, topic_config);
    assert_eq!(StatusCode::CONFLICT, status);
    assert_eq!(expected, body);
    // Create a topic with no name
    let topic_config = TopicCreateConfig {
        message_ttl: Some(12),
        ttl: None,
    };
    let (status, body): (_, TopicMeta) =
        get_response(&mut server, "topics/", Method::PUT, topic_config);
    let expected = TopicMeta {
        name: body.name.clone(),
        message_ttl: 12,
        ttl: 0,
        created: body.created,
        updated: body.updated,
    };
    assert_eq!(StatusCode::CREATED, status);
    assert_eq!(expected, body);
}

#[test]
fn http_protocol_update_topic() {
    let (_, mut server) = get_server();

    // Create a new topic
    let topic_config = TopicCreateConfig {
        message_ttl: None,
        ttl: None,
    };
    get_status(&mut server, "topics/test_topic", Method::PUT, topic_config);
    // Update the topic
    let topic_config = TopicUpdateConfig {
        message_ttl: Some(60),
        ttl: Some(73),
    };
    let (status, body): (_, TopicMeta) = get_response(
        &mut server,
        "topics/test_topic",
        Method::PATCH,
        topic_config,
    );
    let mut expected = TopicMeta {
        name: String::from("test_topic"),
        message_ttl: 60,
        ttl: 73,
        created: body.created,
        updated: body.updated,
    };
    assert_eq!(StatusCode::OK, status);
    assert_eq!(expected, body);
    // Update the topic with no values
    let topic_config = TopicUpdateConfig {
        message_ttl: None,
        ttl: None,
    };
    let (status, body): (_, TopicMeta) = get_response(
        &mut server,
        "topics/test_topic",
        Method::PATCH,
        topic_config.clone(),
    );
    expected.updated = body.updated;
    assert_eq!(StatusCode::OK, status);
    assert_eq!(expected, body);
    // Update a non existent topic
    let status = get_status(
        &mut server,
        "topics/does_not_exist",
        Method::PATCH,
        topic_config,
    );
    assert_eq!(StatusCode::NOT_FOUND, status);
}

#[test]
fn http_protocol_delete_and_get_topic() {
    let (config, mut server) = get_server();

    // Create a new topic
    let topic_config = TopicCreateConfig {
        message_ttl: None,
        ttl: None,
    };
    get_status(&mut server, "topics/test", Method::PUT, topic_config);
    // Get the topic
    let (status, body): (_, TopicMeta) = get_response(&mut server, "topics/test", Method::GET, ());
    let expected = TopicMeta {
        name: String::from("test"),
        message_ttl: config.default_message_ttl.num_seconds(),
        ttl: 0,
        created: body.created,
        updated: body.updated,
    };
    assert_eq!(StatusCode::OK, status);
    assert_eq!(expected, body);
    // Delete the topic
    let status = get_status(&mut server, "topics/test", Method::DELETE, ());
    assert_eq!(StatusCode::OK, status);
    // Delete a topic that does not exist
    let status = get_status(&mut server, "topics/does_not_exist", Method::DELETE, ());
    assert_eq!(StatusCode::NOT_FOUND, status);
    // Get a topic that does not exist
    let status = get_status(&mut server, "topics/test", Method::GET, ());
    assert_eq!(StatusCode::NOT_FOUND, status);
}

#[test]
fn http_protocol_create_subscription() {
    let (config, mut server) = get_server();

    // Create a new topic
    let topic_config = TopicCreateConfig {
        message_ttl: None,
        ttl: None,
    };
    get_status(&mut server, "topics/test_topic", Method::PUT, topic_config);

    // Create a new subscription
    let subscription_config = SubscriptionCreateConfig {
        topic: String::from("test_topic"),
        ack_deadline: None,
        ttl: None,
        historical: Some(false),
    };
    let (status, body): (_, SubscriptionMeta) = get_response(
        &mut server,
        "subscriptions/test",
        Method::PUT,
        subscription_config,
    );
    let expected = SubscriptionMeta {
        name: String::from("test"),
        topic: String::from("test_topic"),
        ack_deadline: config.default_ack_deadline.num_seconds(),
        ttl: 0,
        created: body.created,
        updated: body.updated,
    };
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(expected, body);
    // Try and create a subscription that already exists
    let subscription_config = SubscriptionCreateConfig {
        topic: String::from("test_topic"),
        ack_deadline: Some(45),
        ttl: None,
        historical: None,
    };
    let (status, body) = get_response(
        &mut server,
        "subscriptions/test",
        Method::PUT,
        subscription_config,
    );
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(expected, body);
    // Create a subscription with no name
    let subscription_config = SubscriptionCreateConfig {
        topic: String::from("test_topic"),
        ack_deadline: Some(67),
        ttl: None,
        historical: Some(false),
    };
    let (status, body): (_, SubscriptionMeta) = get_response(
        &mut server,
        "subscriptions/",
        Method::PUT,
        subscription_config,
    );
    let expected = SubscriptionMeta {
        name: body.name.clone(),
        topic: String::from("test_topic"),
        ack_deadline: 67,
        ttl: 0,
        created: body.created,
        updated: body.updated,
    };
    assert_eq!(StatusCode::CREATED, status);
    assert_eq!(expected, body);
}

#[test]
fn http_protocol_update_subscription() {
    let (_, mut server) = get_server();

    // Create a new topic
    let topic_config = TopicCreateConfig {
        message_ttl: None,
        ttl: None,
    };
    get_status(&mut server, "topics/test_topic", Method::PUT, topic_config);

    // Create a new subscription
    let subscription_config = SubscriptionCreateConfig {
        topic: String::from("test_topic"),
        ack_deadline: None,
        ttl: None,
        historical: Some(true),
    };
    get_status(
        &mut server,
        "subscriptions/test_subscription",
        Method::PUT,
        subscription_config,
    );
    // Update the subscription
    let subscription_config = SubscriptionUpdateConfig {
        ack_deadline: Some(60),
        ttl: Some(73),
    };
    let (status, body): (_, SubscriptionMeta) = get_response(
        &mut server,
        "subscriptions/test_subscription",
        Method::PATCH,
        subscription_config,
    );
    let mut expected = SubscriptionMeta {
        name: String::from("test_subscription"),
        topic: String::from("test_topic"),
        ack_deadline: 60,
        ttl: 73,
        created: body.created,
        updated: body.updated,
    };
    assert_eq!(status, StatusCode::OK);
    assert_eq!(expected, body);
    // Update the subscription with no deadline
    let subscription_config = SubscriptionUpdateConfig {
        ack_deadline: None,
        ttl: None,
    };
    let (status, body): (_, SubscriptionMeta) = get_response(
        &mut server,
        "subscriptions/test_subscription",
        Method::PATCH,
        subscription_config.clone(),
    );
    expected.updated = body.updated;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(expected, body);
    // Update a non existent topic
    let status = get_status(
        &mut server,
        "subscriptions/does_not_exist",
        Method::PATCH,
        subscription_config,
    );
    assert_eq!(StatusCode::NOT_FOUND, status);
}

#[test]
fn http_protocol_delete_and_get_subscription() {
    let (config, mut server) = get_server();

    // Create a new topic
    let topic_config = TopicCreateConfig {
        message_ttl: None,
        ttl: None,
    };
    get_status(&mut server, "topics/test_topic", Method::PUT, topic_config);

    // Create a new subscription
    let subscription_config = SubscriptionCreateConfig {
        topic: String::from("test_topic"),
        ack_deadline: None,
        ttl: None,
        historical: Some(true),
    };
    get_status(
        &mut server,
        "subscriptions/test",
        Method::PUT,
        subscription_config,
    );
    // Get the subscription
    let (status, body): (_, SubscriptionMeta) =
        get_response(&mut server, "subscriptions/test", Method::GET, ());
    let expected = SubscriptionMeta {
        name: String::from("test"),
        topic: String::from("test_topic"),
        ack_deadline: config.default_ack_deadline.num_seconds(),
        ttl: 0,
        created: body.created,
        updated: body.updated,
    };
    assert_eq!(StatusCode::OK, status);
    assert_eq!(expected, body);
    // Delete the subscription
    let status = get_status(&mut server, "subscriptions/test", Method::DELETE, ());
    assert_eq!(StatusCode::OK, status);
    // Delete a subscription that does not exist
    let status = get_status(
        &mut server,
        "subscriptions/does_not_exist",
        Method::DELETE,
        (),
    );
    assert_eq!(StatusCode::NOT_FOUND, status);
    // Get a subscription that does not exist
    let status = get_status(&mut server, "subscriptions/test", Method::GET, ());
    assert_eq!(StatusCode::NOT_FOUND, status);
}

#[test]
fn http_protocol_lists() {
    let (config, mut server) = get_server();

    // Create a new topics
    let topic_config = TopicCreateConfig {
        message_ttl: None,
        ttl: None,
    };
    get_status(
        &mut server,
        "topics/topic0",
        Method::PUT,
        topic_config.clone(),
    );
    get_status(&mut server, "topics/topic1", Method::PUT, topic_config);

    // Create new subscriptions
    let subscription_config = SubscriptionCreateConfig {
        topic: String::from("topic0"),
        ack_deadline: None,
        ttl: None,
        historical: Some(true),
    };
    get_status(
        &mut server,
        "subscriptions/subscription0",
        Method::PUT,
        subscription_config.clone(),
    );
    get_status(
        &mut server,
        "subscriptions/subscription1",
        Method::PUT,
        subscription_config,
    );
    let subscription_config = SubscriptionCreateConfig {
        topic: String::from("topic1"),
        ack_deadline: None,
        ttl: None,
        historical: Some(false),
    };
    get_status(
        &mut server,
        "subscriptions/subscription2",
        Method::PUT,
        subscription_config,
    );

    // List the topics
    let (status, mut body): (_, TopicList) = get_response(&mut server, "topics/", Method::GET, ());
    assert_eq!(StatusCode::OK, status);
    assert_eq!(2, body.topics.len());
    body.topics.sort();
    let expected = TopicList::new(vec![
        TopicMeta {
            name: String::from("topic0"),
            message_ttl: config.default_message_ttl.num_seconds(),
            ttl: 0,
            created: body.topics[0].created,
            updated: body.topics[0].updated,
        },
        TopicMeta {
            name: String::from("topic1"),
            message_ttl: config.default_message_ttl.num_seconds(),
            ttl: 0,
            created: body.topics[1].created,
            updated: body.topics[1].updated,
        },
    ]);
    assert_eq!(expected, body);
    // List the subscriptions
    let (status, mut body): (_, SubscriptionList) =
        get_response(&mut server, "subscriptions/", Method::GET, ());
    assert_eq!(StatusCode::OK, status);
    assert_eq!(3, body.subscriptions.len());
    body.subscriptions.sort();
    let expected = SubscriptionList::new(vec![
        SubscriptionMeta {
            name: String::from("subscription0"),
            topic: String::from("topic0"),
            ack_deadline: config.default_ack_deadline.num_seconds(),
            ttl: 0,
            created: body.subscriptions[0].created,
            updated: body.subscriptions[0].updated,
        },
        SubscriptionMeta {
            name: String::from("subscription1"),
            topic: String::from("topic0"),
            ack_deadline: config.default_ack_deadline.num_seconds(),
            ttl: 0,
            created: body.subscriptions[1].created,
            updated: body.subscriptions[1].updated,
        },
        SubscriptionMeta {
            name: String::from("subscription2"),
            topic: String::from("topic1"),
            ack_deadline: config.default_ack_deadline.num_seconds(),
            ttl: 0,
            created: body.subscriptions[2].created,
            updated: body.subscriptions[2].updated,
        },
    ]);
    assert_eq!(expected, body);
    // List the topic subscriptions
    let expected = SubscriptionNameList::new(vec![
        String::from("subscription0"),
        String::from("subscription1"),
    ]);
    let (status, mut body): (_, SubscriptionNameList) =
        get_response(&mut server, "topics/topic0/subscriptions", Method::GET, ());
    assert_eq!(StatusCode::OK, status);
    body.subscriptions.sort();
    assert_eq!(expected, body);
    // Delete a subscription
    get_status(
        &mut server,
        "subscriptions/subscription1",
        Method::DELETE,
        (),
    );
    // List the topic subscriptions
    let expected = SubscriptionNameList::new(vec![String::from("subscription0")]);
    let (status, mut body): (_, SubscriptionNameList) =
        get_response(&mut server, "topics/topic0/subscriptions", Method::GET, ());
    assert_eq!(StatusCode::OK, status);
    body.subscriptions.sort();
    assert_eq!(expected, body);
}

#[test]
fn http_protocol_ttls() {
    let (_, mut server) = get_server();

    // Create topics
    let topic_config = TopicCreateConfig {
        message_ttl: Some(2),
        ttl: Some(0),
    };
    get_status(&mut server, "topics/topic0", Method::PUT, topic_config);
    let topic_config = TopicCreateConfig {
        message_ttl: Some(2),
        ttl: Some(1),
    };
    get_status(&mut server, "topics/topic1", Method::PUT, topic_config);

    // Create subscriptions
    let topic_config = SubscriptionCreateConfig {
        topic: String::from("topic0"),
        ack_deadline: Some(1),
        ttl: None,
        historical: Some(true),
    };
    get_status(&mut server, "subscriptions/sub0", Method::PUT, topic_config);
    let topic_config = SubscriptionCreateConfig {
        topic: String::from("topic0"),
        ack_deadline: Some(1),
        ttl: Some(1),
        historical: Some(true),
    };
    get_status(&mut server, "subscriptions/sub1", Method::PUT, topic_config);

    // Wait for ttls to expire
    thread::sleep(time::Duration::from_millis(2000));

    let status = get_status(&mut server, "topics/topic0", Method::GET, ());
    assert_eq!(StatusCode::OK, status);
    let status = get_status(&mut server, "topics/topic1", Method::GET, ());
    assert_eq!(StatusCode::NOT_FOUND, status);

    let status = get_status(&mut server, "subscriptions/sub0", Method::GET, ());
    assert_eq!(StatusCode::OK, status);
    let status = get_status(&mut server, "subscriptions/sub1", Method::GET, ());
    assert_eq!(StatusCode::NOT_FOUND, status);
}

#[test]
fn http_protocol_basic() {
    let (_, mut server) = get_server();

    // Create a new topics
    let topic_config = TopicCreateConfig {
        message_ttl: Some(2),
        ttl: None,
    };
    get_status(&mut server, "topics/topic0", Method::PUT, topic_config);
    // Publish messages
    let messages = RawMessageList::new(vec![
        RawMessage::new(String::from("first")),
        RawMessage::new(String::from("second")),
        RawMessage::new(String::from("third")),
    ]);
    let (status, body): (_, MessageIdList) = get_response(
        &mut server,
        "topics/topic0/publish",
        Method::POST,
        messages.clone(),
    );
    assert_eq!(StatusCode::OK, status);
    assert_eq!(messages.messages.len(), body.message_ids.len());

    // Create a new subscriptions
    let mut subscription_config = SubscriptionCreateConfig {
        topic: String::from("topic0"),
        ack_deadline: Some(1),
        ttl: None,
        historical: Some(true),
    };
    get_status(
        &mut server,
        "subscriptions/subscription0",
        Method::PUT,
        subscription_config.clone(),
    );
    subscription_config.historical = None;
    get_status(
        &mut server,
        "subscriptions/subscription1",
        Method::PUT,
        subscription_config.clone(),
    );

    // Pull the first two messages
    let pull_config = PullConfig::new(2);
    let (status, body): (_, MessageList) = get_response(
        &mut server,
        "subscriptions/subscription0/pull",
        Method::POST,
        pull_config.clone(),
    );
    assert_eq!(StatusCode::OK, status);
    assert_eq!(pull_config.max_messages.unwrap(), body.messages.len());
    assert_eq!(body.messages[0].data, String::from("first"));
    assert_eq!(body.messages[1].data, String::from("second"));
    let saved_messages = body;
    // Try and pull messages from not historical subscription1
    let pull_config = PullConfig::new(100);
    let (status, body): (_, MessageList) = get_response(
        &mut server,
        "subscriptions/subscription1/pull",
        Method::POST,
        pull_config.clone(),
    );
    assert_eq!(StatusCode::OK, status);
    assert_eq!(0, body.messages.len());
    // Ack the second one
    let message_ids = MessageIdList::new(vec![saved_messages.messages[1].id]);
    let status = get_status(
        &mut server,
        "subscriptions/subscription0/ack",
        Method::POST,
        message_ids,
    );
    assert_eq!(StatusCode::OK, status);
    // Wait for the ack deadline to expire
    thread::sleep(time::Duration::from_millis(1100));
    // Try and pull 5 more message
    let pull_config = PullConfig::new(5);
    let (status, body): (_, MessageList) = get_response(
        &mut server,
        "subscriptions/subscription0/pull",
        Method::POST,
        pull_config,
    );
    assert_eq!(StatusCode::OK, status);
    assert_eq!(2, body.messages.len());
    assert_eq!(String::from("first"), body.messages[0].data);
    assert_eq!(String::from("third"), body.messages[1].data);

    // Publish more messages
    let messages = RawMessageList::new(vec![
        RawMessage::new(String::from("fourth")),
        RawMessage::new(String::from("fifth")),
    ]);
    let (status, body): (_, MessageIdList) = get_response(
        &mut server,
        "topics/topic0/publish",
        Method::POST,
        messages.clone(),
    );
    assert_eq!(StatusCode::OK, status);
    assert_eq!(messages.messages.len(), body.message_ids.len());
    // Sleep so the first messages expire
    thread::sleep(time::Duration::from_millis(1100));

    // Pull messages from subscription1
    let pull_config = PullConfig::new(100);
    let (status, body): (_, MessageList) = get_response(
        &mut server,
        "subscriptions/subscription1/pull",
        Method::POST,
        pull_config,
    );
    assert_eq!(StatusCode::OK, status);
    assert_eq!(2, body.messages.len());
    assert_eq!(String::from("fourth"), body.messages[0].data);
    assert_eq!(String::from("fifth"), body.messages[1].data);

    // Try and publish to nonexistent topic
    let messages = RawMessageList::new(vec![]);
    let status = get_status(
        &mut server,
        "subscriptions/does_not_exist/pull",
        Method::POST,
        messages,
    );
    assert_eq!(StatusCode::NOT_FOUND, status);

    // Try and pull from a nonexistent subscription
    let pull_config = PullConfig::new(100);
    let status = get_status(
        &mut server,
        "subscriptions/does_not_exist/pull",
        Method::POST,
        pull_config,
    );
    assert_eq!(StatusCode::NOT_FOUND, status);

    // Try and ack to a nonexistent subscription
    let message_ids = MessageIdList::new(vec![]);
    let status = get_status(
        &mut server,
        "subscriptions/does_not_exist/ack",
        Method::POST,
        message_ids,
    );
    assert_eq!(StatusCode::NOT_FOUND, status);
}

#[test]
fn http_protocol_general_handlers() {
    let (_, mut server) = get_server();

    // let status = get_status_with_prefix(&mut server, "", "", Method::GET, ());
    // assert_eq!(StatusCode::OK, status);
    // let status = get_status_with_prefix(&mut server, "", "ui", Method::GET, ());
    // assert_eq!(StatusCode::OK, status);
    let status = get_status(&mut server, "heartbeat", Method::GET, ());
    assert_eq!(StatusCode::OK, status);
    let status = get_status(&mut server, "metrics", Method::GET, ());
    assert_eq!(StatusCode::OK, status);
}
