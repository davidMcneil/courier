import { isNumber, isObject, isString, isUndefined, logError } from "./util";

function percentageFromProcessed(processed: number, total: number): number {
  let percentageProcessed = 1;
  if (total > 0) {
    percentageProcessed = processed / total;
  }
  return Math.max(0, percentageProcessed);
}

export interface TopicMetrics {
  name: string;
  messages: number;
  messagesAllTime: number;
  expiredAllTime: number;
  messageTtl: number;
  ttl: number;
  created: Date;
  updated: Date;
  // Computed fields
  messagesInterval: number;
  expiredInterval: number;
  percentageProcessed: number;
}

function newTopicMetrics(name: string): TopicMetrics {
  return {
    name,
    messages: 0,
    messagesAllTime: 0,
    expiredAllTime: 0,
    messageTtl: 0,
    ttl: 0,
    created: new Date(),
    updated: new Date(),
    messagesInterval: 0,
    expiredInterval: 0,
    percentageProcessed: 0,
  };
}

function topicMetricsFromJson(name: string, json: any): TopicMetrics {
  const topicMetrics = newTopicMetrics(name);
  if (isObject(json)) {
    if (isNumber(json.messages)) {
      topicMetrics.messages = json.messages;
    } else {
      logError("Failed to parse TopicMetrics 'messages' is not a number", json);
    }
    if (isNumber(json.messages_all_time)) {
      topicMetrics.messagesAllTime = json.messages_all_time;
    } else {
      logError("Failed to parse TopicMetrics 'messages_all_time' is not a number", json);
    }
    if (isNumber(json.expired_all_time)) {
      topicMetrics.expiredAllTime = json.expired_all_time;
    } else {
      logError("Failed to parse TopicMetrics 'expired_all_time' is not a number", json);
    }
    if (isNumber(json.message_ttl)) {
      topicMetrics.messageTtl = json.message_ttl;
    } else {
      logError("Failed to parse TopicMetrics 'message_ttl' is not a number", json);
    }
    if (isNumber(json.ttl)) {
      topicMetrics.ttl = json.ttl;
    } else {
      logError("Failed to parse TopicMetrics 'ttl' is not a number", json);
    }
    const created = new Date(json.created);
    if (isNumber(created.getTime())) {
      topicMetrics.created = created;
    } else {
      logError("Failed to parse TopicMetrics 'created' is not a date", json);
    }
    const updated = new Date(json.updated);
    if (isNumber(updated.getTime())) {
      topicMetrics.updated = updated;
    } else {
      logError("Failed to parse TopicMetrics 'updated' is not a date", json);
    }
  } else {
    logError("Failed to parse TopicMetrics is not an object", json);
  }
  return topicMetrics;
}

function computeTopicMetrics(
  metrics: TopicMetrics,
  subscriptions: SubscriptionMetrics[],
  previous: TopicMetrics = newTopicMetrics(""),
) {
  metrics.messagesInterval = metrics.messagesAllTime - previous.messagesAllTime;
  metrics.expiredInterval = metrics.expiredAllTime - previous.expiredAllTime;

  // Calculate percentage unprocessed
  const total = metrics.messages * subscriptions.length;
  let processed = 0;
  for (const subscription of subscriptions) {
    processed += subscription.normalizedMessageIndex - subscription.pending;
  }
  metrics.percentageProcessed = percentageFromProcessed(processed, total);
}

export interface SubscriptionMetrics {
  name: string;
  pending: number;
  pulledAllTime: number;
  pulledRetriesAllTime: number;
  ackedAllTime: number;
  acksAllTime: number;
  topic: string;
  messageIndex: number;
  ackDeadline: number;
  ttl: number;
  created: Date;
  updated: Date;
  // Computed fields
  topicMessages: number;
  normalizedMessageIndex: number;
  pulledInterval: number;
  pulledRetriesInterval: number;
  ackedInterval: number;
  acksInterval: number;
  percentageProcessed: number;
  orphaned: boolean;
}

