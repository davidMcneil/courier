#include <memory>
#include <string>

#include "courier/client.h"
#include "cpr/cpr.h"
#include "nlohmann/json.h"
#include "tl/expected.h"

namespace {}

namespace courier {

using nonstd::optional;
using tl::expected;
using json = nlohmann::json;

Client::Client(const std::string& base_url) {
    this->base_url = base_url;
    if (!base_url.empty() && base_url.back() == '/') {
        this->base_url.pop_back();
    }
}

expected<Topic, std::string> Client::create_topic(
    const std::string& name, const TopicCreateConfig& config) {
    std::string url = this->base_url + Client::topics_path + "/" + name;

    // Parse the request body
    json req_json;
    to_json(req_json, config);

    // Send the request
    cpr::Response res =
        cpr::Put(cpr::Url{url}, cpr::Body{req_json.dump()},
                 cpr::Header{{"Content-Type", "application/json"}});

    // Check the status code
    int status_code = res.status_code;
    if (400 <= status_code && status_code < 500) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " client error for url: " + url);
    } else if (500 <= status_code && status_code < 600) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " server error for url: " + url);
    }

    // Parse the response body
    Topic topic;
    try {
        json res_json = json::parse(res.text);
        from_json(res_json, topic);
    } catch (json::exception& e) {
        return tl::make_unexpected(std::string(e.what()));
    }

    return expected<Topic, std::string>(topic);
}

expected<Topic, std::string> Client::create_topic_with_uuid(
    const TopicCreateConfig& config) {
    return this->create_topic("", config);
}

expected<Topic, std::string> Client::update_topic(
    const std::string& name, const TopicUpdateConfig& config) {
    std::string url = this->base_url + Client::topics_path + "/" + name;

    // Parse the request body
    json req_json;
    to_json(req_json, config);

    // Send the request
    cpr::Response res =
        cpr::Patch(cpr::Url{url}, cpr::Body{req_json.dump()},
                   cpr::Header{{"Content-Type", "application/json"}});

    // Check the status code
    int status_code = res.status_code;
    if (400 <= status_code && status_code < 500) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " client error for url: " + url);
    } else if (500 <= status_code && status_code < 600) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " server error for url: " + url);
    }

    // Parse the response body
    Topic topic;
    try {
        json res_json = json::parse(res.text);
        from_json(res_json, topic);
    } catch (json::exception& e) {
        return tl::make_unexpected(std::string(e.what()));
    }

    return expected<Topic, std::string>(topic);
}

expected<int, std::string> Client::delete_topic(const std::string& name) {
    std::string url = this->base_url + Client::topics_path + "/" + name;

    // Send the request
    cpr::Response res = cpr::Delete(
        cpr::Url{url}, cpr::Header{{"Content-Type", "application/json"}});

    // Check the status code
    int status_code = res.status_code;
    if (400 <= status_code && status_code < 500) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " client error for url: " + url);
    } else if (500 <= status_code && status_code < 600) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " server error for url: " + url);
    }

    return expected<int, std::string>(0);
}

expected<Topic, std::string> Client::get_topic(const std::string& name) {
    std::string url = this->base_url + Client::topics_path + "/" + name;

    // Send the request
    cpr::Response res = cpr::Get(
        cpr::Url{url}, cpr::Header{{"Content-Type", "application/json"}});

    // Check the status code
    int status_code = res.status_code;
    if (400 <= status_code && status_code < 500) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " client error for url: " + url);
    } else if (500 <= status_code && status_code < 600) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " server error for url: " + url);
    }

    // Parse the response body
    Topic topic;
    try {
        json res_json = json::parse(res.text);
        from_json(res_json, topic);
    } catch (json::exception& e) {
        return tl::make_unexpected(std::string(e.what()));
    }

    return expected<Topic, std::string>(topic);
}

expected<TopicList, std::string> Client::list_topics() {
    std::string url = this->base_url + Client::topics_path + "/";

    // Send the request
    cpr::Response res = cpr::Get(
        cpr::Url{url}, cpr::Header{{"Content-Type", "application/json"}});

    // Check the status code
    int status_code = res.status_code;
    if (400 <= status_code && status_code < 500) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " client error for url: " + url);
    } else if (500 <= status_code && status_code < 600) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " server error for url: " + url);
    }

    // Parse the response body
    TopicList topic_list;
    try {
        json res_json = json::parse(res.text);
        from_json(res_json, topic_list);
    } catch (json::exception& e) {
        return tl::make_unexpected(std::string(e.what()));
    }

    return expected<TopicList, std::string>(topic_list);
}

