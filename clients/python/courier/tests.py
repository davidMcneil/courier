import unittest
from client import Client


class TestStringMethods(unittest.TestCase):

    def test(self):
        client = Client("http://127.0.0.1:3140")

        topic_name = "python-topic"
        sub_name = "python-sub"

        # Make sure everything we create does not exist
        try:
            client.delete_topic(topic_name)
            client.delete_subscription("python-sub")
        except:
            pass

        # Test create
        topic = client.create_topic(
            topic_name, {"ttl": 3600, "message_ttl": 60})
        self.assertEqual(topic["name"], topic_name)
        self.assertEqual(topic["ttl"], 3600)
        self.assertEqual(topic["message_ttl"], 60)

        sub = client.create_subscription(
            sub_name, {"topic": topic_name, "ttl": 3600, "ack_deadline": 60})
        self.assertEqual(sub["name"], sub_name)
        self.assertEqual(sub["topic"], topic_name)
        self.assertEqual(sub["ttl"], 3600)
        self.assertEqual(sub["ack_deadline"], 60)

        # Test update
        topic = client.update_topic(
            topic_name, {"ttl": 5000, "message_ttl": 12})
        self.assertEqual(topic["name"], topic_name)
        self.assertEqual(topic["ttl"], 5000)
        self.assertEqual(topic["message_ttl"], 12)

        sub = client.update_subscription(
            sub_name, {"ttl": 12000, "ack_deadline": 72})
        self.assertEqual(sub["name"], sub_name)
        self.assertEqual(sub["topic"], topic_name)
        self.assertEqual(sub["ttl"], 12000)
        self.assertEqual(sub["ack_deadline"], 72)

        # Test get
        topic = client.get_topic(topic_name)
        self.assertEqual(topic["name"], topic_name)
        self.assertEqual(topic["ttl"], 5000)
        self.assertEqual(topic["message_ttl"], 12)

        sub = client.get_subscription(sub_name)
        self.assertEqual(sub["name"], sub_name)
        self.assertEqual(sub["topic"], topic_name)
        self.assertEqual(sub["ttl"], 12000)
        self.assertEqual(sub["ack_deadline"], 72)

        # Test create with uuid and list
        topic = client.create_topic_with_uuid({})
        topics = client.list_topics()["topics"]
        topic_names = map(lambda t: t["name"], topics)
        self.assertSetEqual(
            set([topic_name, topic["name"]]), set(topic_names))

        sub = client.create_subscription_with_uuid({"topic": topic_name})
        subs = client.list_subscriptions()["subscriptions"]
        sub_names = map(lambda t: t["name"], subs)
        self.assertSetEqual(
            set([sub_name, sub["name"]]), set(sub_names))

        sub_names = client.get_topic_subscriptions(
            topic_name)["subscription_names"]
        self.assertSetEqual(
            set([sub_name, sub["name"]]), set(sub_names))

        # Test delete and list
        client.delete_subscription(sub["name"])
        subs = client.list_subscriptions()["subscriptions"]
        sub_names = map(lambda t: t["name"], subs)
        self.assertSetEqual(set([sub_name]), set(sub_names))

        client.delete_topic(topic["name"])
        topics = client.list_topics()["topics"]
        topic_names = map(lambda t: t["name"], topics)
        self.assertSetEqual(set([topic_name]), set(topic_names))

        # Test publish, pull, and ack
        client.publish(topic_name, "data1")
        client.publish(topic_name, ["data2", "data3"])

        message1 = client.pull(sub_name)["messages"][0]
        messages = client.pull(sub_name, 2)["messages"]
        message2 = messages[0]
        message3 = messages[1]
        self.assertEqual(message1["data"], "data1")
        self.assertEqual(message2["data"], "data2")
        self.assertEqual(message3["data"], "data3")

        message_ids = client.ack(sub_name, [message1["id"], message2[
                                 "id"], message3["id"]])["message_ids"]
        self.assertSetEqual(set([message1["id"], message2[
                                 "id"], message3["id"]]), set(message_ids))


if __name__ == '__main__':
    unittest.main()
