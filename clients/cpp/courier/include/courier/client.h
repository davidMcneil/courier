#ifndef _COURIER_CLIENT_HH
#define _COURIER_CLIENT_HH

#include <memory>
#include <string>

#include "tl/expected.h"
#include "types.h"

namespace httplib {
class Client;
}

namespace courier {
using courier::Topic;
using courier::TopicCreateConfig;
using tl::expected;

enum class HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
};

class Client {
   public:
    static constexpr const char* topics_path = "/api/v0/topics";
    static constexpr const char* subscriptions_path = "/api/v0/subscriptions";

    Client(const std::string& base_url);

    expected<Topic, std::string> create_topic(const std::string& name,
                                              const TopicCreateConfig& config);

    expected<Topic, std::string> create_topic_with_uuid(
        const TopicCreateConfig& config);

    expected<Topic, std::string> update_topic(const std::string& name,
                                              const TopicUpdateConfig& config);

    expected<int, std::string> delete_topic(const std::string& name);

    expected<Topic, std::string> get_topic(const std::string& name);

    expected<TopicList, std::string> list_topics();

    expected<MessageIdList, std::string> publish(const std::string& topic,
                                                 const std::string& data);

    expected<MessageIdList, std::string> publish(
        const std::string& topic, const std::vector<std::string>& data);

    expected<SubscriptionNameList, std::string> get_topic_subscriptions(
        const std::string& topic);

    expected<Subscription, std::string> create_subscription(
        const std::string& name, const SubscriptionCreateConfig& config);

    expected<Subscription, std::string> create_subscription_with_uuid(
        const SubscriptionCreateConfig& config);

    expected<Subscription, std::string> update_subscription(
        const std::string& name, const SubscriptionUpdateConfig& config);

    expected<int, std::string> delete_subscription(const std::string& name);

    expected<Subscription, std::string> get_subscription(
        const std::string& name);

    expected<SubscriptionList, std::string> list_subscriptions();

    expected<MessageList, std::string> pull(const std::string& subscription,
                                            uint64_t max_messages = 1);

    expected<MessageIdList, std::string> ack(const std::string& subscription,
                                             const std::string& message_id);

    expected<MessageIdList, std::string> ack(
        const std::string& subscription,
        const std::vector<std::string>& message_ids);

   private:
    std::string base_url;
};
}  // namespace courier

#endif  // _COURIER_TYPES_HH_
