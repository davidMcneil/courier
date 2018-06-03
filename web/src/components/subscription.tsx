import { Component } from "inferno";
import { SubscriptionMetrics } from "../utils/data_parsers";

interface Props {
  metrics: SubscriptionMetrics;
}

export function Subscription(props: Props) {
  return (
    <tr>
      <td>{props.metrics.name}</td>
      <td>{props.metrics.topic}</td>
      <td>{props.metrics.ackDeadline}</td>
      <td>{props.metrics.topicMessageIndex}</td>
      <td>{props.metrics.numPulledAllTime}</td>
      <td>{props.metrics.numAckedAllTime}</td>
      <td>{props.metrics.numPulledInterval}</td>
      <td>{props.metrics.numAckedInterval}</td>
      <td>{props.metrics.numPending}</td>
      <td>{props.metrics.percentageProcessed}</td>
      <td>{props.metrics.orphaned}</td>
    </tr>
  );
}
