import { isNumber, isObject, isString, isUndefined } from "./util";

export interface TopicMetrics {
  name: string;
  messageTtl: number;
  ttl: number;
  numMessagesAllTime: number;
  numExpiredAllTime: number;
  numMessagesInterval: number;
  numExpiredInterval: number;
  numMessages: number;
  percentageProcessed: number;
  created: Date;
  updated: Date;
}

function newTopicMetrics(name: string): TopicMetrics {
  return {
    name,
    messageTtl: 0,
    ttl: 0,
    numMessagesAllTime: 0,
    numExpiredAllTime: 0,
    numMessagesInterval: 0,
    numExpiredInterval: 0,
    numMessages: 0,
    percentageProcessed: 0,
    created: new Date(),
    updated: new Date(),
  };
}

function topicMetricsFromAny(name: string, blob: any): TopicMetrics {
  const topicMetrics = newTopicMetrics(name);
  if (isObject(blob)) {
    if (isNumber(blob.messages)) {
      topicMetrics.numMessages = blob.messages;
    }
    if (isNumber(blob.messages_all_time)) {
      topicMetrics.numMessagesAllTime = blob.messages_all_time;
    }
    if (isNumber(blob.expired_all_time)) {
      topicMetrics.numExpiredAllTime = blob.expired_all_time;
    }
    if (isNumber(blob.message_ttl)) {
      topicMetrics.messageTtl = blob.message_ttl;
    }
    if (isNumber(blob.ttl)) {
      topicMetrics.ttl = blob.ttl;
    }
    const created = new Date(blob.created);
    if (isNumber(created.getTime())) {
      topicMetrics.created = created;
    }
    const updated = new Date(blob.updated);
    if (isNumber(updated.getTime())) {
      topicMetrics.updated = updated;
    }
  }
  return topicMetrics;
}

export interface SubscriptionMetrics {
  name: string;
  topic: string;
  ackDeadline: number;
  ttl: number;
  topicNumMessages: number;
  topicMessageIndex: number;
  normalizedTopicMessageIndex: number;
  numPulledAllTime: number;
  numAckedAllTime: number;
  numPulledInterval: number;
  numAckedInterval: number;
  numPending: number;
  percentageProcessed: number;
  orphaned: boolean;
  created: Date;
  updated: Date;
}

function newSubscriptionMetrics(name: string): SubscriptionMetrics {
  return {
    name,
    topic: "",
    ackDeadline: 0,
    ttl: 0,
    topicNumMessages: 0,
    topicMessageIndex: 0,
    normalizedTopicMessageIndex: 0,
    numPulledAllTime: 0,
    numAckedAllTime: 0,
    numPulledInterval: 0,
    numAckedInterval: 0,
    numPending: 0,
    percentageProcessed: 0,
    orphaned: false,
    created: new Date(),
    updated: new Date(),
  };
}

function subscriptionMetricsFromAny(name: string, blob: any): SubscriptionMetrics {
  const subscriptionMetrics = newSubscriptionMetrics(name);
  if (isObject(blob)) {
    if (isNumber(blob.pending)) {
      subscriptionMetrics.numPending = blob.pending;
    }
    if (isNumber(blob.pulled_all_time)) {
      subscriptionMetrics.numPulledAllTime = blob.pulled_all_time;
    }
    if (isNumber(blob.acked_all_time)) {
      subscriptionMetrics.numAckedAllTime = blob.acked_all_time;
    }
    if (isNumber(blob.topic_message_index)) {
      subscriptionMetrics.topicMessageIndex = blob.topic_message_index;
    }
    if (isNumber(blob.ack_deadline)) {
      subscriptionMetrics.ackDeadline = blob.ack_deadline;
    }
    if (isNumber(blob.ttl)) {
      subscriptionMetrics.ttl = blob.ttl;
    }
    if (isString(blob.topic)) {
      subscriptionMetrics.topic = blob.topic;
    }
    const created = new Date(blob.created);
    if (isNumber(created.getTime())) {
      subscriptionMetrics.created = created;
    }
    const updated = new Date(blob.updated);
    if (isNumber(updated.getTime())) {
      subscriptionMetrics.updated = updated;
    }
  }
  return subscriptionMetrics;
}

