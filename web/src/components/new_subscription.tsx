import { ChangeEvent, Component, FormEvent } from "inferno";
import { HEADERS, logError, str2uint, subscriptionsUrl } from "../utils/util";

interface State {
  name: string;
  topic: string;
  ackDeadline: number;
  historical: boolean;
}

const emptyState: State = {
  name: "",
  topic: "",
  ackDeadline: 60,
  historical: false,
};

export class NewSubscription extends Component<null, State> {
  public state = emptyState;

  constructor(props: null, context: null) {
    super(props, context);

    this.setName = this.setName.bind(this);
    this.setTopic = this.setTopic.bind(this);
    this.setAckDeadline = this.setAckDeadline.bind(this);
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
          <label class="label">Ack Deadline</label>
          <div class="control">
            <input
              class="input"
              type="number"
              value={this.state.ackDeadline}
              onInput={this.setAckDeadline}
            />
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

  private setHistorical(event: ChangeEvent<HTMLInputElement>) {
    this.setState({ historical: event.currentTarget.checked });
  }

  private create() {
    const body = {
      topic: this.state.topic,
      ack_deadline: this.state.ackDeadline,
      historical: this.state.historical,
    };
    const init = { method: "PUT", headers: HEADERS, body: JSON.stringify(body) };
    fetch(subscriptionsUrl(this.state.name), init)
      .then(response => {
        if (!response.ok) {
          throw new Error(`Response was ${response.status}.`);
        }
      })
      .catch(error => {
        logError("Failed to create new subscription!", error);
      });
  }
}
