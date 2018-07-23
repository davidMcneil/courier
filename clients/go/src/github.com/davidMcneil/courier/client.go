package courier

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"net/url"
	"path"
)

const topicsPath = "/api/v0/topics"
const subscriptionsPath = "/api/v0/subscriptions"

// Client TODO
type Client struct {
	baseURL *url.URL
	http    http.Client
}

// NewClient TODO
func NewClient(baseURLString string) (*Client, error) {
	baseURL, err := url.Parse(baseURLString)
	if err != nil {
		return nil, err
	}
	return &Client{
		baseURL: baseURL,
		http:    http.Client{},
	}, nil
}

// CreateTopic TODO
func (c *Client) CreateTopic(name string, config TopicCreateConfig) (*Topic, error) {
	url := c.baseURL.String() + path.Join(topicsPath, name)
	if name == "" {
		url += "/"
	}

	body, err := json.Marshal(config)
	if err != nil {
		return nil, err
	}

	responseBody, err := c.send(http.MethodPut, url, bytes.NewBuffer(body))
	if err != nil {
		return nil, err
	}

	topic := &Topic{}
	err = json.Unmarshal(responseBody, topic)
	if err != nil {
		return nil, err
	}
	return topic, nil
}

// CreateTopicWithUUID TODO
func (c *Client) CreateTopicWithUUID(config TopicCreateConfig) (*Topic, error) {
	return c.CreateTopic("", config)
}

// UpdateTopic TODO
func (c *Client) UpdateTopic(name string, config TopicUpdateConfig) (*Topic, error) {
	url := c.baseURL.String() + path.Join(topicsPath, name)

	body, err := json.Marshal(config)
	if err != nil {
		return nil, err
	}

	responseBody, err := c.send(http.MethodPatch, url, bytes.NewBuffer(body))
	if err != nil {
		return nil, err
	}

	topic := &Topic{}
	err = json.Unmarshal(responseBody, topic)
	if err != nil {
		return nil, err
	}
	return topic, nil
}

// DeleteTopic TODO
func (c *Client) DeleteTopic(name string) error {
	url := c.baseURL.String() + path.Join(topicsPath, name)

	_, err := c.send(http.MethodDelete, url, nil)
	if err != nil {
		return err
	}

	return nil
}

// GetTopic TODO
func (c *Client) GetTopic(name string) (*Topic, error) {
	url := c.baseURL.String() + path.Join(topicsPath, name)

	responseBody, err := c.send(http.MethodGet, url, nil)
	if err != nil {
		return nil, err
	}

	topic := &Topic{}
	err = json.Unmarshal(responseBody, topic)
	if err != nil {
		return nil, err
	}
	return topic, nil
}

// ListTopics TODO
func (c *Client) ListTopics() (*TopicList, error) {
	url := c.baseURL.String() + topicsPath + "/"

	responseBody, err := c.send(http.MethodGet, url, nil)
	if err != nil {
		return nil, err
	}

	topicList := &TopicList{}
	err = json.Unmarshal(responseBody, topicList)
	if err != nil {
		return nil, err
	}
	return topicList, nil
}

// PublishOne TODO
func (c *Client) PublishOne(topic string, data string) (*MessageIDList, error) {
	return c.Publish(topic, []string{data})
}

// Publish TODO
func (c *Client) Publish(topic string, data []string) (*MessageIDList, error) {
	url := c.baseURL.String() + path.Join(topicsPath, topic, "publish")

	rawMessages := []*RawMessage{}
	for _, d := range data {
		rawMessages = append(rawMessages, NewRawMessage(d))
	}

	rawMessageList := NewRawMessageList(rawMessages)
	body, err := json.Marshal(rawMessageList)
	if err != nil {
		return nil, err
	}

	responseBody, err := c.send(http.MethodPost, url, bytes.NewBuffer(body))
	if err != nil {
		return nil, err
	}

	messageIDList := &MessageIDList{}
	err = json.Unmarshal(responseBody, messageIDList)
	if err != nil {
		return nil, err
	}
	return messageIDList, nil
}

// GetTopicSubscriptions TODO
func (c *Client) GetTopicSubscriptions(topic string) (*SubscriptionNameList, error) {
	url := c.baseURL.String() + path.Join(topicsPath, topic, "subscriptions")

	responseBody, err := c.send(http.MethodGet, url, nil)
	if err != nil {
		return nil, err
	}

	subscriptionNameList := &SubscriptionNameList{}
	err = json.Unmarshal(responseBody, subscriptionNameList)
	if err != nil {
		return nil, err
	}
	return subscriptionNameList, nil
}

