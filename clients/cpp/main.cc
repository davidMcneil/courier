#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN

#include <set>

#include "doctest/doctest.h"
#include "tl/expected.h"

#include "courier/client.h"
#include "courier/types.h"

using namespace courier;
using std::set;
using std::string;
using std::vector;
using tl::expected;

TEST_CASE("courier") {
    Client client("http://127.0.0.1:3140/");

    string topic_name = "cpp-topic";
    string sub_name = "cpp-sub";

    // Make sure everything we create does not exist
    client.delete_topic(topic_name);
    client.delete_subscription(sub_name);

    // Test create
    TopicCreateConfig topic_create_config;
    topic_create_config.ttl = 3600;
    topic_create_config.message_ttl = 60;
    Topic topic = *client.create_topic(topic_name, topic_create_config);
    CHECK(topic.name == topic_name);
    CHECK(topic.ttl == 3600);
    CHECK(topic.message_ttl == 60);

    SubscriptionCreateConfig subscription_create_config;
    subscription_create_config.topic = topic_name;
    subscription_create_config.ttl = 3600;
    subscription_create_config.ack_deadline = 60;
    Subscription sub =
        *client.create_subscription(sub_name, subscription_create_config);
    CHECK(sub.name == sub_name);
    CHECK(sub.topic == topic_name);
    CHECK(sub.ttl == 3600);
    CHECK(sub.ack_deadline == 60);

    // Test update
    TopicUpdateConfig topic_update_config;
    topic_update_config.ttl = 5000;
    topic_update_config.message_ttl = 12;
    topic = *client.update_topic(topic_name, topic_update_config);
    CHECK(topic.name == topic_name);
    CHECK(topic.ttl == 5000);
    CHECK(topic.message_ttl == 12);

    SubscriptionUpdateConfig subscription_update_config;
    subscription_update_config.ttl = 12000;
    subscription_update_config.ack_deadline = 72;
    sub = *client.update_subscription(sub_name, subscription_update_config);
    CHECK(sub.name == sub_name);
    CHECK(sub.topic == topic_name);
    CHECK(sub.ttl == 12000);
    CHECK(sub.ack_deadline == 72);

    // Test get
    topic = *client.get_topic(topic_name);
    CHECK(topic.name == topic_name);
    CHECK(topic.ttl == 5000);
    CHECK(topic.message_ttl == 12);

    sub = *client.get_subscription(sub_name);
    CHECK(sub.name == sub_name);
    CHECK(sub.topic == topic_name);
    CHECK(sub.ttl == 12000);
    CHECK(sub.ack_deadline == 72);

    // Test create with uuid and list
    topic = *client.create_topic_with_uuid(topic_create_config);
    vector<Topic> topics = (*client.list_topics()).topics;
    vector<string> topic_names;
    for (const Topic& t : topics) {
        topic_names.push_back(t.name);
    }
    vector<string> topic_names_truth = {topic_name, topic.name};
    CHECK(set<string>(topic_names_truth.begin(), topic_names_truth.end()) ==
          set<string>(topic_names.begin(), topic_names.end()));

    subscription_create_config.topic = topic_name;
    sub = *client.create_subscription_with_uuid(subscription_create_config);
    vector<Subscription> subs = (*client.list_subscriptions()).subscriptions;
    vector<string> sub_names;
    for (const Subscription& s : subs) {
        sub_names.push_back(s.name);
    }
    vector<string> sub_names_truth = {sub_name, sub.name};
    CHECK(set<string>(sub_names_truth.begin(), sub_names_truth.end()) ==
          set<string>(sub_names.begin(), sub_names.end()));

    sub_names =
        (*client.get_topic_subscriptions(topic_name)).subscription_names;
    sub_names_truth = {sub_name, sub.name};
    CHECK(set<string>(sub_names_truth.begin(), sub_names_truth.end()) ==
          set<string>(sub_names.begin(), sub_names.end()));

    // Test delete and list
    client.delete_subscription(sub.name);
    subs = (*client.list_subscriptions()).subscriptions;
    sub_names.clear();
    for (const Subscription& s : subs) {
        sub_names.push_back(s.name);
    }
    sub_names_truth = {sub_name};
    CHECK(set<string>(sub_names_truth.begin(), sub_names_truth.end()) ==
          set<string>(sub_names.begin(), sub_names.end()));

    client.delete_topic(topic.name);
    topics = (*client.list_topics()).topics;
    topic_names.clear();
    for (const Topic& t : topics) {
        topic_names.push_back(t.name);
    }
    topic_names_truth = {topic_name};
    CHECK(set<string>(topic_names_truth.begin(), topic_names_truth.end()) ==
          set<string>(topic_names.begin(), topic_names.end()));

    // Test publish, pull, and ack
    client.publish(topic_name, "data1");
    client.publish(topic_name, vector<string>({"data2", "data3"}));

    Message message1 = (*client.pull(sub_name)).messages[0];
    vector<Message> messages = (*client.pull(sub_name, 2)).messages;
    Message message2 = messages[0];
    Message message3 = messages[1];
    CHECK(message1.data == "data1");
    CHECK(message2.data == "data2");
    CHECK(message3.data == "data3");

    vector<string> message_ids =
        (*client.ack(sub_name, {message1.id, message2.id, message3.id}))
            .message_ids;
    vector<string> message_ids_truth = {message1.id, message2.id, message3.id};
    CHECK(set<string>(message_ids_truth.begin(), message_ids_truth.end()) ==
          set<string>(message_ids.begin(), message_ids.end()));
}