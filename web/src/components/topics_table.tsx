import { SubscriptionMetrics, TopicMetrics } from "../utils/data_parsers";
import { NotificationType } from "../utils/types";
import { Topic } from "./topic";

interface Props {
  topics: TopicMetrics[];
  topic2subscriptions: Map<string, SubscriptionMetrics[]>;
  setNotification: (type: NotificationType, message: string) => void;
  setDeleteConfirmation: (message: string, action: () => void) => void;
}

export function TopicsTable(props: Props) {
  let topics: JSX.Element[] = props.topics
    .sort((a, b) => {
      if (a.name < b.name) {
        return -1;
      }
      if (a.name > b.name) {
        return 1;
      }
      return 0;
    })
    .map(t => {
      let subscriptions = [];
      if (props.topic2subscriptions.has(t.name)) {
        subscriptions = props.topic2subscriptions.get(t.name);
      }
      return (
        <Topic
          key={`row_${t.name}`}
          metrics={t}
          subscriptions={subscriptions}
          setNotification={props.setNotification}
          setDeleteConfirmation={props.setDeleteConfirmation}
        />
      );
    });
  if (topics.length === 0) {
    topics = [
      <tr>
        <td colSpan={7} class="has-text-centered has-text-weight-bold">
          No Topics
        </td>
      </tr>,
    ];
  }
  return (
    <table
      class="table table-with-bottom-border is-hoverable is-narrow is-fullwidth"
      $HasKeyedChildren
    >
      {[
        <thead key="header">
          <th />
          <th>Name</th>
          <th>Messages</th>
          <th>Subscriptions</th>
          <th>Processed</th>
          <th>Age</th>
          <th />
        </thead>,
        ...topics,
      ]}
    </table>
  );
}