function newSubscriptionMetrics(name: string): SubscriptionMetrics {
  return {
    name,
    pending: 0,
    pulledAllTime: 0,
    pulledRetriesAllTime: 0,
    ackedAllTime: 0,
    acksAllTime: 0,
    topic: "",
    messageIndex: 0,
    ackDeadline: 0,
    ttl: 0,
    created: new Date(),
    updated: new Date(),
    topicMessages: 0,
    normalizedMessageIndex: 0,
    pulledInterval: 0,
    pulledRetriesInterval: 0,
    ackedInterval: 0,
    acksInterval: 0,
    percentageProcessed: 0,
    orphaned: false,
  };
}

function subscriptionMetricsFromJson(name: string, json: any): SubscriptionMetrics {
  const subscriptionMetrics = newSubscriptionMetrics(name);
  if (isObject(json)) {
    if (isNumber(json.pending)) {
      subscriptionMetrics.pending = json.pending;
    } else {
      logError("Failed to parse SubscriptionMetrics 'pending' is number a date", json);
    }
    if (isNumber(json.pulled_all_time)) {
      subscriptionMetrics.pulledAllTime = json.pulled_all_time;
    } else {
      logError("Failed to parse SubscriptionMetrics 'pulled_all_time' not a number", json);
    }
    if (isNumber(json.pulled_retries_all_time)) {
      subscriptionMetrics.pulledRetriesAllTime = json.pulled_retries_all_time;
    } else {
      logError("Failed to parse SubscriptionMetrics 'pulled_retries_all_time' not a number", json);
    }
    if (isNumber(json.acked_all_time)) {
      subscriptionMetrics.ackedAllTime = json.acked_all_time;
    } else {
      logError("Failed to parse SubscriptionMetrics 'acked_all_time' not a number", json);
    }
    if (isNumber(json.acks_all_time)) {
      subscriptionMetrics.acksAllTime = json.acks_all_time;
    } else {
      logError("Failed to parse SubscriptionMetrics 'acks_all_time' not a number", json);
    }
    if (isString(json.topic)) {
      subscriptionMetrics.topic = json.topic;
    } else {
      logError("Failed to parse SubscriptionMetrics 'topic' is not a string", json);
    }
    if (isNumber(json.message_index)) {
      subscriptionMetrics.messageIndex = json.message_index;
    } else {
      logError("Failed to parse SubscriptionMetrics 'message_index' not a number", json);
    }
    if (isNumber(json.ack_deadline)) {
      subscriptionMetrics.ackDeadline = json.ack_deadline;
    } else {
      logError("Failed to parse SubscriptionMetrics 'ack_deadline' not a number", json);
    }
    if (isNumber(json.ttl)) {
      subscriptionMetrics.ttl = json.ttl;
    } else {
      logError("Failed to parse SubscriptionMetrics 'ttl' is not a number", json);
    }
    const created = new Date(json.created);
    if (isNumber(created.getTime())) {
      subscriptionMetrics.created = created;
    } else {
      logError("Failed to parse SubscriptionMetrics 'created' is not a date", json);
    }
    const updated = new Date(json.updated);
    if (isNumber(updated.getTime())) {
      subscriptionMetrics.updated = updated;
    } else {
      logError("Failed to parse SubscriptionMetrics 'updated' is not a date", json);
    }
  } else {
    logError("Failed to parse SubscriptionMetrics is not an object", json);
  }
  return subscriptionMetrics;
}

function computeSubscriptionMetrics(
  metrics: SubscriptionMetrics,
  topic: TopicMetrics | undefined,
  previous: SubscriptionMetrics = newSubscriptionMetrics(""),
) {
  metrics.pulledInterval = metrics.pulledAllTime - previous.pulledAllTime;
  metrics.pulledRetriesInterval = metrics.pulledRetriesAllTime - previous.pulledRetriesAllTime;
  metrics.ackedInterval = metrics.ackedAllTime - previous.ackedAllTime;
  metrics.acksInterval = metrics.acksAllTime - previous.acksAllTime;

  if (isUndefined(topic)) {
    metrics.orphaned = true;
    return;
  }

  metrics.topicMessages = topic.messages;
  metrics.normalizedMessageIndex = topic.messages - (topic.messagesAllTime - metrics.messageIndex);
  const processed = metrics.normalizedMessageIndex - metrics.pending;
  metrics.percentageProcessed = percentageFromProcessed(processed, topic.messages);
}

