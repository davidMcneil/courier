import { Component } from "inferno";

import { SubscriptionMetrics } from "../utils/data_parsers";
import { NotificationType } from "../utils/types";
import {
  fetchError2message,
  HEADERS,
  numberAsPercentage,
  numberAsTimeStr,
  subscriptionsUrl,
} from "../utils/util";

interface Props {
  metrics: SubscriptionMetrics;
  setNotification: (type: NotificationType, message: string) => void;
  setDeleteConfirmation: (message: string, action: () => void) => void;
}

interface State {
  expanded: boolean;
}

export class Subscription extends Component<Props, State> {
  public state = { expanded: false };

  constructor(props: null, context: null) {
    super(props, context);

    this.toggleExpanded = this.toggleExpanded.bind(this);
    this.tryDelete = this.tryDelete.bind(this);
    this.delete = this.delete.bind(this);
  }

  public render() {
    const m = this.props.metrics;
    const expanded = this.state.expanded;
    const age = (new Date().getTime() - m.created.getTime()) / 1000;
    let expires: string | number = "never";
    if (m.ttl !== 0) {
      const updatedAgo = (new Date().getTime() - m.updated.getTime()) / 1000;
      expires = Math.ceil(m.ttl - updatedAgo);
    }
    return (
      <tbody key={m.name} class={`${expanded ? "no-bottom-border" : ""}`}>
        <tr>
          <td class={"is-table-icon has-text-centered"}>
            <a onClick={this.toggleExpanded}>
              {expanded ? <span class="arrow-down" /> : <span class="arrow-right" />}
            </a>
          </td>
          <td>{m.name}</td>
          <td class={m.orphaned ? "has-text-danger" : ""}>{m.topic}</td>
          <td>{m.pending}</td>
          <td>
            {m.normalizedMessageIndex}&nbsp;/&nbsp;{m.topicMessages}
          </td>
          <td>{numberAsPercentage(m.percentageProcessed)}</td>
          <td>{numberAsTimeStr(age)}</td>
          <td class={"is-table-icon has-text-centered"}>
            <a class="delete is-small" onClick={this.tryDelete} />
          </td>
        </tr>
        <tr class={expanded ? "" : "is-hidden"}>
          <td />
          <td colSpan={7}>
            <table class="table is-bordered is-narrow is-fullwidth">
              <thead>
                <th>Pulled</th>
                <th>Retries</th>
                <th>Acks</th>
                <th>Acked</th>
                <th>Ack Deadline (s)</th>
                <th>TTL (s)</th>
                <th>Expires (s)</th>
                <th>Updated</th>
                <th>Created</th>
              </thead>
              <tr>
                <td>{m.pulledAllTime}</td>
                <td>{m.pulledRetriesAllTime}</td>
                <td>{m.acksAllTime}</td>
                <td>{m.ackedAllTime}</td>
                <td>{m.ackDeadline}</td>
                <td>{m.ttl}</td>
                <td>{expires} </td>
                <td>{m.updated.toISOString()}</td>
                <td>{m.created.toISOString()}</td>
              </tr>
            </table>
          </td>
        </tr>
      </tbody>
    );
  }

  private toggleExpanded() {
    this.setState({ expanded: !this.state.expanded });
  }

  private tryDelete() {
    this.props.setDeleteConfirmation(
      `Are you sure you want to delete subscription '${this.props.metrics.name}'?`,
      this.delete,
    );
  }

  private delete() {
    const name = this.props.metrics.name;
    const init = { method: "DELETE", headers: HEADERS };
    fetch(subscriptionsUrl(name), init)
      .then(response => {
        if (!response.ok) {
          throw response;
        }
        this.props.setNotification(NotificationType.Success, `Deleted subscription '${name}'.`);
      })
      .catch(error => {
        const message = `Unable to delete subscription '${name}'! (${fetchError2message(error)})`;
        this.props.setNotification(NotificationType.Failure, message);
      });
  }
}
