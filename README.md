# Courier

![Travis](https://img.shields.io/travis/davidMcneil/courier.svg) [![Coverage Status](https://coveralls.io/repos/github/davidMcneil/courier/badge.svg?branch=master)](https://coveralls.io/github/davidMcneil/courier?branch=master)
![Coveralls github](https://img.shields.io/coveralls/github/davidMcneil/courier.svg)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT) [![license](http://img.shields.io/badge/license-APACHE-blue.svg)](./LICENSE-APACHE)

A simple pub/sub service.

Courier provides an in-memory, non-distributed pub/sub service with an http, json interface. There are three objects that apps using Courier interact with **messages**, **topics**, and **subscriptions**. The basic flow is that apps **pub**lish messages to a given topic while **sub**scribers read messages from the topic to which they are subscribed.