export interface CourierState {
  topicsAllTime: number;
  subscriptionsAllTime: number;
  memoryResidentSetSize: number;
  startTime: Date;
  topicMap: Map<string, TopicMetrics>;
  subscriptionMap: Map<string, SubscriptionMetrics>;
  topic2subscriptions: Map<string, SubscriptionMetrics[]>;
  // Computed fields
  topics: number;
  subscriptions: number;
  messages: number;
  pending: number;
  messagesAllTime: number;
  expiredAllTime: number;
  pulledAllTime: number;
  pulledRetriesAllTime: number;
  acksAllTime: number;
  ackedAllTime: number;
  topicsInterval: number;
  subscriptionsInterval: number;
  messagesInterval: number;
  expiredInterval: number;
  pulledInterval: number;
  pulledRetriesInterval: number;
  ackedInterval: number;
  acksInterval: number;
  memoryResidentSetSizeInterval: number;
  percentageProcessed: number;
}

export function newCourierState(): CourierState {
  return {
    topicsAllTime: 0,
    subscriptionsAllTime: 0,
    memoryResidentSetSize: 0,
    startTime: new Date(),
    topicMap: new Map<string, TopicMetrics>(),
    subscriptionMap: new Map<string, SubscriptionMetrics>(),
    topic2subscriptions: new Map<string, SubscriptionMetrics[]>(),
    topics: 0,
    subscriptions: 0,
    messages: 0,
    pending: 0,
    messagesAllTime: 0,
    expiredAllTime: 0,
    pulledAllTime: 0,
    pulledRetriesAllTime: 0,
    acksAllTime: 0,
    ackedAllTime: 0,
    topicsInterval: 0,
    subscriptionsInterval: 0,
    messagesInterval: 0,
    expiredInterval: 0,
    pulledInterval: 0,
    pulledRetriesInterval: 0,
    ackedInterval: 0,
    acksInterval: 0,
    memoryResidentSetSizeInterval: 0,
    percentageProcessed: 0,
  };
}

