import { Component } from "inferno";
import { TopicMetrics } from "../utils/data_parsers";
import { NotificationType } from "../utils/types";
import { fetchError2message, HEADERS, numberAsTimeStr, topicsUrl } from "../utils/util";

interface Props {
  metrics: TopicMetrics;
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
    const expanded = this.state.expanded;
    const age = (new Date().getTime() - m.created.getTime()) / 1000;
    return (
      <tbody key={m.name} class={`${expanded ? "no-bottom-border" : ""}`}>
        <tr>
          <td class={"is-table-icon has-text-centered"}>
            <a onClick={this.toggleExpanded}>
              {expanded ? <span class="arrow-down" /> : <span class="arrow-right" />}
            </a>
          </td>
          <td>{m.name}</td>
          <td>{m.numMessages}</td>
          <td>0</td>
          <td>{m.percentageProcessed}</td>
          <td>{numberAsTimeStr(age)}</td>
          <td class={"is-table-icon has-text-centered"}>
            <a class="delete is-small" onClick={this.tryDelete} />
          </td>
        </tr>
        <tr class={expanded ? "" : "is-hidden"}>
          <td />
          <td colSpan={5}>
            <table class="table is-bordered is-narrow is-fullwidth">
              <thead>
                <td>Messages Interval</td>
                <td>Messages All Time</td>
                <td>Expired Interval</td>
                <td>Expired All Time</td>
                <td>TTL (s)</td>
                <td>Message TTL (s)</td>
                <td>Updated</td>
                <td>Created</td>
              </thead>
              <tr>
                <td>{m.numMessagesInterval}</td>
                <td>{m.numMessagesAllTime}</td>
                <td>{m.numExpiredInterval}</td>
                <td>{m.numExpiredAllTime}</td>
                <td>0</td>
                <td>{m.messageTtl}</td>
                <td>yyyy-mm-ddThh:mm::ss</td>
                <td>{m.created.toISOString()}</td>
              </tr>
            </table>
            <table>Subscriptions Table</table>
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
