extern crate chrono;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;
extern crate uuid;

mod types;

use reqwest::header::{ContentType, Headers};
use std::error::Error;
use url::Url;
use uuid::Uuid;

pub use types::{
    MessageIdList, MessageList, PullConfig, RawMessage, RawMessageList, Subscription,
    SubscriptionCreateConfig, SubscriptionList, SubscriptionNameList, SubscriptionUpdateConfig,
    Topic, TopicCreateConfig, TopicList, TopicUpdateConfig,
};

static TOPICS_PATH: &'static str = "/api/v0/topics";
static SUBSCRIPTIONS_PATH: &'static str = "/api/v0/subscriptions";

pub struct Client {
    base_url: url::Url,
    http: reqwest::Client,
}

impl Client {
    pub fn new(base_url: &str) -> Result<Self, Box<dyn Error>> {
        let base_url = Url::parse(base_url)?;
        let mut headers = Headers::new();
        headers.set(ContentType::json());
        let http = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;
        Ok(Client { base_url, http })
    }

    pub fn create_topic(
        &self,
        name: &str,
        config: &TopicCreateConfig,
    ) -> Result<Topic, Box<dyn Error>> {
        let url = self.base_url.join(&format!("{}/{}", TOPICS_PATH, name))?;
        Ok(self.http
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
        Ok(self.http
            .patch(url)
            .json(&config)
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn delete_topic(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let url = self.base_url.join(&format!("{}/{}", TOPICS_PATH, name))?;
        Ok(self.http.delete(url).send()?.error_for_status()?.json()?)
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
        let url = self.base_url
            .join(&format!("{}/{}/publish", TOPICS_PATH, topic))?;
        let mut raw_messages = Vec::with_capacity(data.len());
        for datum in data.into_iter() {
            raw_messages.push(RawMessage::new(datum));
        }
        Ok(self.http
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
        let url = self.base_url
            .join(&format!("{}/{}/subscriptions", TOPICS_PATH, topic))?;
        Ok(self.http.get(url).send()?.error_for_status()?.json()?)
    }

    pub fn create_subscription(
        &self,
        name: &str,
        config: &SubscriptionCreateConfig,
    ) -> Result<Subscription, Box<dyn Error>> {
        let url = self.base_url
            .join(&format!("{}/{}", SUBSCRIPTIONS_PATH, name))?;
        Ok(self.http
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
    ) -> Result<Topic, Box<dyn Error>> {
        let url = self.base_url
            .join(&format!("{}/{}", SUBSCRIPTIONS_PATH, name))?;
        Ok(self.http
            .patch(url)
            .json(&config)
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn delete_subscription(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let url = self.base_url
            .join(&format!("{}/{}", SUBSCRIPTIONS_PATH, name))?;
        Ok(self.http.delete(url).send()?.error_for_status()?.json()?)
    }

    pub fn get_subscription(&self, name: &str) -> Result<Subscription, Box<dyn Error>> {
        let url = self.base_url
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

    pub fn pull(&self, name: &str, max_messages: usize) -> Result<MessageList, Box<dyn Error>> {
        let url = self.base_url
            .join(&format!("{}/{}/pull", SUBSCRIPTIONS_PATH, name))?;
        Ok(self.http
            .post(url)
            .json(&PullConfig::new(max_messages))
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn ack_one(&self, name: &str, message_id: Uuid) -> Result<MessageIdList, Box<dyn Error>> {
        self.ack(name, vec![message_id])
    }

    pub fn ack(&self, name: &str, message_ids: Vec<Uuid>) -> Result<MessageIdList, Box<dyn Error>> {
        let url = self.base_url
            .join(&format!("{}/{}/ack", SUBSCRIPTIONS_PATH, name))?;
        Ok(self.http
            .post(url)
            .json(&MessageIdList::new(message_ids))
            .send()?
            .error_for_status()?
            .json()?)
    }
}