// CreateSubscription TODO
func (c *Client) CreateSubscription(name string, config SubscriptionCreateConfig) (*Subscription, error) {
	url := c.baseURL.String() + path.Join(subscriptionsPath, name)
	if name == "" {
		url += "/"
	}

	body, err := json.Marshal(config)
	if err != nil {
		return nil, err
	}

	responseBody, err := c.send(http.MethodPut, url, bytes.NewBuffer(body))
	if err != nil {
		return nil, err
	}

	Subscription := &Subscription{}
	err = json.Unmarshal(responseBody, Subscription)
	if err != nil {
		return nil, err
	}
	return Subscription, nil
}

// CreateSubscriptionWithUUID TODO
func (c *Client) CreateSubscriptionWithUUID(config SubscriptionCreateConfig) (*Subscription, error) {
	return c.CreateSubscription("", config)
}

// UpdateSubscription TODO
func (c *Client) UpdateSubscription(name string, config SubscriptionUpdateConfig) (*Subscription, error) {
	url := c.baseURL.String() + path.Join(subscriptionsPath, name)

	body, err := json.Marshal(config)
	if err != nil {
		return nil, err
	}

	responseBody, err := c.send(http.MethodPatch, url, bytes.NewBuffer(body))
	if err != nil {
		return nil, err
	}

	Subscription := &Subscription{}
	err = json.Unmarshal(responseBody, Subscription)
	if err != nil {
		return nil, err
	}
	return Subscription, nil
}

// DeleteSubscription TODO
func (c *Client) DeleteSubscription(name string) error {
	url := c.baseURL.String() + path.Join(subscriptionsPath, name)

	_, err := c.send(http.MethodDelete, url, nil)
	if err != nil {
		return err
	}

	return nil
}

// GetSubscription TODO
func (c *Client) GetSubscription(name string) (*Subscription, error) {
	url := c.baseURL.String() + path.Join(subscriptionsPath, name)

	responseBody, err := c.send(http.MethodGet, url, nil)
	if err != nil {
		return nil, err
	}

	subscription := &Subscription{}
	err = json.Unmarshal(responseBody, subscription)
	if err != nil {
		return nil, err
	}
	return subscription, nil
}

// ListSubscriptions TODO
func (c *Client) ListSubscriptions() (*SubscriptionList, error) {
	url := c.baseURL.String() + subscriptionsPath + "/"

	responseBody, err := c.send(http.MethodGet, url, nil)
	if err != nil {
		return nil, err
	}

	subscriptionList := &SubscriptionList{}
	err = json.Unmarshal(responseBody, subscriptionList)
	if err != nil {
		return nil, err
	}
	return subscriptionList, nil
}

// PullOne TODO
func (c *Client) PullOne(name string) (*MessageList, error) {
	return c.Pull(name, 1)
}

// Pull TODO
func (c *Client) Pull(name string, maxMessages uint) (*MessageList, error) {
	url := c.baseURL.String() + path.Join(subscriptionsPath, name, "pull")

	body, err := json.Marshal(NewPullConfig(maxMessages))
	if err != nil {
		return nil, err
	}

	responseBody, err := c.send(http.MethodPost, url, bytes.NewReader(body))
	if err != nil {
		return nil, err
	}

	messageList := &MessageList{}
	err = json.Unmarshal(responseBody, messageList)
	if err != nil {
		return nil, err
	}
	return messageList, nil
}

// AckOne TODO
func (c *Client) AckOne(name string, messageID string) (*MessageIDList, error) {
	return c.Ack(name, []string{messageID})
}

// Ack TODO
func (c *Client) Ack(name string, messageIds []string) (*MessageIDList, error) {
	url := c.baseURL.String() + path.Join(subscriptionsPath, name, "ack")

	body, err := json.Marshal(NewMessageIDList(messageIds))
	if err != nil {
		return nil, err
	}

	responseBody, err := c.send(http.MethodPost, url, bytes.NewReader(body))
	if err != nil {
		return nil, err
	}

	messageIDList := &MessageIDList{}
	err = json.Unmarshal(responseBody, messageIDList)
	if err != nil {
		return nil, err
	}
	return messageIDList, nil
}

func (c *Client) send(method string, url string, body io.Reader) ([]byte, error) {
	request, err := http.NewRequest(method, url, body)
	request.Header.Set("Content-Type", "application/json")
	if err != nil {
		return nil, err
	}

	response, err := c.http.Do(request)
	if err != nil {
		return nil, err
	}

	statusCode := response.StatusCode
	if 400 <= statusCode && statusCode < 500 {
		return nil, fmt.Errorf("%v client error for url: %v", statusCode, url)
	} else if 500 <= statusCode && statusCode < 600 {
		return nil, fmt.Errorf("%v server error for url: %v", statusCode, url)
	}

	responseBody, err := ioutil.ReadAll(response.Body)
	if err != nil {
		return nil, err
	}
	defer response.Body.Close()
	return responseBody, nil
}