expected<MessageIdList, std::string> Client::publish(const std::string& topic,
                                                     const std::string& data) {
    return publish(topic, std::vector<std::string>({data}));
}

expected<MessageIdList, std::string> Client::publish(
    const std::string& topic, const std::vector<std::string>& data) {
    std::string url =
        this->base_url + Client::topics_path + "/" + topic + "/publish";

    // Build up the request body
    std::vector<RawMessage> raw_messages;
    for (const std::string& d : data) {
        raw_messages.push_back(RawMessage::create(d));
    }

    // Parse the request body
    json req_json;
    to_json(req_json, RawMessageList::create(raw_messages));

    // Send the request
    cpr::Response res =
        cpr::Post(cpr::Url{url}, cpr::Body{req_json.dump()},
                  cpr::Header{{"Content-Type", "application/json"}});

    // Check the status code
    int status_code = res.status_code;
    if (400 <= status_code && status_code < 500) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " client error for url: " + url);
    } else if (500 <= status_code && status_code < 600) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " server error for url: " + url);
    }

    // Parse the response body
    MessageIdList message_id_list;
    try {
        json res_json = json::parse(res.text);
        from_json(res_json, message_id_list);
    } catch (json::exception& e) {
        return tl::make_unexpected(std::string(e.what()));
    }

    return expected<MessageIdList, std::string>(message_id_list);
}

expected<SubscriptionNameList, std::string> Client::get_topic_subscriptions(
    const std::string& topic) {
    std::string url =
        this->base_url + Client::topics_path + "/" + topic + "/subscriptions";

    // Send the request
    cpr::Response res = cpr::Get(
        cpr::Url{url}, cpr::Header{{"Content-Type", "application/json"}});

    // Check the status code
    int status_code = res.status_code;
    if (400 <= status_code && status_code < 500) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " client error for url: " + url);
    } else if (500 <= status_code && status_code < 600) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " server error for url: " + url);
    }

    // Parse the response body
    SubscriptionNameList subscription_name_list;
    try {
        json res_json = json::parse(res.text);
        from_json(res_json, subscription_name_list);
    } catch (json::exception& e) {
        return tl::make_unexpected(std::string(e.what()));
    }

    return expected<SubscriptionNameList, std::string>(subscription_name_list);
}

expected<Subscription, std::string> Client::create_subscription(
    const std::string& name, const SubscriptionCreateConfig& config) {
    std::string url = this->base_url + Client::subscriptions_path + "/" + name;

    // Parse the request body
    json req_json;
    to_json(req_json, config);

    // Send the request
    cpr::Response res =
        cpr::Put(cpr::Url{url}, cpr::Body{req_json.dump()},
                 cpr::Header{{"Content-Type", "application/json"}});

    // Check the status code
    int status_code = res.status_code;
    if (400 <= status_code && status_code < 500) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " client error for url: " + url);
    } else if (500 <= status_code && status_code < 600) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " server error for url: " + url);
    }

    // Parse the response body
    Subscription subscription;
    try {
        json res_json = json::parse(res.text);
        from_json(res_json, subscription);
    } catch (json::exception& e) {
        return tl::make_unexpected(std::string(e.what()));
    }

    return expected<Subscription, std::string>(subscription);
}

expected<Subscription, std::string> Client::create_subscription_with_uuid(
    const SubscriptionCreateConfig& config) {
    return this->create_subscription("", config);
}

expected<Subscription, std::string> Client::update_subscription(
    const std::string& name, const SubscriptionUpdateConfig& config) {
    std::string url = this->base_url + Client::subscriptions_path + "/" + name;

    // Parse the request body
    json req_json;
    to_json(req_json, config);

    // Send the request
    cpr::Response res =
        cpr::Patch(cpr::Url{url}, cpr::Body{req_json.dump()},
                   cpr::Header{{"Content-Type", "application/json"}});

    // Check the status code
    int status_code = res.status_code;
    if (400 <= status_code && status_code < 500) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " client error for url: " + url);
    } else if (500 <= status_code && status_code < 600) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " server error for url: " + url);
    }

    // Parse the response body
    Subscription subscription;
    try {
        json res_json = json::parse(res.text);
        from_json(res_json, subscription);
    } catch (json::exception& e) {
        return tl::make_unexpected(std::string(e.what()));
    }

    return expected<Subscription, std::string>(subscription);
}

