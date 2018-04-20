# Courier

[![Build Status](https://travis-ci.org/davidMcneil/courier.svg?branch=master)](https://travis-ci.org/davidMcneil/courier)
[![Coverage Status](https://coveralls.io/repos/github/davidMcneil/courier/badge.svg)](https://coveralls.io/github/davidMcneil/courier)
[![LoC](https://tokei.rs/b1/github/davidMcneil/courier)](https://github.com/davidMcneil/courier)
[![License](http://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![License](http://img.shields.io/badge/license-APACHE-blue.svg)](./LICENSE-APACHE)

A simple pub/sub service.

Courier provides an in-memory, non-distributed pub/sub service with an http, json interface. There are three objects that apps using Courier interact with **messages**, **topics**, and **subscriptions**. The basic flow is that apps **pub**lish messages to a given topic while **sub**scribers read messages from the topic to which they are subscribed.

## Install

## HTTP JSON API

**Table of Contents**

[Topic End Points](#topic_end_points)

* [Create](#topic_create)
* [Update](#topic_update)
* [Delete](#topic_delete)
* [Get](#topic_get)
* [List](#topic_list)
* [Subscriptions](#topic_subscriptions)
* [Publish](#topic_publish)

[Subscription End Points](#subscription_end_points)

* [Create](#subscription_create)
* [Update](#subscription_update)
* [Delete](#subscription_delete)
* [Get](#subscription_get)
* [List](#subscription_list)
* [Pull](#subscription_pull)
* [Ack](#subscription_ack)

All messages which require the following http headers to be set:

| Headers      | Value            |
| ------------ | ---------------- |
| Content-Type | application/json |

## Types

### Topic <a name="topic_type"></a>

```json
{
    name: <string>,
    message_ttl: <u32>,
}
```

### Subscription <a name="subscription_type"></a>

```json
{
    name: <string>,
    topic: <string>,
    ack_deadline: <u32>,
}
```

### Topic End Points <a name="topic_end_points"></a>

#### Create - (PUT) /api/v0/topics/create/<topic> <a name="topic_create"></a>

Create a new topic.

##### Request

```json
{
  message_ttl: <u32>,
}
```

| Parameter   | Description                                                            | Units   | Format | Required |
| ----------- | ---------------------------------------------------------------------- | ------- | ------ | -------- |
| topic       | The unique name of the topic, a random name will be generated if empty |         | path   | false    |
| message_ttl | The time to live (ttl) applied to all messages                         | seconds | body   | false    |

##### Response

| Status Code    | Response Body        | Description                                                                     |
| -------------- | -------------------- | ------------------------------------------------------------------------------- |
| 201 (Created)  | [Topic](#topic_type) | Successfully created a new topic                                                |
| 409 (Conflict) | &lt;empty&gt;        | Could not create a topic because a topic with the specified name already exists |

#### Update - (PATCH) /api/v0/topics/update/<topic> <a name="topic_update"></a>

Update a topic.

##### Request

```json
{
  message_ttl: <u32>,
}
```

| Parameter   | Description                                                            | Units   | Format | Required |
| ----------- | ---------------------------------------------------------------------- | ------- | ------ | -------- |
| topic       | The unique name of the topic, a random name will be generated if empty |         | path   | true     |
| message_ttl | The time to live (ttl) applied to all messages                         | seconds | body   | false    |

##### Response

| Status Code     | Response Body        | Description                                        |
| --------------- | -------------------- | -------------------------------------------------- |
| 200 (Ok)        | [Topic](#topic_type) | Successfully updated the topic                     |
| 404 (Not Found) | &lt;empty&gt;        | A topic with the specified name could not be found |

#### Delete - (DELETE) /api/v0/topics/delete/<topic> <a name="topic_delete"></a>

Delete a topic.

##### Request

| Parameter | Description                     | Units | Format | Required |
| --------- | ------------------------------- | ----- | ------ | -------- |
| topic     | The name of the topic to delete |       | path   | true     |

##### Response

| Status Code     | Response Body | Description                                        |
| --------------- | ------------- | -------------------------------------------------- |
| 200 (Ok)        | &lt;empty&gt; | Successfully deleted the topic                     |
| 404 (Not Found) | &lt;empty&gt; | A topic with the specified name could not be found |

### Subscription End Points <a name="subscription_end_points"></a>

#### Create - (PUT) /api/v0/subscriptions/create/<subscription> <a name="subscription_create"></a>

Create a new subscription.

##### Request

```json
{
  ack_deadline: <u32>,
}
```

| Parameter    | Description                                                                   | Units   | Format | Required |
| ------------ | ----------------------------------------------------------------------------- | ------- | ------ | -------- |
| subscription | The unique name of the subscription, a random name will be generated if empty |         | path   | false    |
| topic        | The name of the topic to subscribe                                            |         | body   | true     |
| ack_deadline | The amount of time given to ack a message before it is resent                 | seconds | body   | false    |

##### Response

| Status Code    | Response Body                      | Description                                                                                   |
| -------------- | ---------------------------------- | --------------------------------------------------------------------------------------------- |
| 201 (Created)  | [subscription](#subscription_type) | Successfully created a new subscription                                                       |
| 409 (Conflict) | &lt;empty&gt;                      | Could not create a subscription because a subscription with the specified name already exists |

#### Update - (PATCH) /api/v0/subscriptions/update/<subscription> <a name="subscription_update"></a>

Update a subscription.

##### Request

```json
{
  ack_deadline: <u32>,
}
```

| Parameter    | Description                                                                   | Units   | Format | Required |
| ------------ | ----------------------------------------------------------------------------- | ------- | ------ | -------- |
| subscription | The unique name of the subscription, a random name will be generated if empty |         | path   | true     |
| ack_deadline | The amount of time given to ack a message before it is resent                 | seconds | body   | false    |

##### Response

| Status Code     | Response Body                      | Description                                               |
| --------------- | ---------------------------------- | --------------------------------------------------------- |
| 200 (Ok)        | [Subscription](#subscription_type) | Successfully updated the subscription                     |
| 404 (Not Found) | &lt;empty&gt;                      | A subscription with the specified name could not be found |

#### Delete - (DELETE) /api/v0/subscriptions/delete/<subscription> <a name="subscription_delete"></a>

Delete a subscription.

##### Request

| Parameter    | Description                            | Units | Format | Required |
| ------------ | -------------------------------------- | ----- | ------ | -------- |
| subscription | The name of the subscription to delete |       | path   | true     |

##### Response

| Status Code     | Response Body | Description                                               |
| --------------- | ------------- | --------------------------------------------------------- |
| 200 (Ok)        | &lt;empty&gt; | Successfully deleted the subscription                     |
| 404 (Not Found) | &lt;empty&gt; | A subscription with the specified name could not be found |

## Develop

Examples
curl -X PUT -H "Content-Type: application/json" -d '{"message_ttl": 600}' http://localhost:8000/api/v0/topics/testing && echo
curl -X POST -H "content-Type: application/json" -d '{"messages": [{"data": "test"}]}' http://localhost:8000/api/v0/topics/testing/publish && echo
curl -X PUT -H "Content-Type: application/json" -d '{"topic": "testing", "ack_deadline": 60}' http://localhost:8000/api/v0/subscriptions/sub0 && echo
curl -X POST -H "Content-Type: application/json" http://localhost:8000/api/v0/subscriptions/sub0/pull && echo
curl -X POST -H "Content-Type: application/json" -d '{"message_ids": ["c639b587-e5b7-4fdd-92ee-42ddc03aca54"]}' http://localhost:8000/api/v0/subscriptions/sub0/ack && echo
