import { Component, FormEvent } from "inferno";

import { HEADERS, logError, publishUrl } from "../utils/util";

interface State {
  topic: string;
  message: string;
}

const emptyState: State = {
  topic: "",
  message: "",
};

export class Publish extends Component<null, State> {
  public state = emptyState;

  constructor(props: null, context: null) {
    super(props, context);

    this.setTopic = this.setTopic.bind(this);
    this.setMessage = this.setMessage.bind(this);
    this.publish = this.publish.bind(this);
  }

  public render() {
    return (
      <div>
        <div class="field has-addons">
          <div class="control is-expanded">
            <input
              class="input"
              type="text"
              placeholder="Topic name..."
              value={this.state.topic}
              onInput={this.setTopic}
            />
          </div>
          <div class="control">
            <input class="button is-primary" type="button" value="Publish" onClick={this.publish} />
          </div>
        </div>

        <div class="field">
          <div class="control">
            <textarea
              class="textarea"
              placeholder="Message to publish..."
              rows={5}
              onInput={this.setMessage}
            />
          </div>
        </div>
      </div>
    );
  }

  private setTopic(event: FormEvent<HTMLInputElement>) {
    this.setState({ topic: event.currentTarget.value });
  }

  private setMessage(event: FormEvent<HTMLTextAreaElement>) {
    this.setState({ message: event.currentTarget.value });
  }

  private publish() {
    const body = {
      messages: [{ data: this.state.message }],
    };
    const init = { method: "POST", headers: HEADERS, body: JSON.stringify(body) };
    fetch(publishUrl(this.state.topic), init)
      .then(response => {
        if (!response.ok) {
          throw new Error(`Response was ${response.status}.`);
        }
      })
      .catch(error => {
        logError("Failed to publish message!", error);
      });
  }
}