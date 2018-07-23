package courier

import (
	"fmt"
	"sort"
	"testing"
)

func assertEqual(t *testing.T, a interface{}, b interface{}) {
	if a == b {
		return
	}
	message := fmt.Sprintf("%v != %v", a, b)
	t.Fatal(message)
}

func assertSliceEqual(t *testing.T, a, b []string) {

	if a == nil && b == nil {
		return
	}

	if a == nil || b == nil {
		t.Fatalf("%v != %v", a, b)
	}

	if len(a) != len(b) {
		t.Fatalf("len(%v) != len(%v)", len(a), len(b))
	}

	for i := range a {
		if a[i] != b[i] {
			t.Fatalf("%v != %v", a[i], b[i])
		}
	}
}

func TestTimeConsuming(t *testing.T) {
	client, _ := NewClient("http://127.0.0.1:3140")

	topicName := "go-topic"
	subName := "go-sub"

	// Make sure everything we create does not exist
	client.DeleteSubscription(subName)
	client.DeleteTopic(topicName)

	// Test create
	ttl := uint32(3600)
	messageTTL := uint32(60)
	topic, err := client.CreateTopic(
		topicName, TopicCreateConfig{TTL: &ttl, MessageTTL: &messageTTL})
	if err != nil {
		t.Fatalf("err: %+v", err)
	}
	assertEqual(t, topic.Name, topicName)
	assertEqual(t, topic.TTL, int64(3600))
	assertEqual(t, topic.MessageTTL, int64(60))

	ttl = uint32(3600)
	ackDeadline := uint32(60)
	sub, err := client.CreateSubscription(
		subName, SubscriptionCreateConfig{Topic: topicName, TTL: &ttl, AckDeadline: &ackDeadline})
	if err != nil {
		t.Fatalf("err: %+v", err)
	}
	assertEqual(t, sub.Name, subName)
	assertEqual(t, sub.Topic, topicName)
	assertEqual(t, sub.TTL, int64(3600))
	assertEqual(t, sub.AckDeadline, int64(60))

	// Test update
	ttl = uint32(5000)
	messageTTL = uint32(12)
	topic, _ = client.UpdateTopic(
		topicName, TopicUpdateConfig{TTL: &ttl, MessageTTL: &messageTTL})
	assertEqual(t, topic.Name, topicName)
	assertEqual(t, topic.TTL, int64(5000))
	assertEqual(t, topic.MessageTTL, int64(12))

	ttl = uint32(12000)
	ackDeadline = uint32(72)
	sub, _ = client.UpdateSubscription(
		subName, SubscriptionUpdateConfig{TTL: &ttl, AckDeadline: &ackDeadline})
	assertEqual(t, sub.Name, subName)
	assertEqual(t, sub.Topic, topicName)
	assertEqual(t, sub.TTL, int64(12000))
	assertEqual(t, sub.AckDeadline, int64(72))

	// Test get
	topic, _ = client.GetTopic(topicName)
	assertEqual(t, topic.Name, topicName)
	assertEqual(t, topic.TTL, int64(5000))
	assertEqual(t, topic.MessageTTL, int64(12))

	sub, _ = client.GetSubscription(subName)
	assertEqual(t, sub.Name, subName)
	assertEqual(t, sub.Topic, topicName)
	assertEqual(t, sub.TTL, int64(12000))
	assertEqual(t, sub.AckDeadline, int64(72))

	// Test create with uuid and list
	topic, _ = client.CreateTopicWithUUID(TopicCreateConfig{TTL: nil, MessageTTL: nil})
	topicsTemp, err := client.ListTopics()
	topics := topicsTemp.Topics
	topicNames := []string{}
	for _, t := range topics {
		topicNames = append(topicNames, t.Name)
	}
	topicNamesTruth := []string{topicName, topic.Name}
	sort.Strings(topicNamesTruth)
	sort.Strings(topicNames)
	assertSliceEqual(t, topicNamesTruth, topicNames)

	sub, _ = client.CreateSubscriptionWithUUID(SubscriptionCreateConfig{Topic: topicName, TTL: nil, AckDeadline: nil})
	subsTemp, _ := client.ListSubscriptions()
	subs := subsTemp.Subscriptions
	subNames := []string{}
	for _, t := range subs {
		subNames = append(subNames, t.Name)
	}
	subNamesTruth := []string{subName, sub.Name}
	sort.Strings(subNamesTruth)
	sort.Strings(subNames)
	assertSliceEqual(t,
		subNamesTruth, subNames)

	subNamesTemp, _ := client.GetTopicSubscriptions(topicName)
	subNames = subNamesTemp.SubscriptionNames
	sort.Strings(subNames)
	assertSliceEqual(t, subNamesTruth, subNames)

	// Test delete and list
	client.DeleteSubscription(sub.Name)
	subsTemp, _ = client.ListSubscriptions()
	subs = subsTemp.Subscriptions
	subNames = []string{}
	for _, t := range subs {
		subNames = append(subNames, t.Name)
	}
	subNamesTruth = []string{subName}
	sort.Strings(subNamesTruth)
	sort.Strings(subNames)
	assertSliceEqual(t, subNamesTruth, subNames)

	client.DeleteTopic(topic.Name)
	topicsTemp, _ = client.ListTopics()
	topics = topicsTemp.Topics
	topicNames = []string{}
	for _, t := range topics {
		topicNames = append(topicNames, t.Name)
	}
	topicNamesTruth = []string{topicName}
	sort.Strings(topicNamesTruth)
	sort.Strings(topicNames)
	assertSliceEqual(t, topicNamesTruth, topicNames)

	// Test publish, pull, and ack
	client.PublishOne(topicName, "data1")
	client.Publish(topicName, []string{"data2", "data3"})

	message1Temp, _ := client.PullOne(subName)
	message1 := message1Temp.Messages[0]
	messagesTemp, _ := client.Pull(subName, 2)
	messages := messagesTemp.Messages
	message2 := messages[0]
	message3 := messages[1]
	assertEqual(t, message1.Data, "data1")
	assertEqual(t, message2.Data, "data2")
	assertEqual(t, message3.Data, "data3")

	messageIdsTemp, _ := client.Ack(subName, []string{message1.ID, message2.ID, message3.ID})
	messageIds := messageIdsTemp.MessageIds
	messageIdsTruth := []string{message1.ID, message2.ID, message3.ID}
	sort.Strings(messageIdsTruth)
	sort.Strings(messageIds)
	assertSliceEqual(t, messageIdsTruth, messageIds)

	// Delete what we created
	client.DeleteSubscription(subName)
	client.DeleteTopic(topicName)
}
