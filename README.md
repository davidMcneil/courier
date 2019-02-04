# Courier

[![Build Status](https://travis-ci.org/davidMcneil/courier.svg?branch=master)](https://travis-ci.org/davidMcneil/courier)
[![Coverage Status](https://coveralls.io/repos/github/davidMcneil/courier/badge.svg?branch=master)](https://coveralls.io/github/davidMcneil/courier?branch=master)
[![LoC](https://tokei.rs/b1/github/davidMcneil/courier)](https://github.com/davidMcneil/courier)
[![License](http://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![License](http://img.shields.io/badge/license-APACHE-blue.svg)](./LICENSE-APACHE)

Courier provides an in-memory pub/sub service with an HTTP, JSON interface. There are three primary objects that apps using Courier interact with **messages**, **topics**, and **subscriptions**. The basic flow is that apps **pub**lish messages to a given topic while **sub**scribers read messages from the topic to which they are subscribed.

_I am a full-stack software engineer whose language of choice is Rust. I am interested in pursuing new opportunities. See my [resume](https://github.com/davidMcneil/resume/raw/master/resume.pdf) and please contact me with potential openings._

## Install

Grab the [latest release](https://github.com/davidMcneil/courier/releases/latest). The `x86_64-unknown-linux-musl` is 100% statically linked and _should_ run on any x86, unix-like system. Currently, Courier is not built for other architectures.

## Setup

Run `courier -h` to see commands and options. If you execute `courier run`, the services will be bound to host `0.0.0.0` on port `3140`. Run `courier ui` to open up your default web browser to the management page or navigate your browser to [http://0.0.0.0:3140/ui](http://0.0.0.0:3140/ui)

You can interact with Courier through the web interface or programmatically through the HTTP, JSON API. For examples see the C++, Go, Python, and Rust [clients]().

## HTTP JSON API <a name="http_json_api"></a>

**Table of Contents**

- [Topic End Points](#topic_end_points)
  - [Create](#topic_create)
  - [Update](#topic_update)
  - [Delete](#topic_delete)
  - [Get](#topic_get)
  - [List](#topic_list)
  - [Subscriptions](#topic_subscriptions)
  - [Publish](#topic_publish)
- [Subscription End Points](#subscription_end_points)
  - [Create](#subscription_create)
  - [Update](#subscription_update)
  - [Delete](#subscription_delete)
  - [Get](#subscription_get)
  - [List](#subscription_list)
  - [Pull](#subscription_pull)
  - [Ack](#subscription_ack)

All messages require the following HTTP headers to be set:

| Headers      | Value            |
| ------------ | ---------------- |
| Content-Type | application/json |

## Types

### Topic <a name="topic_type"></a>

```js
{
  "name": "string", // The name of the topic
  "message_ttl": "i64", // The time to live (ttl) applied to all messages, use 0 for no ttl (seconds)
  "ttl": "i64", // The time to live (ttl) of the topic, use 0 for no ttl (seconds)
  "created": "string", // When the topic was created as an ISO 8601 datetime string (UTC)
  "updated": "string" // // When the topic was last updated as an ISO 8601 datetime string (UTC)
}
```

### TopicList <a name="topic_list_type"></a>

```js
{
  "topics": "Topic[]"
}
```

### Subscription <a name="subscription_type"></a>

```js
{
  "name": "string", // The name of the subscriptions
  "topic": "string", // The name of the topic to subscribe to
  "ack_deadline": "i64", // The amount of time given to ack a message before it is resent (seconds)
  "ttl": "i64", // The time to live (ttl) of the subscription, use 0 for no ttl (seconds)
  "created": "string", // When the subscription was created as an ISO 8601 datetime string (UTC)
  "updated": "string" // // When the subscription was last updated as an ISO 8601 datetime string (UTC)
}
```

### SubscriptionList <a name="topic_list_type"></a>

```js
{
  "subscriptions": "Subscription[]"
}
```

### SubscriptionNameList <a name="subscription_name_list_type"></a>

```js
{
  "subscription_names": "string[]"
}
```

### RawMessage <a name="raw_message_type"></a>

```js
{
  "data": "string" // The messages contents as a string blob
}
```

### Message <a name="message_type"></a>

```js
{
  "id": "string", // The unique id of the message
  "time": "string", // When the messages was published as an ISO 8601 datetime string (UTC)
  "tries": "u32", // The number of times the message has been pulled
  "data": "string" // The messages contents as a string blob
}
```

### MessageList <a name="message_list_type"></a>

```js
{
  "messages": "Message[]"
}
```

### MessageIdList <a name="message_id_list_type"></a>

```js
{
  "message_ids": "string[]"
}
```

### Topic End Points <a name="topic_end_points"></a>

#### Create - (PUT) /api/v1/topics/&lt;topic&gt; <a name="topic_create"></a>

Create a new topic.

##### Request

```js
{
  "message_ttl": "u32",
  "ttl": "u32"
}
```

| Parameter   | Description                                                            | Units   | Format | Required |
| ----------- | ---------------------------------------------------------------------- | ------- | ------ | -------- |
| topic       | The unique name of the topic, a random name will be generated if empty | n/a     | path   | false    |
| message_ttl | The time to live (ttl) applied to all messages, use 0 for no ttl       | seconds | body   | false    |
| ttl         | The time to live (ttl) of the topic, use 0 for no ttl                  | seconds | body   | false    |

##### Response

| Status Code    | Response Body        | Description                                                                     |
| -------------- | -------------------- | ------------------------------------------------------------------------------- |
| 201 (Created)  | [Topic](#topic_type) | Successfully created a new topic                                                |
| 409 (Conflict) | &lt;empty&gt;        | Could not create a topic because a topic with the specified name already exists |

#### Update - (PATCH) /api/v1/topics/&lt;topic&gt; <a name="topic_update"></a>

Update a topic. Updates the topic's `updated` field regardless of if a value is actually updated.

##### Request

```js
{
  "message_ttl": "u32",
  "ttl": "u32"
}
```

| Parameter   | Description                                                      | Units   | Format | Required |
| ----------- | ---------------------------------------------------------------- | ------- | ------ | -------- |
| topic       | The name of the topic                                            | n/a     | path   | true     |
| message_ttl | The time to live (ttl) applied to all messages, use 0 for no ttl | seconds | body   | false    |
| ttl         | The time to live (ttl) of the topic, use 0 for no ttl            | seconds | body   | false    |

##### Response

| Status Code     | Response Body        | Description                                        |
| --------------- | -------------------- | -------------------------------------------------- |
| 200 (Ok)        | [Topic](#topic_type) | Successfully updated the topic                     |
| 404 (Not Found) | &lt;empty&gt;        | A topic with the specified name could not be found |

#### Delete - (DELETE) /api/v1/topics/&lt;topic&gt; <a name="topic_delete"></a>

Delete a topic. This will also delete all the subscriptions subscribed to this topic.

##### Request

| Parameter | Description           | Units | Format | Required |
| --------- | --------------------- | ----- | ------ | -------- |
| topic     | The name of the topic | n/a   | path   | true     |

##### Response

| Status Code     | Response Body | Description                                        |
| --------------- | ------------- | -------------------------------------------------- |
| 200 (Ok)        | &lt;empty&gt; | Successfully deleted the topic                     |
| 404 (Not Found) | &lt;empty&gt; | A topic with the specified name could not be found |

#### Get - (GET) /api/v1/topics/&lt;topic&gt; <a name="topic_get"></a>

Get a topic.

##### Request

| Parameter | Description           | Units | Format | Required |
| --------- | --------------------- | ----- | ------ | -------- |
| topic     | The name of the topic | n/a   | path   | true     |

##### Response

| Status Code     | Response Body        | Description                                        |
| --------------- | -------------------- | -------------------------------------------------- |
| 200 (Ok)        | [Topic](#topic_type) | Successfully retrieved the topic                   |
| 404 (Not Found) | &lt;empty&gt;        | A topic with the specified name could not be found |

#### List - (GET) /api/v1/topics <a name="topic_list"></a>

List all of the topics.

##### Request

| Parameter | Description | Units | Format | Required |
| --------- | ----------- | ----- | ------ | -------- |
|           |             |       |        |          |

##### Response

| Status Code | Response Body                 | Description                           |
| ----------- | ----------------------------- | ------------------------------------- |
| 200 (Ok)    | [TopicList](#topic_list_type) | Successfully retrieved the topic list |

#### Subscriptions - (GET) /api/v1/topics/&lt;topic&gt;/subscriptions <a name="topic_subscriptions"></a>

List all of the subscription names which are subscribed to this topic.

##### Request

| Parameter | Description           | Units | Format | Required |
| --------- | --------------------- | ----- | ------ | -------- |
| topic     | The name of the topic | n/a   | path   | true     |

##### Response

| Status Code     | Response Body                                        | Description                                        |
| --------------- | ---------------------------------------------------- | -------------------------------------------------- |
| 200 (Ok)        | [SubscriptionNameList](#subscription_name_list_type) | Successfully retrieved the subscription name list  |
| 404 (Not Found) | &lt;empty&gt;                                        | A topic with the specified name could not be found |

#### Publish - (POST) /api/v1/topics/&lt;topic&gt;/publish <a name="topic_publish"></a>

Add messages to a topic. Updates the topics `updated` fields

```js
{
  "raw_messages": "RawMessage[]"
}
```

##### Request

| Parameter | Description              | Units | Format | Required |
| --------- | ------------------------ | ----- | ------ | -------- |
| topic     | The name of the topic    | n/a   | path   | true     |
| messages  | The list of raw messages | n/a   | body   | true     |

##### Response

| Status Code     | Response Body                          | Description                                        |
| --------------- | -------------------------------------- | -------------------------------------------------- |
| 200 (Ok)        | [MessageIdList](#message_id_list_type) | Successfully published the messages                |
| 404 (Not Found) | &lt;empty&gt;                          | A topic with the specified name could not be found |

### Subscription End Points <a name="subscription_end_points"></a>

#### Create - (PUT) /api/v1/subscriptions/&lt;subscription&gt; <a name="subscription_create"></a>

Create a new subscription.

##### Request

```js
{
  "topic": "string",
  "ack_deadline": "u32",
  "ttl": "u32",
  "historical": "bool"
}
```

| Parameter    | Description                                                                                                                                                                      | Units   | Format | Required |
| ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------- | ------ | -------- |
| subscription | The unique name of the subscription, a random name will be generated if empty                                                                                                    |         | path   | false    |
| topic        | The name of the topic to subscribe                                                                                                                                               |         | body   | true     |
| ack_deadline | The amount of time given to ack a message before it is resent                                                                                                                    | seconds | body   | false    |
| ttl          | The time to live (ttl) of the subscription, use 0 for no ttl                                                                                                                     | seconds | body   | false    |
| historical   | Should this subscription start pulling from the first message that is part of the subscribed topic, otherwise it will only pull messages added after the subscription is created |         | body   | false    |

##### Response

| Status Code    | Response Body                      | Description                                                                                   |
| -------------- | ---------------------------------- | --------------------------------------------------------------------------------------------- |
| 201 (Created)  | [Subscription](#subscription_type) | Successfully created a new subscription                                                       |
| 409 (Conflict) | &lt;empty&gt;                      | Could not create a subscription because a subscription with the specified name already exists |

#### Update - (PATCH) /api/v1/subscriptions/&lt;subscription&gt; <a name="subscription_update"></a>

Update a subscription. Update the subscriptions `updated` field regardless of if a value is actually updated.

##### Request

```js
{
  "ack_deadline": "u32",
  "ttl": "u32"
}
```

| Parameter    | Description                                                   | Units   | Format | Required |
| ------------ | ------------------------------------------------------------- | ------- | ------ | -------- |
| subscription | The name of the subscription                                  | n/a     | path   | true     |
| ack_deadline | The amount of time given to ack a message before it is resent | seconds | body   | false    |
| ttl          | The time to live (ttl) of the subscription, use 0 for no ttl  | seconds | body   | false    |

##### Response

| Status Code     | Response Body                      | Description                                               |
| --------------- | ---------------------------------- | --------------------------------------------------------- |
| 200 (Ok)        | [Subscription](#subscription_type) | Successfully updated the subscription                     |
| 404 (Not Found) | &lt;empty&gt;                      | A subscription with the specified name could not be found |

#### Delete - (DELETE) /api/v1/subscriptions/&lt;subscription&gt; <a name="subscription_delete"></a>

Delete a subscription.

##### Request

| Parameter    | Description                  | Units | Format | Required |
| ------------ | ---------------------------- | ----- | ------ | -------- |
| subscription | The name of the subscription | n/a   | path   | true     |

##### Response

| Status Code     | Response Body | Description                                               |
| --------------- | ------------- | --------------------------------------------------------- |
| 200 (Ok)        | &lt;empty&gt; | Successfully deleted the subscription                     |
| 404 (Not Found) | &lt;empty&gt; | A subscription with the specified name could not be found |

#### Get - (GET) /api/v1/subscriptions/&lt;subscription&gt; <a name="subscription_get"></a>

Get a subscription.

##### Request

| Parameter    | Description                  | Units | Format | Required |
| ------------ | ---------------------------- | ----- | ------ | -------- |
| subscription | The name of the subscription | n/a   | path   | true     |

##### Response

| Status Code     | Response Body                      | Description                                               |
| --------------- | ---------------------------------- | --------------------------------------------------------- |
| 200 (Ok)        | [Subscription](#subscription_type) | Successfully retrieved the subscription                   |
| 404 (Not Found) | &lt;empty&gt;                      | A subscription with the specified name could not be found |

#### List - (GET) /api/v1/subscriptions <a name="subscription_list"></a>

List all of the subscriptions.

##### Request

| Parameter | Description | Units | Format | Required |
| --------- | ----------- | ----- | ------ | -------- |
|           |             |       |        |          |

##### Response

| Status Code | Response Body                               | Description                                  |
| ----------- | ------------------------------------------- | -------------------------------------------- |
| 200 (Ok)    | [SubscriptionList](#subscription_list_type) | Successfully retrieved the subscription list |

#### Pull - (POST) /api/v1/subscriptions/&lt;subscription&gt;/pull <a name="subscription_pull"></a>

Pull messages from a subscription. Updates the subscriptions `updated` field.

```js
{
  "max_messages": "u32"
}
```

##### Request

| Parameter    | Description                            | Units | Format | Required |
| ------------ | -------------------------------------- | ----- | ------ | -------- |
| subscription | The name of the subscription           | n/a   | path   | true     |
| max_messages | The max number of messages to retrieve | n/a   | body   | false    |

##### Response

| Status Code     | Response Body                     | Description                                               |
| --------------- | --------------------------------- | --------------------------------------------------------- |
| 200 (Ok)        | [MessageList](#message_list_type) | Successfully retrieved the messages                       |
| 404 (Not Found) | &lt;empty&gt;                     | A subscription with the specified name could not be found |

#### Ack - (POST) /api/v1/subscriptions/&lt;subscription&gt;/ack <a name="subscription_ack"></a>

Acknowledged that messages have been processed. Updates the subscriptions `updated` field. Returns only the ids which
where successfully acked.

```js
{
  "message_ids": "string[]"
}
```

##### Request

| Parameter    | Description                            | Units | Format | Required |
| ------------ | -------------------------------------- | ----- | ------ | -------- |
| subscription | The name of the subscription           | n/a   | path   | true     |
| message_ids  | The ids of the messaged to acknowledge | n/a   | body   | true     |

##### Response

| Status Code     | Response Body                          | Description                                               |
| --------------- | -------------------------------------- | --------------------------------------------------------- |
| 200 (Ok)        | [MessageIdList](#message_id_list_type) | Successfully acknowledged the messages                    |
| 404 (Not Found) | &lt;empty&gt;                          | A subscription with the specified name could not be found |

## Develop

This project makes heavy use of the [rust](https://www.rust-lang.org/en-US/) ecosystem. It is highly recommended to use [rustup](https://rustup.rs/) and [cargo](https://github.com/rust-lang/cargo) when working on Courier.

Courier also depends on:

- [rustfmt](https://github.com/rust-lang-nursery/rustfmt) - for code formatting
- [clippy](https://github.com/rust-lang-nursery/rust-clippy) - for linting
- [tarpaulin](https://github.com/xd009642/tarpaulin) - for code coverage
- [cross](https://github.com/japaric/cross) - for cross compilation

These can be installed with:

    > rustup component add rustfmt-preview
    > cargo +nightly install clippy
    > cargo install cargo-tarpaulin
    > cargo install cross

Run the application

    > cargo run

Run the test suite

    > cargo test

Check test coverage

    > cargo tarpaulin --ignore-tests --line --no-count

Check the code formating

    > cargo fmt --all -- --check

Lint with clippy

    > cargo +nightly clippy --lib --bins

Perform a cross release build with the [musl](https://www.musl-libc.org/) target

    > cross build --release --target=x86_64-unknown-linux-musl

## Web Development

Courier uses [yarn](https://yarnpkg.com/) for web development, but the commands should work equally well with npm.

Install dependencies

    > yarn install

Start the development server

    > yarn start

Create a production build

    > yarn build

Clean

    > yarn clean

## Example Commands

Create a topic and subscription. Publish a message to the topic and then pull and ack the message.

    > curl -X PUT -H "Content-Type: application/json" -d '{"message_ttl": 600}' http://localhost:3140/api/v1/topics/topic0 && echo
    > curl -X PUT -H "Content-Type: application/json" -d '{"topic": "topic0", "ack_deadline": 60}' http://localhost:3140/api/v1/subscriptions/sub0 && echo
    > curl -X POST -H "content-Type: application/json" -d '{"messages": [{"data": "testing 123"}]}' http://localhost:3140/api/v1/topics/topic0/publish && echo
    > curl -X POST -H "Content-Type: application/json" -d '{}' http://localhost:3140/api/v1/subscriptions/sub0/pull && echo
    > curl -X POST -H "Content-Type: application/json" -d '{"message_ids": ["<some id>"]}' http://localhost:3140/api/v1/subscriptions/sub0/ack && echo
