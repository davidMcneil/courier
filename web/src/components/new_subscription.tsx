import { ChangeEvent, Component, FormEvent } from "inferno";
import { subscriptionFromJson } from "../utils/data_parsers";
import { NotificationType } from "../utils/types";
import { fetchError2message, HEADERS, str2uint, subscriptionsUrl } from "../utils/util";

interface Props {
  setNotification: (type: NotificationType, message: string) => void;
}

interface State {
  name: string;
  topic: string;
  ackDeadline: number;
  ttl: number;
  historical: boolean;
}

const emptyState: State = {
  name: "",
  topic: "",
  ackDeadline: 60,
  ttl: 0,
  historical: false,
};

export class NewSubscription extends Component<Props, State> {
  public state = emptyState;

  constructor(props: null, context: null) {
    super(props, context);

    this.setName = this.setName.bind(this);
    this.setTopic = this.setTopic.bind(this);
    this.setAckDeadline = this.setAckDeadline.bind(this);
    this.setTtl = this.setTtl.bind(this);
    this.setHistorical = this.setHistorical.bind(this);
    this.create = this.create.bind(this);
  }

  public render() {
    return (
      <div>
        <div class="field">
          <label class="label">Subscription Name</label>
          <div class="control">
            <input class="input" type="text" value={this.state.name} onInput={this.setName} />
          </div>
        </div>

        <div class="field">
          <label class="label">Topic</label>
          <div class="control">
            <input class="input" type="text" value={this.state.topic} onInput={this.setTopic} />
          </div>
        </div>

        <div class="field">
          <label class="label">Ack Deadline (s)</label>
          <div class="control">
            <input
              class="input"
              type="number"
              value={this.state.ackDeadline}
              onInput={this.setAckDeadline}
            />
          </div>
        </div>

        <div class="field">
          <label class="label">Time to Live (s)</label>
          <div class="control">
            <input class="input" type="number" value={this.state.ttl} onInput={this.setTtl} />
          </div>
        </div>

        <label class="checkbox">
          <input
            class="checkbox"
            type="checkbox"
            checked={this.state.historical}
            onChange={this.setHistorical}
          />
          &nbsp;Historical
        </label>

        <div class="field is-grouped is-grouped-right">
          <div class="control">
            <input class="button is-primary" type="button" value="Create" onClick={this.create} />
          </div>
        </div>
      </div>
    );
  }

  private setName(event: FormEvent<HTMLInputElement>) {
    this.setState({ name: event.currentTarget.value });
  }

  private setTopic(event: FormEvent<HTMLInputElement>) {
    this.setState({ topic: event.currentTarget.value });
  }

  private setAckDeadline(event: FormEvent<HTMLInputElement>) {
    this.setState({ ackDeadline: str2uint(event.currentTarget.value) });
  }

  private setTtl(event: FormEvent<HTMLInputElement>) {
    this.setState({ ttl: str2uint(event.currentTarget.value) });
  }

  private setHistorical(event: ChangeEvent<HTMLInputElement>) {
    this.setState({ historical: event.currentTarget.checked });
  }

  private create() {
    const body = {
      topic: this.state.topic,
      ack_deadline: this.state.ackDeadline,
      ttl: this.state.ttl,
      historical: this.state.historical,
    };
    const init = { method: "PUT", headers: HEADERS, body: JSON.stringify(body) };
    fetch(subscriptionsUrl(this.state.name), init)
      .then(response => {
        if (!response.ok) {
          throw response;
        }
        return response.json();
      })
      .then(json => {
        const subscription = subscriptionFromJson(json);
        if (subscription !== null) {
          this.props.setNotification(
            NotificationType.Success,
            `Created subscription '${subscription.name}'.`,
          );
        } else {
          this.props.setNotification(NotificationType.Failure, `Unable to parse subscription!`);
        }
      })
      .catch(error => {
        const message = `Unable to create subscription! (${fetchError2message(error)})`;
        this.props.setNotification(NotificationType.Failure, message);
      });
  }
}
