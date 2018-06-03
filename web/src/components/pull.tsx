import { ChangeEvent, Component, FormEvent } from "inferno";
import { createElement } from "inferno-create-element";

import { messageFromMessagesBlob } from "../utils/data_parsers";
import { ackUrl, HEADERS, logError, pullUrl } from "../utils/util";

interface State {
  subscription: string;
  ack: boolean;
  id: string;
  time: Date | null;
  data: string;
}

const emptyMessage = {
  id: "",
  time: null,
  data: "",
};

const emptyState: State = {
  subscription: "",
  ack: false,
  ...emptyMessage,
};

const maxMessages = 1;

export class Pull extends Component<null, State> {
  public state = emptyState;

  constructor(props: null, context: null) {
    super(props, context);

    this.setSubscription = this.setSubscription.bind(this);
    this.setAck = this.setAck.bind(this);
    this.pull = this.pull.bind(this);
    this.ackLast = this.ackLast.bind(this);
  }

  public render() {
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
          <div class="level-right">
            <span class="has-text-weight-bold">Time:&nbsp;</span>
            {this.state.time === null ? "<no message>" : this.state.time.toISOString()}
          </div>
        </div>

        <div class="field">
          <div class="control">
            {createElement(
              "textarea",
              { class: "textarea", placeholder: "Message contents...", rows: 6, readonly: true },
              this.state.data,
            )}
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
                // checked={this.state.ack}
                // onChange={this.setAck}
              />
              &nbsp;Format
            </label>
          </div>

          <div class="control">
            <input
              class="button is-primary"
              type="button"
              value="Ack Last"
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
        throw new Error(`Response was ${response.status}.`);
      })
      .then(json => {
        const message = messageFromMessagesBlob(json);
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
        logError("Failed to pull message!", error);
      });
  }

  private ackLast() {
    const body = {
      message_ids: [this.state.id],
    };
    const init = { method: "POST", headers: HEADERS, body: JSON.stringify(body) };
    fetch(ackUrl(this.state.subscription), init)
      .then(response => {
        if (!response.ok) {
          throw new Error(`Response was ${response.status}.`);
        }
      })
      .catch(error => {
        logError("Failed to ack message!", error);
      });
  }
}
