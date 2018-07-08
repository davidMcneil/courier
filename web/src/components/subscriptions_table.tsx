import { SubscriptionMetrics } from "../utils/data_parsers";
import { NotificationType } from "../utils/types";
import { Subscription } from "./subscription";

interface Props {
  subscriptions: SubscriptionMetrics[];
  setNotification: (type: NotificationType, message: string) => void;
  setDeleteConfirmation: (message: string, action: () => void) => void;
}

export function SubscriptionsTable(props: Props) {
  let subscriptions: JSX.Element[] = props.subscriptions
    .sort((a, b) => {
      if (a.name < b.name) {
        return -1;
      }
      if (a.name > b.name) {
        return 1;
      }
      return 0;
    })
    .map(s => (
      <Subscription
        key={`row_${s.name}`}
        metrics={s}
        setNotification={props.setNotification}
        setDeleteConfirmation={props.setDeleteConfirmation}
      />
    ));
  if (subscriptions.length === 0) {
    subscriptions = [
      <tr>
        <td colSpan={8} class="has-text-centered has-text-weight-bold">
          No Subscriptions
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
          <tr>
            <th />
            <th>Subscription</th>
            <th>Topic</th>
            <th>Pending</th>
            <th>Index</th>
            <th>Processed</th>
            <th>Age</th>
            <th />
          </tr>
        </thead>,
        ...subscriptions,
      ]}
    </table>
  );
}
