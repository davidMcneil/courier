import { Component } from "inferno";
import { TopicMetrics } from "../utils/data_parsers";

interface Props {
  metrics: TopicMetrics;
}

export function Topic(props: Props) {
  return (
    <tr>
      <td>{props.metrics.name}</td>
      <td>{props.metrics.numMessages}</td>
      <td>{props.metrics.numMessagesInterval}</td>
      <td>{props.metrics.numMessagesAllTime}</td>
      <td>{props.metrics.messageTtl}</td>
    </tr>
  );
}