export interface CourierState {
  numTopicsAllTime: number;
  numMessagesAllTime: number;
  numExpiredAllTime: number;
  numTopicsInterval: number;
  numMessagesInterval: number;
  numExpiredInterval: number;
  numTopics: number;
  numMessages: number;
  numSubscriptionsAllTime: number;
  numPulledAllTime: number;
  numAckedAllTime: number;
  numSubscriptionsInterval: number;
  numPulledInterval: number;
  numAckedInterval: number;
  numSubscriptions: number;
  numPending: number;
  percentageProcessed: number;
  memoryResidentSetSizeInterval: number;
  memoryResidentSetSize: number;
  startTime: Date;
  topic2subscriptions: Map<string, SubscriptionMetrics[]>;
  topics: Map<string, TopicMetrics>;
  subscriptions: Map<string, SubscriptionMetrics>;
}

export function newCourierState(): CourierState {
  return {
    numTopicsAllTime: 0,
    numMessagesAllTime: 0,
    numExpiredAllTime: 0,
    numTopicsInterval: 0,
    numMessagesInterval: 0,
    numExpiredInterval: 0,
    numTopics: 0,
    numMessages: 0,
    numSubscriptionsAllTime: 0,
    numPulledAllTime: 0,
    numAckedAllTime: 0,
    numSubscriptionsInterval: 0,
    numPulledInterval: 0,
    numAckedInterval: 0,
    numSubscriptions: 0,
    numPending: 0,
    percentageProcessed: 0,
    memoryResidentSetSizeInterval: 0,
    memoryResidentSetSize: 0,
    startTime: new Date(),
    topic2subscriptions: new Map<string, SubscriptionMetrics[]>(),
    topics: new Map<string, TopicMetrics>(),
    subscriptions: new Map<string, SubscriptionMetrics>(),
  };
}

function computeState(
  current: CourierState,
  previous: CourierState = newCourierState(),
): CourierState {
  const c = current;
  const p = previous;
  c.memoryResidentSetSizeInterval = c.memoryResidentSetSize - p.memoryResidentSetSize;

  // Topic metrics
  c.numTopics = c.topics.size;
  for (const [, metrics] of Array.from(current.topics.entries())) {
    c.numMessagesAllTime += metrics.numMessagesAllTime;
    c.numExpiredAllTime += metrics.numExpiredAllTime;
    c.numMessages += metrics.numMessages;
  }
  c.numTopicsInterval = c.numTopicsAllTime - p.numTopicsAllTime;
  c.numMessagesInterval = c.numMessagesAllTime - p.numMessagesAllTime;
  c.numExpiredInterval = c.numExpiredAllTime - p.numExpiredAllTime;

  // Subscription metrics
  let totalNumUnprocessed = 0;
  let totalNumSubscriptionMessages = 0;
  c.numSubscriptions = c.subscriptions.size;
  for (const [name, metrics] of Array.from(current.subscriptions.entries())) {
    const topicName = metrics.topic;
    const topicMetrics = c.topics.get(topicName);

    // Check if the subscription has been orphaned
    if (isUndefined(topicMetrics)) {
      metrics.orphaned = true;
      continue;
    }

    // Create a topic to subscriptions lookup
    if (!c.topic2subscriptions.has(topicName)) {
      c.topic2subscriptions.set(topicName, []);
    }
    c.topic2subscriptions.get(topicName).push(metrics);

    // Get the previous metrics subscription metrics
    let previousMetrics = p.subscriptions.get(name);
    if (isUndefined(previousMetrics)) {
      previousMetrics = newSubscriptionMetrics(name);
    }

    // Update subscription specific metrics
    metrics.normalizedTopicMessageIndex =
      topicMetrics.numMessages - (topicMetrics.numMessagesAllTime - metrics.topicMessageIndex);
    metrics.topicNumMessages = topicMetrics.numMessages;
    const numUnprocessed =
      topicMetrics.numMessagesAllTime - metrics.topicMessageIndex + metrics.numPending;
    totalNumUnprocessed += numUnprocessed;
    totalNumSubscriptionMessages += topicMetrics.numMessages;
    let percentageUnprocessed = 0;
    if (topicMetrics.numMessages > 0) {
      percentageUnprocessed = numUnprocessed / topicMetrics.numMessages;
    }
    metrics.percentageProcessed = Math.max(0, 1 - percentageUnprocessed);
    metrics.numPulledInterval = metrics.numPulledAllTime - previousMetrics.numPulledAllTime;
    metrics.numAckedInterval = metrics.numAckedAllTime - previousMetrics.numAckedAllTime;

    // Update global metrics
    c.numPulledAllTime += metrics.numPulledAllTime;
    c.numAckedAllTime += metrics.numAckedAllTime;
    c.numPending += metrics.numPending;
  }
  c.numSubscriptionsInterval = c.numSubscriptionsAllTime - p.numSubscriptionsAllTime;
  c.numPulledInterval = c.numPulledAllTime - p.numPulledAllTime;
  c.numAckedInterval = c.numAckedAllTime - p.numAckedAllTime;
  let totalPercentageUnprocessed = 0;
  if (totalNumSubscriptionMessages > 0) {
    totalPercentageUnprocessed = totalNumUnprocessed / totalNumSubscriptionMessages;
  }
  c.percentageProcessed = Math.max(0, 1 - totalPercentageUnprocessed);
  // Update topic percentage processed
  for (const [topicName, subscriptions] of Array.from(c.topic2subscriptions.entries())) {
    const topicMetrics = c.topics.get(topicName);

    if (isUndefined(topicMetrics)) {
      continue;
    }

    let totalMessages = 0;
    let numUnprocessed = 0;
    for (const subscription of subscriptions) {
      totalMessages += subscription.topicNumMessages;
      numUnprocessed +=
        topicMetrics.numMessagesAllTime - subscription.topicMessageIndex + subscription.numPending;
    }
    const percentageUnprocessed = 0;
    if (totalMessages > 0) {
      totalPercentageUnprocessed = numUnprocessed / totalMessages;
    }
    topicMetrics.percentageProcessed = Math.max(0, 1 - totalPercentageUnprocessed);
  }
  return current;
}