export function courierStateFromJson(
  json: any,
  previous: CourierState = newCourierState(),
): CourierState {
  const c = newCourierState();
  let pending = 0;
  let pulledAllTime = 0;
  let pulledRetriesAllTime = 0;
  let acksAllTime = 0;
  let ackedAllTime = 0;
  let subscriptionTotalMessages = 0;
  let subscriptionTotalProcessed = 0;
  if (isObject(json)) {
    if (isNumber(json.topics_all_time)) {
      c.topicsAllTime = json.topics_all_time;
    } else {
      logError("Failed to parse CourierState 'topics_all_time' is not a number", json);
    }
    if (isNumber(json.subscriptions_all_time)) {
      c.subscriptionsAllTime = json.subscriptions_all_time;
    } else {
      logError("Failed to parse CourierState 'subscriptions_all_time' is not a number", json);
    }
    if (isNumber(json.memory_resident_set_size)) {
      c.memoryResidentSetSize = json.memory_resident_set_size;
    } else {
      logError("Failed to parse CourierState 'memory_resident_set_size' is not a number", json);
    }
    const startTime = new Date(json.start_time);
    if (isNumber(startTime.getTime())) {
      c.startTime = startTime;
    } else {
      logError("Failed to parse CourierState 'updated' is not a date", json);
    }
    if (isObject(json.topics)) {
      for (const name of Object.keys(json.topics)) {
        c.topicMap.set(name, topicMetricsFromJson(name, json.topics[name]));

        // Initialize a topic to subscription lookup
        c.topic2subscriptions.set(name, []);
      }
    } else {
      logError("Failed to parse CourierState 'topics' is not an array", json);
    }
    if (isObject(json.subscriptions)) {
      for (const name of Object.keys(json.subscriptions)) {
        const subscription = subscriptionMetricsFromJson(name, json.subscriptions[name]);
        c.subscriptionMap.set(name, subscription);
        const topicName = subscription.topic;

        // Calculate computed fields
        const topic = c.topicMap.get(topicName);
        const previousSubscription = previous.subscriptionMap.get(name);
        computeSubscriptionMetrics(subscription, topic, previousSubscription);

        // Insert the subscription into the topic2subscription lookup
        c.topic2subscriptions.get(topicName).push(subscription);

        pending += subscription.pending;
        pulledAllTime += subscription.pulledAllTime;
        pulledRetriesAllTime += subscription.pulledRetriesAllTime;
        acksAllTime += subscription.acksAllTime;
        ackedAllTime += subscription.ackedAllTime;
        subscriptionTotalMessages += topic.messages;
        subscriptionTotalProcessed += subscription.messageIndex - subscription.pending;
      }
    } else {
      logError("Failed to parse CourierState 'subscriptions' is not an array", json);
    }
  } else {
    logError("Failed to parse CourierState is not an object", json);
  }
  // Compute topic computed fields
  let messages = 0;
  let messagesAllTime = 0;
  let expiredAllTime = 0;
  for (const [name, metrics] of Array.from(c.topicMap.entries())) {
    const subscriptions = c.topic2subscriptions.get(name);
    const previousTopic = previous.topicMap.get(name);
    computeTopicMetrics(metrics, subscriptions, previousTopic);

    messages += metrics.messages;
    messagesAllTime += metrics.messagesAllTime;
    expiredAllTime += metrics.expiredAllTime;
  }

  c.topics = c.topicMap.size;
  c.subscriptions = c.subscriptionMap.size;
  c.messages = messages;
  c.pending = pending;
  c.messagesAllTime = messagesAllTime;
  c.expiredAllTime = expiredAllTime;
  c.pulledAllTime = pulledAllTime;
  c.pulledRetriesAllTime = pulledRetriesAllTime;
  c.acksAllTime = acksAllTime;
  c.ackedAllTime = ackedAllTime;
  c.topicsInterval = c.topicsAllTime - previous.topicsAllTime;
  c.subscriptionsInterval = c.subscriptionsAllTime - previous.subscriptionsAllTime;
  c.messagesInterval = c.messagesAllTime - previous.messagesAllTime;
  c.expiredInterval = c.expiredAllTime - previous.expiredAllTime;
  c.pulledInterval = c.pulledAllTime - previous.pulledAllTime;
  c.pulledRetriesInterval = c.pulledRetriesAllTime - previous.pulledRetriesAllTime;
  c.ackedInterval = c.ackedAllTime - previous.ackedAllTime;
  c.acksInterval = c.acksAllTime - previous.acksAllTime;
  c.memoryResidentSetSizeInterval = c.memoryResidentSetSize - previous.memoryResidentSetSize;
  c.percentageProcessed = percentageFromProcessed(
    subscriptionTotalProcessed,
    subscriptionTotalMessages,
  );
  return c;
}

export interface Message {
  id: string;
  time: Date;
  tries: number;
  data: string;
}

export function newMessage(): Message {
  return {
    id: "",
    time: new Date(),
    tries: 0,
    data: "",
  };
}

export function messageFromMessagesJson(json: any): Message | null {
  if (isObject(json) && Array.isArray(json.messages) && json.messages.length > 0) {
    const created = newMessage();
    const message = json.messages[0];
    if (isString(message.id)) {
      created.id = message.id;
    }
    const time = new Date(message.time);
    if (isNumber(time.getTime())) {
      created.time = new Date();
    }
    if (isNumber(message.tries)) {
      created.tries = message.tries;
    }
    if (isString(message.data)) {
      created.data = message.data;
    }
    return created;
  }
  return null;
}

export interface Topic {
  name: string;
  messageTtl: number;
  ttl: number;
}

export function topicFromJson(json: any): Topic | null {
  if (isObject(json) && isString(json.name) && isNumber(json.message_ttl) && isNumber(json.ttl)) {
    return {
      name: json.name,
      messageTtl: json.message_ttl,
      ttl: json.ttl,
    };
  }
  return null;
}

export interface Subscription {
  name: string;
  topic: string;
  ackDeadline: number;
}

export function subscriptionFromJson(json: any): Subscription | null {
  if (
    isObject(json) &&
    isString(json.name) &&
    isString(json.topic) &&
    isNumber(json.ack_deadline)
  ) {
    return {
      name: json.name,
      topic: json.topic,
      ackDeadline: json.ack_deadline,
    };
  }
  return null;
}
