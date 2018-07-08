import { classNames } from "classnames";

interface Props {
  message: string;
  action: () => void;
  clearDeleteConfirmation: () => void;
}

export function DeleteConfirmation(props: Props) {
  return (
    <div class={classNames("modal", { "is-active": props.message !== "" })}>
      <div class="modal-background" />
      <div class="modal-content">
        <div class="box">
          <p>{props.message}</p>
          <div class="field is-grouped is-grouped-right">
            <div class="control">
              <input
                type="button"
                class="button is-danger"
                value="Delete"
                onClick={() => {
                  props.action();
                  props.clearDeleteConfirmation();
                }}
              />
            </div>
            <div class="control">
              <input
                type="button"
                class="button"
                value={"Cancel"}
                onClick={props.clearDeleteConfirmation}
              />
            </div>
          </div>
        </div>
      </div>
      <button
        class="modal-close is-large"
        aria-label="close"
        onClick={props.clearDeleteConfirmation}
      />
    </div>
  );
}
