import { Component, FormEvent } from "inferno";
import { HEADERS, logError, str2uint, topicsUrl } from "../utils/util";

interface State {
  name: string;
  ttl: number;
}

const emptyState: State = {
  name: "",
  ttl: 3600,
};

export class NewTopic extends Component<null, State> {
  public state = emptyState;

  constructor(props: null, context: null) {
    super(props, context);

    this.setName = this.setName.bind(this);
    this.setTtl = this.setTtl.bind(this);
    this.create = this.create.bind(this);
  }

  public render() {
    return (
      <div>
        <div class="field">
          <label class="label">Topic Name</label>
          <div class="control">
            <input class="input" type="text" value={this.state.name} onInput={this.setName} />
          </div>
        </div>

        <div class="field">
          <label class="label">Time to Live (seconds)</label>
          <div class="control">
            <input class="input" type="number" value={this.state.ttl} onInput={this.setTtl} />
          </div>
        </div>

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

  private setTtl(event: FormEvent<HTMLInputElement>) {
    this.setState({ ttl: str2uint(event.currentTarget.value) });
  }

  private create() {
    const body = {
      message_ttl: this.state.ttl,
    };
    const init = { method: "PUT", headers: HEADERS, body: JSON.stringify(body) };
    fetch(topicsUrl(this.state.name), init)
      .then(response => {
        if (!response.ok) {
          throw new Error(`Response was ${response.status}.`);
        }
      })
      .catch(error => {
        logError("Failed to create new topic!", error);
      });
  }
}
