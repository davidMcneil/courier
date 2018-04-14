# Courier

[![Build Status](https://travis-ci.org/davidMcneil/courier.svg?branch=master)](https://travis-ci.org/davidMcneil/courier)
[![Coverage Status](https://coveralls.io/repos/github/davidMcneil/courier/badge.svg)](https://coveralls.io/github/davidMcneil/courier)
[![LoC](https://tokei.rs/b1/github/davidMcneil/courier)](https://github.com/davidMcneil/courier)
[![License](http://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![License](http://img.shields.io/badge/license-APACHE-blue.svg)](./LICENSE-APACHE)

A simple pub/sub service.

Courier provides an in-memory, non-distributed pub/sub service with an http, json interface. There are three objects that apps using Courier interact with **messages**, **topics**, and **subscriptions**. The basic flow is that apps **pub**lish messages to a given topic while **sub**scribers read messages from the topic to which they are subscribed.

Examples
curl -X PUT -H "Content-Type: application/json" -d '{"message_ttl": 600}' http://localhost:8000/api/v0/topics/testing && echo
curl -X POST -H "content-Type: application/json" -d '{"messages": [{"data": "test"}]}' http://localhost:8000/api/v0/topics/testing/publish && echo
curl -X PUT -H "Content-Type: application/json" -d '{"topic": "testing", "ack_deadline": 60}' http://localhost:8000/api/v0/subscriptions/sub0 && echo
curl -X POST -H "Content-Type: application/json" http://localhost:8000/api/v0/subscriptions/sub0/pull && echo
curl -X POST -H "Content-Type: application/json" -d '{"message_ids": ["c639b587-e5b7-4fdd-92ee-42ddc03aca54"]}' http://localhost:8000/api/v0/subscriptions/sub0/ack && echo
