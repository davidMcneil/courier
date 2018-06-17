import { Component } from "inferno";

import { CourierState } from "../utils/data_parsers";
import { NotificationType } from "../utils/types";

interface Props {
  type: NotificationType;
  message: string;
  clear: () => void;
}

export function Notification(props: Props) {
  const type = props.type === NotificationType.Failure ? "is-danger" : "is-primary";
  const prefix = props.type === NotificationType.Failure ? "Failure:" : "Success:";
  return (
    <div class={props.message === "" ? "is-hidden" : ""}>
      <section class="section">
        <div class="container">
          <div class={`notification ${type}`}>
            <button class="delete" onClick={props.clear} />
            <b>{prefix}</b>&nbsp;
            {props.message}
          </div>
        </div>
      </section>
    </div>
  );
}
