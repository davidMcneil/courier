import { ChangeEvent, Component, FormEvent } from "inferno";

import { messageFromMessagesJson } from "../utils/data_parsers";
import { NotificationType } from "../utils/types";
import { ackUrl, fetchError2message, HEADERS, isArray, isJson, pullUrl } from "../utils/util";

interface Props {
  setNotification: (type: NotificationType, message: string) => void;
}

interface State {
  subscription: string;
  ack: boolean;
  format: boolean;
  id: string;
  time: Date | null;
  tries: number | null;
  data: string;
}

const emptyMessage = {
  id: "",
  time: null,
  tries: null,
  data: "",
};

const emptyState: State = {
  subscription: "",
  ack: false,
  format: false,
  ...emptyMessage,
};

const maxMessages = 1;

export class Pull extends Component<Props, State> {
  public state = emptyState;

  constructor(props: null, context: null) {
    super(props, context);

    this.setSubscription = this.setSubscription.bind(this);
    this.setAck = this.setAck.bind(this);
    this.setFormat = this.setFormat.bind(this);
    this.pull = this.pull.bind(this);
    this.ackLast = this.ackLast.bind(this);
  }

  public render() {
    let displayData = this.state.data;
    if (this.state.format) {
      const [wasJson, json] = isJson(displayData);
      if (wasJson) {
        displayData = JSON.stringify(json, null, 4);
      }
    }
    return (
      <div>
        <div class="field has-addons">
          <div class="control is-expanded">
            <input
              class="input"
              type="text"
              placeholder="Subscription name..."
              value={this.state.subscription}
              onInput={this.setSubscription}
            />
          </div>
          <div class="control">
            <input class="button is-primary" type="button" value="Pull" onClick={this.pull} />
          </div>
        </div>

        <div class="level is-marginless">
          <div class="level-left">
            <span class="has-text-weight-bold">ID:&nbsp;</span>{" "}
            {this.state.id === "" ? "<no message>" : this.state.id}
          </div>
        </div>
        <div class="level is-marginless">
          <div class="level-left">
            <span class="has-text-weight-bold">Tries:&nbsp;</span>{" "}
            {this.state.tries === null ? "<no message>" : this.state.tries}
          </div>
          <div class="level-right">
            <span class="has-text-weight-bold">Time:&nbsp;</span>
            {this.state.time === null ? "<no message>" : this.state.time.toISOString()}
          </div>
        </div>

        <div class="field">
          <div class="control">
            <textarea
              class="textarea"
              placeholder="Message contents..."
              rows={9}
              readOnly={true}
              value={displayData}
            />
          </div>
        </div>

        <div class="field is-grouped is-grouped-right">
          <div class="control">
            <label class="checkbox">
              <input
                class="checkbox"
                type="checkbox"
                checked={this.state.ack}
                onChange={this.setAck}
              />
              &nbsp;Ack Immediately
            </label>
          </div>

          <div class="control">
            <label class="checkbox">
              <input
                class="checkbox"
                type="checkbox"
                checked={this.state.format}
                onChange={this.setFormat}
              />
              &nbsp;Format
            </label>
          </div>

          <div class="control">
            <input
              class="button is-primary"
              type="button"
              value="Ack Last"
              disabled={this.state.id === ""}
              onClick={this.ackLast}
            />
          </div>
        </div>
      </div>
    );
  }

  private setSubscription(event: FormEvent<HTMLInputElement>) {
    this.setState({ subscription: event.currentTarget.value });
  }

  private setAck(event: ChangeEvent<HTMLInputElement>) {
    this.setState({ ack: event.currentTarget.checked });
  }

  private setFormat(event: ChangeEvent<HTMLInputElement>) {
    this.setState({ format: event.currentTarget.checked });
  }

  private pull() {
    const body = {
      max_messages: maxMessages,
    };
    const init = { method: "POST", headers: HEADERS, body: JSON.stringify(body) };
    fetch(pullUrl(this.state.subscription), init)
      .then(response => {
        if (response.ok) {
          return response.json();
        }
        throw response;
      })
      .then(json => {
        const message = messageFromMessagesJson(json);
        if (message !== null) {
          this.setState({ ...message });
          if (this.state.ack) {
            this.ackLast();
          }
        } else {
          this.setState({ ...emptyMessage });
        }
      })
      .catch(error => {
        const message = `Unable to pull message! (${fetchError2message(error)})`;
        this.props.setNotification(NotificationType.Failure, message);
      });
  }

  private ackLast() {
    const id = this.state.id;
    const body = {
      message_ids: [id],
    };
    const init = { method: "POST", headers: HEADERS, body: JSON.stringify(body) };
    fetch(ackUrl(this.state.subscription), init)
      .then(response => {
        if (response.ok) {
          return response.json();
        }
        throw response;
      })
      .then(json => {
        if (isArray(json.message_ids) && json.message_ids.length === 1) {
          this.props.setNotification(
            NotificationType.Success,
            `Acked message '${json.message_ids[0]}'.`,
          );
        } else {
          throw new Error(`could not parse '${json}' as message ids`);
        }
      })
      .catch(error => {
        const message = `Unable to ack message! (${fetchError2message(error)})`;
        this.props.setNotification(NotificationType.Failure, message);
      });
  }
}
