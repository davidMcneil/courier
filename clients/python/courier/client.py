import requests
from urlparse import urljoin

topics_path = "/api/v0/topics"
subscriptions_path = "/api/v0/subscriptions"


class Client:

    def __init__(self, base_url):
        self.base_url = base_url

    def create_topic(self, name, config):
        url = urljoin(
            self.base_url, "{0}/{1}".format(topics_path, name))
        resp = requests.put(url, json=config)
        resp.raise_for_status()
        return resp.json()

    def create_topic_with_uuid(self, config):
        return self.create_topic("", config)

    def update_topic(self, name, config):
        url = urljoin(
            self.base_url, "{0}/{1}".format(topics_path, name))
        resp = requests.patch(url, json=config)
        resp.raise_for_status()
        return resp.json()

    def delete_topic(self, name):
        url = urljoin(
            self.base_url, "{0}/{1}".format(topics_path, name))
        resp = requests.delete(url)
        resp.raise_for_status()

    def get_topic(self, name):
        url = urljoin(
            self.base_url, "{0}/{1}".format(topics_path, name))
        resp = requests.get(url)
        resp.raise_for_status()
        return resp.json()

    def list_topics(self):
        url = urljoin(
            self.base_url, "{0}/".format(topics_path))
        resp = requests.get(url)
        resp.raise_for_status()
        return resp.json()

    def publish(self, topic, data):
        raw_messages = []
        if isinstance(data, basestring):
            raw_messages = [{"data": data}]
        else:
            raw_messages = map(lambda d: {"data": d}, data)
        url = urljoin(
            self.base_url, "{0}/{1}/publish".format(topics_path, topic))
        resp = requests.post(url, json={"raw_messages": raw_messages})
        resp.raise_for_status()
        return resp.json()

    def get_topic_subscriptions(self, topic):
        url = urljoin(
            self.base_url, "{0}/{1}/subscriptions".format(topics_path, topic))
        resp = requests.get(url)
        resp.raise_for_status()
        return resp.json()

    def create_subscription(self, name, config):
        url = urljoin(
            self.base_url, "{0}/{1}".format(subscriptions_path, name))
        resp = requests.put(url, json=config)
        resp.raise_for_status()
        return resp.json()

    def create_subscription_with_uuid(self, config):
        return self.create_subscription("", config)

    def update_subscription(self, name, config):
        url = urljoin(
            self.base_url, "{0}/{1}".format(subscriptions_path, name))
        resp = requests.patch(url, json=config)
        resp.raise_for_status()
        return resp.json()

    def delete_subscription(self, name):
        url = urljoin(
            self.base_url, "{0}/{1}".format(subscriptions_path, name))
        resp = requests.delete(url)
        resp.raise_for_status()

    def get_subscription(self, name):
        url = urljoin(
            self.base_url, "{0}/{1}".format(subscriptions_path, name))
        resp = requests.get(url)
        resp.raise_for_status()
        return resp.json()

    def list_subscriptions(self):
        url = urljoin(
            self.base_url, "{0}/".format(subscriptions_path))
        resp = requests.get(url)
        resp.raise_for_status()
        return resp.json()

    def pull(self, subscription, max_messages=1):
        url = urljoin(
            self.base_url, "{0}/{1}/pull".format(subscriptions_path, subscription))
        resp = requests.post(url, json={"max_messages": max_messages})
        resp.raise_for_status()
        return resp.json()

    def ack(self, subscription, uuids):
        message_ids = []
        if isinstance(uuids, basestring):
            message_ids = [uuids]
        else:
            message_ids = uuids
        url = urljoin(
            self.base_url, "{0}/{1}/ack".format(subscriptions_path, subscription))
        resp = requests.post(url, json={"message_ids": message_ids})
        resp.raise_for_status()
        return resp.json()
