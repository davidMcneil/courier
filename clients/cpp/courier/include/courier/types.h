#ifndef _COURIER_TYPES_HH
#define _COURIER_TYPES_HH

#include <string>
#include <vector>

#include "nlohmann/json.h"
#include "nonstd/optional.h"

namespace nlohmann {
template <typename T>
struct adl_serializer<nonstd::optional<T>>;
}  // namespace nlohmann

namespace courier {

using nonstd::optional;
using json = nlohmann::json;

using datetime = std::string;

/// A message which can be published to a [Topic](struct.Topic.html).
struct Message {
    /// Unique identifier for this message.
    std::string id;
    /// Time the message was published.
    datetime time;
    /// Number of times the message has been tried (pulled).
    int32_t tries;
    /// Actual message data.
    std::string data;
    Message() : tries(0) {}
};

void to_json(json& j, const Message& m);
void from_json(const json& j, Message& m);

/// A subscription meta type used for serialization.
struct Subscription {
    /// Unique name for this subscription.
    std::string name;
    /// Topic name the subscription is subscribed to.
    std::string topic;
    /// Amount of time given to ack a message in seconds.
    int64_t ack_deadline;
    /// Time to live of the subscription in seconds.
    int64_t ttl;
    /// Time the subscription was created.
    datetime created;
    /// Time the subscription was last updated.
    datetime updated;
    Subscription() : ack_deadline(0), ttl(0) {}
};

void to_json(json& j, const Subscription& s);
void from_json(const json& j, Subscription& s);

/// A topic meta type used for serialization.
struct Topic {
    /// Unique name of the topic.
    std::string name;
    /// Message time to live in seconds.
    int64_t message_ttl;
    /// Time to live of the topic in seconds.
    int64_t ttl;
    /// Time the topic was created.
    datetime created;
    /// Time the topic was updated.
    datetime updated;
    Topic() : message_ttl(0), ttl(0) {}
};

void to_json(json& j, const Topic& t);
void from_json(const json& j, Topic& t);

struct TopicCreateConfig {
    optional<int32_t> message_ttl;
    optional<int32_t> ttl;
    TopicCreateConfig() {}
};

void to_json(json& j, const TopicCreateConfig& t);

void from_json(const json& j, TopicCreateConfig& t);

struct TopicUpdateConfig {
    optional<int32_t> message_ttl;
    optional<int32_t> ttl;
    TopicUpdateConfig() {}
};

void to_json(json& j, const TopicUpdateConfig& t);

void from_json(const json& j, TopicUpdateConfig& t);

struct TopicList {
    std::vector<Topic> topics;
    TopicList() {}
};

void to_json(json& j, const TopicList& t);
void from_json(const json& j, TopicList& t);

struct RawMessage {
    std::string data;
    RawMessage() {}
    static RawMessage create(std::string data) {
        RawMessage raw_message;
        raw_message.data = data;
        return raw_message;
    }
};

void to_json(json& j, const RawMessage& r);
void from_json(const json& j, RawMessage& r);

struct RawMessageList {
    std::vector<RawMessage> raw_messages;
    RawMessageList() {}
    static RawMessageList create(const std::vector<RawMessage>& raw_messages) {
        RawMessageList raw_message_list;
        raw_message_list.raw_messages = raw_messages;
        return raw_message_list;
    }
};

void to_json(json& j, const RawMessageList& r);
void from_json(const json& j, RawMessageList& r);

struct MessageIdList {
    std::vector<std::string> message_ids;
    MessageIdList() {}
    static MessageIdList create(const std::vector<std::string>& message_ids) {
        MessageIdList message_id_list;
        message_id_list.message_ids = message_ids;
        return message_id_list;
    }
};

void to_json(json& j, const MessageIdList& m);
void from_json(const json& j, MessageIdList& m);

struct SubscriptionNameList {
    std::vector<std::string> subscription_names;
    SubscriptionNameList() {}
};

void to_json(json& j, const SubscriptionNameList& s);
void from_json(const json& j, SubscriptionNameList& s);

struct SubscriptionCreateConfig {
    std::string topic;
    optional<int32_t> ack_deadline;
    optional<int32_t> ttl;
    optional<bool> historical;
    SubscriptionCreateConfig() {}
};

void to_json(json& j, const SubscriptionCreateConfig& s);
void from_json(const json& j, SubscriptionCreateConfig& s);

struct SubscriptionUpdateConfig {
    optional<int32_t> ack_deadline;
    optional<int32_t> ttl;
    SubscriptionUpdateConfig() {}
};

void to_json(json& j, const SubscriptionUpdateConfig& s);
void from_json(const json& j, SubscriptionUpdateConfig& s);

struct SubscriptionList {
    std::vector<Subscription> subscriptions;
    SubscriptionList() {}
};

void to_json(json& j, const SubscriptionList& s);
void from_json(const json& j, SubscriptionList& s);

struct MessageList {
    std::vector<Message> messages;
    MessageList() {}
};

void to_json(json& j, const MessageList& m);
void from_json(const json& j, MessageList& m);

struct PullConfig {
    optional<uint64_t> max_messages;
    PullConfig() {}
    static PullConfig create(uint64_t max_messages) {
        PullConfig pull_config;
        pull_config.max_messages = max_messages;
        return pull_config;
    }
};

void to_json(json& j, const PullConfig& p);
void from_json(const json& j, PullConfig& p);

}  // namespace courier

#endif  // _COURIER_TYPES_HH_