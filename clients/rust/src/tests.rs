use crate::types::*;
use crate::Client;
use std::collections::HashSet;
use std::iter::FromIterator;
use uuid::Uuid;

#[test]
fn client() {
    let client = Client::new("http://127.0.0.1:3140").unwrap();

    if !client.heartbeat() {
        return;
    }

    let topic_name = "rust-topic";
    let sub_name = "rust-sub";

    // Make sure everything we create does not exist
    let _ = client.delete_subscription(sub_name);
    let _ = client.delete_topic(topic_name);

    // Test create
    let topic = client
        .create_topic(
            &topic_name,
            &TopicCreateConfig {
                ttl: Some(3600),
                message_ttl: Some(60),
            },
        )
        .unwrap();
    assert_eq!(topic.name, topic_name);
    assert_eq!(topic.ttl, 3600);
    assert_eq!(topic.message_ttl, 60);

    let sub = client
        .create_subscription(
            &sub_name,
            &SubscriptionCreateConfig {
                topic: String::from(topic_name),
                ttl: Some(3600),
                ack_deadline: Some(60),
                historical: Some(false),
            },
        )
        .unwrap();
    assert_eq!(sub.name, sub_name);
    assert_eq!(sub.topic, topic_name);
    assert_eq!(sub.ttl, 3600);
    assert_eq!(sub.ack_deadline, 60);

    // Test update
    let topic = client
        .update_topic(
            topic_name,
            &TopicUpdateConfig {
                ttl: Some(5000),
                message_ttl: Some(12),
            },
        )
        .unwrap();
    assert_eq!(topic.name, topic_name);
    assert_eq!(topic.ttl, 5000);
    assert_eq!(topic.message_ttl, 12);

    let sub = client
        .update_subscription(
            sub_name,
            &SubscriptionUpdateConfig {
                ttl: Some(12000),
                ack_deadline: Some(72),
            },
        )
        .unwrap();
    assert_eq!(sub.name, sub_name);
    assert_eq!(sub.topic, topic_name);
    assert_eq!(sub.ttl, 12000);
    assert_eq!(sub.ack_deadline, 72);

    // Test get
    let topic = client.get_topic(topic_name).unwrap();
    assert_eq!(topic.name, topic_name);
    assert_eq!(topic.ttl, 5000);
    assert_eq!(topic.message_ttl, 12);

    let sub = client.get_subscription(sub_name).unwrap();
    assert_eq!(sub.name, sub_name);
    assert_eq!(sub.topic, topic_name);
    assert_eq!(sub.ttl, 12000);
    assert_eq!(sub.ack_deadline, 72);

    // Test create with uuid and list
    let topic = client
        .create_topic_with_uuid(&TopicCreateConfig {
            ttl: None,
            message_ttl: None,
        })
        .unwrap();
    let topics = client.list_topics().unwrap().topics;
    let topic_names = topics.iter().map(|t| t.name.clone());
    let topic_names_truth: HashSet<String> =
        HashSet::from_iter(vec![String::from(topic_name), topic.name.clone()]);
    assert_eq!(topic_names_truth, HashSet::from_iter(topic_names));

    let sub = client
        .create_subscription_with_uuid(&SubscriptionCreateConfig {
            topic: String::from(topic_name),
            ttl: None,
            ack_deadline: None,
            historical: None,
        })
        .unwrap();
    let subs = client.list_subscriptions().unwrap().subscriptions;
    let sub_names = subs.iter().map(|s| s.name.clone());
    let sub_names_truth: HashSet<String> =
        HashSet::from_iter(vec![String::from(sub_name), sub.name.clone()]);
    assert_eq!(sub_names_truth, HashSet::from_iter(sub_names));

    let sub_names = client
        .get_topic_subscriptions(topic_name)
        .unwrap()
        .subscription_names;
    assert_eq!(sub_names_truth, HashSet::from_iter(sub_names));

    // Test delete and list
    client.delete_subscription(&sub.name).unwrap();
    let subs = client.list_subscriptions().unwrap().subscriptions;
    let sub_names = subs.iter().map(|t| t.name.clone());
    let sub_names_truth: HashSet<String> = HashSet::from_iter(vec![String::from(sub_name)]);
    assert_eq!(sub_names_truth, HashSet::from_iter(sub_names));

    client.delete_topic(&topic.name).unwrap();
    let topics = client.list_topics().unwrap().topics;
    let topic_names = topics.iter().map(|t| t.name.clone());
    let topic_names_truth: HashSet<String> = HashSet::from_iter(vec![String::from(topic_name)]);
    assert_eq!(topic_names_truth, HashSet::from_iter(topic_names));

    // Test publish, pull, and ack
    client
        .publish_one(topic_name, String::from("data1"))
        .unwrap();
    client
        .publish(
            topic_name,
            vec![String::from("data2"), String::from("data3")],
        )
        .unwrap();

    let message1 = &client.pull_one(sub_name).unwrap().messages[0];
    let messages = client.pull(sub_name, 2).unwrap().messages;
    let message2 = &messages[0];
    let message3 = &messages[1];
    assert_eq!(message1.data, "data1");
    assert_eq!(message2.data, "data2");
    assert_eq!(message3.data, "data3");

    let message_ids = client
        .ack(sub_name, vec![message1.id, message2.id, message3.id])
        .unwrap()
        .message_ids;
    let message_ids_truth: HashSet<Uuid> =
        HashSet::from_iter(vec![message1.id, message2.id, message3.id]);
    assert_eq!(message_ids_truth, HashSet::from_iter(message_ids));

    // Delete what we created
    let _ = client.delete_subscription(sub_name);
    let _ = client.delete_topic(topic_name);
}
