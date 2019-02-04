use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::error::Error;
use url;
use url::Url;
use uuid::Uuid;

#[cfg(test)]
mod tests;
mod types;

pub use crate::types::{
    MessageIdList, MessageList, PullConfig, RawMessage, RawMessageList, Subscription,
    SubscriptionCreateConfig, SubscriptionList, SubscriptionNameList, SubscriptionUpdateConfig,
    Topic, TopicCreateConfig, TopicList, TopicUpdateConfig,
};

static HEARTBEAT_PATH: &'static str = "/api/v1/heartbeat";
static TOPICS_PATH: &'static str = "/api/v1/topics";
static SUBSCRIPTIONS_PATH: &'static str = "/api/v1/subscriptions";

pub struct Client {
    base_url: url::Url,
    http: reqwest::Client,
}

impl Client {
    pub fn new(base_url: &str) -> Result<Self, Box<dyn Error>> {
        let base_url = Url::parse(base_url)?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let http = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;
        Ok(Client { base_url, http })
    }

    pub fn heartbeat(&self) -> bool {
        let url = match self.base_url.join(HEARTBEAT_PATH) {
            Ok(url) => url,
            Err(_) => return false,
        };
        match self.http.put(url).send() {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn create_topic(
        &self,
        name: &str,
        config: &TopicCreateConfig,
    ) -> Result<Topic, Box<dyn Error>> {
        let url = self.base_url.join(&format!("{}/{}", TOPICS_PATH, name))?;
        Ok(self
            .http
            .put(url)
            .json(config)
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn create_topic_with_uuid(
        &self,
        config: &TopicCreateConfig,
    ) -> Result<Topic, Box<dyn Error>> {
        self.create_topic("", config)
    }

    pub fn update_topic(
        &self,
        name: &str,
        config: &TopicUpdateConfig,
    ) -> Result<Topic, Box<dyn Error>> {
        let url = self.base_url.join(&format!("{}/{}", TOPICS_PATH, name))?;
        Ok(self
            .http
            .patch(url)
            .json(&config)
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn delete_topic(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let url = self.base_url.join(&format!("{}/{}", TOPICS_PATH, name))?;
        self.http.delete(url).send()?.error_for_status()?;
        Ok(())
    }

    pub fn get_topic(&self, name: &str) -> Result<Topic, Box<dyn Error>> {
        let url = self.base_url.join(&format!("{}/{}", TOPICS_PATH, name))?;
        Ok(self.http.get(url).send()?.error_for_status()?.json()?)
    }

    pub fn list_topics(&self) -> Result<TopicList, Box<dyn Error>> {
        let url = self.base_url.join(&format!("{}/", TOPICS_PATH))?;
        Ok(self.http.get(url).send()?.error_for_status()?.json()?)
    }

    pub fn publish_one(&self, topic: &str, data: String) -> Result<MessageIdList, Box<dyn Error>> {
        self.publish(topic, vec![data])
    }

    pub fn publish(&self, topic: &str, data: Vec<String>) -> Result<MessageIdList, Box<dyn Error>> {
        let url = self
            .base_url
            .join(&format!("{}/{}/publish", TOPICS_PATH, topic))?;
        let mut raw_messages = Vec::with_capacity(data.len());
        for d in data {
            raw_messages.push(RawMessage::new(d));
        }
        Ok(self
            .http
            .post(url)
            .json(&RawMessageList::new(raw_messages))
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn get_topic_subscriptions(
        &self,
        topic: &str,
    ) -> Result<SubscriptionNameList, Box<dyn Error>> {
        let url = self
            .base_url
            .join(&format!("{}/{}/subscriptions", TOPICS_PATH, topic))?;
        Ok(self.http.get(url).send()?.error_for_status()?.json()?)
    }

    pub fn create_subscription(
        &self,
        name: &str,
        config: &SubscriptionCreateConfig,
    ) -> Result<Subscription, Box<dyn Error>> {
        let url = self
            .base_url
            .join(&format!("{}/{}", SUBSCRIPTIONS_PATH, name))?;
        Ok(self
            .http
            .put(url)
            .json(config)
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn create_subscription_with_uuid(
        &self,
        config: &SubscriptionCreateConfig,
    ) -> Result<Subscription, Box<dyn Error>> {
        self.create_subscription("", config)
    }

    pub fn update_subscription(
        &self,
        name: &str,
        config: &SubscriptionUpdateConfig,
    ) -> Result<Subscription, Box<dyn Error>> {
        let url = self
            .base_url
            .join(&format!("{}/{}", SUBSCRIPTIONS_PATH, name))?;
        Ok(self
            .http
            .patch(url)
            .json(&config)
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn delete_subscription(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let url = self
            .base_url
            .join(&format!("{}/{}", SUBSCRIPTIONS_PATH, name))?;
        self.http.delete(url).send()?.error_for_status()?;
        Ok(())
    }

    pub fn get_subscription(&self, name: &str) -> Result<Subscription, Box<dyn Error>> {
        let url = self
            .base_url
            .join(&format!("{}/{}", SUBSCRIPTIONS_PATH, name))?;
        Ok(self.http.get(url).send()?.error_for_status()?.json()?)
    }

    pub fn list_subscriptions(&self) -> Result<SubscriptionList, Box<dyn Error>> {
        let url = self.base_url.join(&format!("{}/", SUBSCRIPTIONS_PATH))?;
        Ok(self.http.get(url).send()?.error_for_status()?.json()?)
    }

    pub fn pull_one(&self, subscription: &str) -> Result<MessageList, Box<dyn Error>> {
        self.pull(subscription, 1)
    }

    pub fn pull(
        &self,
        subscription: &str,
        max_messages: usize,
    ) -> Result<MessageList, Box<dyn Error>> {
        let url = self
            .base_url
            .join(&format!("{}/{}/pull", SUBSCRIPTIONS_PATH, subscription))?;
        Ok(self
            .http
            .post(url)
            .json(&PullConfig::new(max_messages))
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn ack_one(
        &self,
        subscription: &str,
        message_id: Uuid,
    ) -> Result<MessageIdList, Box<dyn Error>> {
        self.ack(subscription, vec![message_id])
    }

    pub fn ack(
        &self,
        subscription: &str,
        message_ids: Vec<Uuid>,
    ) -> Result<MessageIdList, Box<dyn Error>> {
        let url = self
            .base_url
            .join(&format!("{}/{}/ack", SUBSCRIPTIONS_PATH, subscription))?;
        Ok(self
            .http
            .post(url)
            .json(&MessageIdList::new(message_ids))
            .send()?
            .error_for_status()?
            .json()?)
    }
}
