#include "courier/types.h"

namespace nlohmann {
template <typename T>
struct adl_serializer<nonstd::optional<T>> {
    static void to_json(json& j, const nonstd::optional<T>& opt) {
        if (opt) {
            j = *opt;
        } else {
            j = nullptr;
        }
    }

    static void from_json(const json& j, nonstd::optional<T>& opt) {
        if (j.is_null()) {
            opt.reset();
        } else {
            opt = j.get<T>();
        }
    }
};
}  // namespace nlohmann

namespace courier {
void to_json(json& j, const Message& m) {
    j = json{
        {"id", m.id}, {"time", m.time}, {"tries", m.tries}, {"data", m.data}};
}

void from_json(const json& j, Message& m) {
    m.id = j.at("id").get<std::string>();
    m.time = j.at("time").get<std::string>();
    m.tries = j.at("tries").get<int32_t>();
    m.data = j.at("data").get<std::string>();
}

void to_json(json& j, const Subscription& s) {
    j = json{{"name", s.name},
             {"topic", s.topic},
             {"ack_deadline", s.ack_deadline},
             {"ttl", s.ttl},
             {"created", s.created},
             {"updated", s.updated}};
}

void from_json(const json& j, Subscription& s) {
    s.name = j.at("name").get<std::string>();
    s.topic = j.at("topic").get<std::string>();
    s.ack_deadline = j.at("ack_deadline").get<int64_t>();
    s.ttl = j.at("ttl").get<int64_t>();
    s.created = j.at("created").get<datetime>();
    s.updated = j.at("updated").get<datetime>();
}

void to_json(json& j, const Topic& t) {
    j = json{
        {"name", t.name},       {"message_ttl", t.message_ttl}, {"ttl", t.ttl},
        {"created", t.created}, {"updated", t.updated},
    };
}

void from_json(const json& j, Topic& t) {
    t.name = j.at("name").get<std::string>();
    t.message_ttl = j.at("message_ttl").get<int64_t>();
    t.ttl = j.at("ttl").get<int64_t>();
    t.created = j.at("created").get<datetime>();
    t.updated = j.at("updated").get<datetime>();
}

void to_json(json& j, const TopicCreateConfig& t) {
    j = json{{"message_ttl", t.message_ttl}, {"ttl", t.ttl}};
}

void from_json(const json& j, TopicCreateConfig& t) {
    t.message_ttl = j.at("message_ttl").get<optional<int32_t>>();
    t.ttl = j.at("ttl").get<optional<int32_t>>();
}

void to_json(json& j, const TopicUpdateConfig& t) {
    j = json{{"message_ttl", t.message_ttl}, {"ttl", t.ttl}};
}

void from_json(const json& j, TopicUpdateConfig& t) {
    t.message_ttl = j.at("message_ttl").get<optional<int32_t>>();
    t.ttl = j.at("ttl").get<optional<int32_t>>();
}

void to_json(json& j, const TopicList& t) { j = json{{"topics", t.topics}}; }

void from_json(const json& j, TopicList& t) {
    t.topics = j.at("topics").get<std::vector<Topic>>();
}

void to_json(json& j, const RawMessage& r) { j = json{{"data", r.data}}; }

void from_json(const json& j, RawMessage& r) {
    r.data = j.at("data").get<std::string>();
}

void to_json(json& j, const RawMessageList& r) {
    j = json{{"raw_messages", r.raw_messages}};
}

void from_json(const json& j, RawMessageList& r) {
    r.raw_messages = j.at("raw_messages").get<std::vector<RawMessage>>();
}

void to_json(json& j, const MessageIdList& m) {
    j = json{{"message_ids", m.message_ids}};
}

void from_json(const json& j, MessageIdList& m) {
    m.message_ids = j.at("message_ids").get<std::vector<std::string>>();
}

void to_json(json& j, const SubscriptionNameList& s) {
    j = json{{"subscription_names", s.subscription_names}};
}

void from_json(const json& j, SubscriptionNameList& s) {
    s.subscription_names =
        j.at("subscription_names").get<std::vector<std::string>>();
}

void to_json(json& j, const SubscriptionCreateConfig& s) {
    j = json{{"topic", s.topic},
             {"ack_deadline", s.ack_deadline},
             {"ttl", s.ttl},
             {"historical", s.historical}};
}

void from_json(const json& j, SubscriptionCreateConfig& s) {
    s.topic = j.at("topic").get<std::string>();
    s.ack_deadline = j.at("ack_deadline").get<optional<int32_t>>();
    s.ttl = j.at("ttl").get<optional<int32_t>>();
    s.historical = j.at("historical").get<optional<bool>>();
}

void to_json(json& j, const SubscriptionUpdateConfig& s) {
    j = json{
        {"ack_deadline", s.ack_deadline},
        {"ttl", s.ttl},
    };
}

void from_json(const json& j, SubscriptionUpdateConfig& s) {
    s.ack_deadline = j.at("ack_deadline").get<optional<int32_t>>();
    s.ttl = j.at("ttl").get<optional<int32_t>>();
}

void to_json(json& j, const SubscriptionList& s) {
    j = json{{"subscriptions", s.subscriptions}};
}

void from_json(const json& j, SubscriptionList& s) {
    s.subscriptions = j.at("subscriptions").get<std::vector<Subscription>>();
}

void to_json(json& j, const MessageList& m) {
    j = json{{"messages", m.messages}};
}

void from_json(const json& j, MessageList& m) {
    m.messages = j.at("messages").get<std::vector<Message>>();
}

void to_json(json& j, const PullConfig& p) {
    j = json{{"max_messages", p.max_messages}};
}

void from_json(const json& j, PullConfig& p) {
    p.max_messages = j.at("max_messages").get<optional<uint64_t>>();
}

}  // namespace courier