import { Component, FormEvent } from "inferno";
import { topicFromAny } from "../utils/data_parsers";
import { NotificationType } from "../utils/types";
import { fetchError2message, HEADERS, str2uint, topicsUrl } from "../utils/util";

interface Props {
  setNotification: (type: NotificationType, message: string) => void;
}

interface State {
  name: string;
  ttl: number;
}

const emptyState: State = {
  name: "",
  ttl: 3600,
};

export class NewTopic extends Component<Props, State> {
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
          <label class="label">Message Time to Live (s)</label>
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
          throw response;
        }
        return response.json();
      })
      .then(json => {
        const topic = topicFromAny(json);
        if (topic !== null) {
          this.props.setNotification(NotificationType.Success, `Created topic '${topic.name}'.`);
        } else {
          this.props.setNotification(NotificationType.Failure, `Unable to parse topic!`);
        }
      })
      .catch(error => {
        const message = `Unable to create topic! (${fetchError2message(error)})`;
        this.props.setNotification(NotificationType.Failure, message);
      });
  }
}
