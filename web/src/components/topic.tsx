import classNames from "classnames";

import { Component } from "inferno";
import { SubscriptionMetrics, TopicMetrics } from "../utils/data_parsers";
import { NotificationType } from "../utils/types";
import {
  fetchError2message,
  HEADERS,
  numberAsPercentage,
  numberAsTimeStr,
  topicsUrl,
} from "../utils/util";
import { SubscriptionsTable } from "./subscriptions_table";

interface Props {
  metrics: TopicMetrics;
  subscriptions: SubscriptionMetrics[];
  setNotification: (type: NotificationType, message: string) => void;
  setDeleteConfirmation: (message: string, action: () => void) => void;
}

interface State {
  expanded: boolean;
}

export class Topic extends Component<Props, State> {
  public state = { expanded: false };

  constructor(props: null, context: null) {
    super(props, context);

    this.toggleExpanded = this.toggleExpanded.bind(this);
    this.tryDelete = this.tryDelete.bind(this);
    this.delete = this.delete.bind(this);
  }

  public render() {
    const m = this.props.metrics;
    const subscriptions = this.props.subscriptions;
    const expanded = this.state.expanded;
    const age = (new Date().getTime() - m.created.getTime()) / 1000;
    let expires: string | number = "never";
    if (m.ttl !== 0) {
      const updatedAgo = (new Date().getTime() - m.updated.getTime()) / 1000;
      expires = Math.ceil(m.ttl - updatedAgo);
    }
    return (
      <tbody class={classNames({ "no-bottom-border": expanded })}>
        <tr>
          <td class="is-table-icon has-text-centered">
            <a onClick={this.toggleExpanded}>
              {expanded ? <span class="arrow-down" /> : <span class="arrow-right" />}
            </a>
          </td>
          <td>{m.name}</td>
          <td>{m.messages}</td>
          <td>{subscriptions.length}</td>
          <td>{numberAsPercentage(m.percentageProcessed)}</td>
          <td>{numberAsTimeStr(age)}</td>
          <td class="is-table-icon has-text-centered">
            <a class="delete is-small" onClick={this.tryDelete} />
          </td>
        </tr>
        <tr class={classNames({ "is-hidden": !expanded })}>
          <td />
          <td colSpan={5}>
            <table class="table is-bordered is-narrow is-fullwidth">
              <thead>
                <th>Messages Interval</th>
                <th>Expired Interval</th>
                <th>Messages All Time</th>
                <th>Expired All Time</th>
                <th>Message TTL (s)</th>
                <th>TTL (s)</th>
                <th>Expires (s)</th>
                <th>Updated</th>
                <th>Created</th>
              </thead>
              <tr>
                <td>{m.messagesInterval}</td>
                <td>{m.expiredInterval}</td>
                <td>{m.messagesAllTime}</td>
                <td>{m.expiredAllTime}</td>
                <td>{m.messageTtl}</td>
                <td>{m.ttl}</td>
                <td>{expires} </td>
                <td>{m.updated.toISOString()}</td>
                <td>{m.created.toISOString()}</td>
              </tr>
            </table>
            <SubscriptionsTable
              subscriptions={subscriptions}
              setNotification={this.props.setNotification}
              setDeleteConfirmation={this.props.setDeleteConfirmation}
            />
          </td>
          <td />
        </tr>
      </tbody>
    );
  }

  private toggleExpanded() {
    this.setState({ expanded: !this.state.expanded });
  }

  private tryDelete() {
    this.props.setDeleteConfirmation(
      `Are you sure you want to delete topic '${this.props.metrics.name}'?`,
      this.delete,
    );
  }

  private delete() {
    const name = this.props.metrics.name;
    const init = { method: "DELETE", headers: HEADERS };
    fetch(topicsUrl(name), init)
      .then(response => {
        if (!response.ok) {
          throw response;
        }
        this.props.setNotification(NotificationType.Success, `Deleted topic '${name}'.`);
      })
      .catch(error => {
        const message = `Unable to delete topic '${name}'! (${fetchError2message(error)})`;
        this.props.setNotification(NotificationType.Failure, message);
      });
  }
}
