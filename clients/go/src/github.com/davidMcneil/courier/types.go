package courier

import "time"

// Message can be published Topic
type Message struct {
	// Unique identifier for this message.
	ID string `json:"id"`
	// Time the message was published.
	Time time.Time `json:"time"`
	// Number of times the message has been tried (pulled).
	Tries uint32 `json:"tries"`
	// Actual message data.
	Data string `json:"data"`
}

// Subscription can subscribe to a topic
type Subscription struct {
	// Unique name for this subscription.
	Name string `json:"name"`
	// Topic name the subscription is subscribed to.
	Topic string `json:"topic"`
	// Amount of time given to ack a message in seconds.
	AckDeadline int64 `json:"ack_deadline"`
	// Time to live of the subscription in seconds.
	TTL int64 `json:"ttl"`
	// Time the subscription was created.
	Created time.Time `json:"created"`
	// Time the subscription was last updated.
	Updated time.Time `json:"updated"`
}

// Topic can publish messages and have subscriptions.
type Topic struct {
	// Unique name of the topic.
	Name string `json:"name"`
	// Message time to live in seconds.
	MessageTTL int64 `json:"message_ttl"`
	// Time to live of the topic in seconds.
	TTL int64 `json:"ttl"`
	// Time the topic was created.
	Created time.Time `json:"created"`
	// Time the topic was updated.
	Updated time.Time `json:"updated"`
}

// TopicCreateConfig TODO
type TopicCreateConfig struct {
	MessageTTL *uint32 `json:"message_ttl,omitempty"`
	TTL        *uint32 `json:"ttl,omitempty"`
}

// impl TopicCreateConfig {
//     fn new() -> Self {
//         Default::default()
//     }
// }

// TopicUpdateConfig TODO
type TopicUpdateConfig struct {
	MessageTTL *uint32 `json:"message_ttl,omitempty"`
	TTL        *uint32 `json:"ttl,omitempty"`
}

// TopicList TODO
type TopicList struct {
	Topics []Topic `json:"topics"`
}

// impl TopicList {
//     fn new(topics Vec<Topic>) -> Self
//         Self { topics }
//     }
// }

// RawMessage TODO
type RawMessage struct {
	Data string `json:"data"`
}

// NewRawMessage TODO
func NewRawMessage(data string) *RawMessage {
	return &RawMessage{Data: data}
}

// RawMessageList TODO
type RawMessageList struct {
	RawMessages []*RawMessage `json:"raw_messages"`
}

// NewRawMessageList TODO
func NewRawMessageList(rawMessages []*RawMessage) RawMessageList {
	return RawMessageList{RawMessages: rawMessages}
}

// MessageIDList TODO
type MessageIDList struct {
	MessageIds []string `json:"message_ids"`
}

// NewMessageIDList TODO
func NewMessageIDList(messageIds []string) MessageIDList {
	return MessageIDList{MessageIds: messageIds}
}

// SubscriptionNameList TODO
type SubscriptionNameList struct {
	SubscriptionNames []string `json:"subscription_names"`
}

// SubscriptionCreateConfig TODO
type SubscriptionCreateConfig struct {
	Topic       string  `json:"topic"`
	AckDeadline *uint32 `json:"ack_deadline,omitempty"`
	TTL         *uint32 `json:"ttl,omitempty"`
	Historical  *bool   `json:"historical,omitempty"`
}

// impl SubscriptionCreateConfig {
//     fn new(topic &str) -> Self
//         SubscriptionCreateConfig {
//             topic string::from(topic)
//             ack_deadline None
//             ttl None
//             historical None
//         }
//     }
// }

// SubscriptionUpdateConfig TODO
type SubscriptionUpdateConfig struct {
	AckDeadline uint32 `json:"ack_deadline,omitempty"`
	TTL         uint32 `json:"ttl,omitempty"`
}

// SubscriptionList TODO
type SubscriptionList struct {
	Subscriptions []Subscription `json:"subscriptions"`
}

// impl SubscriptionList {
//     fn new(subscriptions Vec<Subscription>) -> Self
//         Self { subscriptions }
//     }
// }

// MessageList TODO
type MessageList struct {
	Messages []Message `json:"messages"`
}

// impl MessageList {
//     fn new(messages Vec<Message>) -> Self
//         Self { messages }
//     }
// }

// PullConfig TODO
type PullConfig struct {
	MaxMessages *uint `json:"max_messages,omitempty"`
}

// NewPullConfig TODO
func NewPullConfig(maxMessages uint) *PullConfig {
	return &PullConfig{
		MaxMessages: &maxMessages,
	}
}
