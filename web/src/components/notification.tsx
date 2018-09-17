import classNames from "classnames";
import { NotificationType } from "../utils/util";

interface Props {
  type: NotificationType;
  message: string;
  clear: () => void;
}

export function Notification(props: Props) {
  const fail = props.type === NotificationType.Failure;
  const prefix = fail ? "Failure:" : "Success:";
  return (
    <div class={classNames({ "is-hidden": props.message === "" })}>
      <section class="section">
        <div class="container">
          <div class={classNames("notification", { "is-danger": fail })}>
            <button class="delete" onClick={props.clear} />
            <b>{prefix}</b>
            &nbsp;
            {props.message}
          </div>
        </div>
      </section>
    </div>
  );
}