export function courierStateFromAny(
  blob: any,
  previous: CourierState = newCourierState(),
): CourierState {
  const courierState = newCourierState();
  if (isObject(blob)) {
    if (isNumber(blob.topics_all_time)) {
      courierState.numTopicsAllTime = blob.topics_all_time;
    }
    if (isNumber(blob.subscriptions_all_time)) {
      courierState.numSubscriptionsAllTime = blob.subscriptions_all_time;
    }
    if (isNumber(blob.memory_resident_set_size)) {
      courierState.memoryResidentSetSize = blob.memory_resident_set_size;
    }
    const startTime = new Date(blob.start_time);
    if (isNumber(startTime.getTime())) {
      courierState.startTime = startTime;
    }
    if (isObject(blob.topics)) {
      for (const key of Object.keys(blob.topics)) {
        courierState.topics.set(key, topicMetricsFromAny(key, blob.topics[key]));
      }
    }
    if (isObject(blob.subscriptions)) {
      for (const key of Object.keys(blob.subscriptions)) {
        courierState.subscriptions.set(
          key,
          subscriptionMetricsFromAny(key, blob.subscriptions[key]),
        );
      }
    }
  }
  computeState(courierState, previous);
  return courierState;
}

export interface Message {
  id: string;
  time: Date;
  data: string;
}

export function newMessage(): Message {
  return {
    id: "",
    time: new Date(),
    data: "",
  };
}

export function messageFromMessagesBlob(blob: any): Message | null {
  if (isObject(blob) && Array.isArray(blob.messages) && blob.messages.length > 0) {
    const created = newMessage();
    const message = blob.messages[0];
    if (isString(message.id)) {
      created.id = message.id;
    }
    const time = new Date(message.time);
    if (isNumber(time.getTime())) {
      created.time = new Date();
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

export function topicFromAny(blob: any): Topic | null {
  if (isObject(blob) && isString(blob.name) && isNumber(blob.message_ttl) && isNumber(blob.ttl)) {
    return {
      name: blob.name,
      messageTtl: blob.message_ttl,
      ttl: blob.ttl,
    };
  }
  return null;
}

export interface Subscription {
  name: string;
  topic: string;
  ackDeadline: number;
}

export function subscriptionFromAny(blob: any): Subscription | null {
  if (
    isObject(blob) &&
    isString(blob.name) &&
    isString(blob.topic) &&
    isNumber(blob.ack_deadline)
  ) {
    return {
      name: blob.name,
      topic: blob.topic,
      ackDeadline: blob.ack_deadline,
    };
  }
  return null;
}