expected<int, std::string> Client::delete_subscription(
    const std::string& name) {
    std::string url = this->base_url + Client::subscriptions_path + "/" + name;

    // Send the request
    cpr::Response res = cpr::Delete(
        cpr::Url{url}, cpr::Header{{"Content-Type", "application/json"}});

    // Check the status code
    int status_code = res.status_code;
    if (400 <= status_code && status_code < 500) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " client error for url: " + url);
    } else if (500 <= status_code && status_code < 600) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " server error for url: " + url);
    }

    return expected<int, std::string>(0);
}

expected<Subscription, std::string> Client::get_subscription(
    const std::string& name) {
    std::string url = this->base_url + Client::subscriptions_path + "/" + name;

    // Send the request
    cpr::Response res = cpr::Get(
        cpr::Url{url}, cpr::Header{{"Content-Type", "application/json"}});

    // Check the status code
    int status_code = res.status_code;
    if (400 <= status_code && status_code < 500) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " client error for url: " + url);
    } else if (500 <= status_code && status_code < 600) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " server error for url: " + url);
    }

    // Parse the response body
    Subscription subscription;
    try {
        json res_json = json::parse(res.text);
        from_json(res_json, subscription);
    } catch (json::exception& e) {
        return tl::make_unexpected(std::string(e.what()));
    }

    return expected<Subscription, std::string>(subscription);
}

expected<SubscriptionList, std::string> Client::list_subscriptions() {
    std::string url = this->base_url + Client::subscriptions_path + "/";

    // Send the request
    cpr::Response res = cpr::Get(
        cpr::Url{url}, cpr::Header{{"Content-Type", "application/json"}});

    // Check the status code
    int status_code = res.status_code;
    if (400 <= status_code && status_code < 500) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " client error for url: " + url);
    } else if (500 <= status_code && status_code < 600) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " server error for url: " + url);
    }

    // Parse the response body
    SubscriptionList subscription_list;
    try {
        json res_json = json::parse(res.text);
        from_json(res_json, subscription_list);
    } catch (json::exception& e) {
        return tl::make_unexpected(std::string(e.what()));
    }

    return expected<SubscriptionList, std::string>(subscription_list);
}

expected<MessageList, std::string> Client::pull(const std::string& subscription,
                                                uint64_t max_messages) {
    std::string url = this->base_url + Client::subscriptions_path + "/" +
                      subscription + "/pull";

    // Parse the request body
    json req_json;
    to_json(req_json, PullConfig::create(max_messages));

    // Send the request
    cpr::Response res =
        cpr::Post(cpr::Url{url}, cpr::Body{req_json.dump()},
                  cpr::Header{{"Content-Type", "application/json"}});

    // Check the status code
    int status_code = res.status_code;
    if (400 <= status_code && status_code < 500) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " client error for url: " + url);
    } else if (500 <= status_code && status_code < 600) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " server error for url: " + url);
    }

    // Parse the response body
    MessageList message_list;
    try {
        json res_json = json::parse(res.text);
        from_json(res_json, message_list);
    } catch (json::exception& e) {
        return tl::make_unexpected(std::string(e.what()));
    }

    return expected<MessageList, std::string>(message_list);
}

expected<MessageIdList, std::string> Client::ack(
    const std::string& subscription, const std::string& message_id) {
    return this->ack(subscription, {message_id});
}

expected<MessageIdList, std::string> Client::ack(
    const std::string& subscription,
    const std::vector<std::string>& message_ids) {
    std::string url = this->base_url + Client::subscriptions_path + "/" +
                      subscription + "/ack";

    // Parse the request body
    json req_json;
    to_json(req_json, MessageIdList::create(message_ids));

    // Send the request
    cpr::Response res =
        cpr::Post(cpr::Url{url}, cpr::Body{req_json.dump()},
                  cpr::Header{{"Content-Type", "application/json"}});

    // Check the status code
    int status_code = res.status_code;
    if (400 <= status_code && status_code < 500) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " client error for url: " + url);
    } else if (500 <= status_code && status_code < 600) {
        std::string status_code_str = std::to_string(status_code);
        return tl::make_unexpected(status_code_str +
                                   " server error for url: " + url);
    }

    // Parse the response body
    MessageIdList message_id_list;
    try {
        json res_json = json::parse(res.text);
        from_json(res_json, message_id_list);
    } catch (json::exception& e) {
        return tl::make_unexpected(std::string(e.what()));
    }

    return expected<MessageIdList, std::string>(message_id_list);
}

}  // namespace courier
