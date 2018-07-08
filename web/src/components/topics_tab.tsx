import classNames from "classnames";

import { CourierState } from "../utils/data_parsers";
import { NotificationType } from "../utils/types";
import { NewTopic } from "./new_topic";
import { Publish } from "./publish";
import { TopicsTable } from "./topics_table";

interface Props {
  visible: boolean;
  courierState: CourierState;
  setNotification: (type: NotificationType, message: string) => void;
  setDeleteConfirmation: (message: string, action: () => void) => void;
}

export function TopicsTab(props: Props) {
  const c = props.courierState;
  return (
    <div class={classNames({ "is-hidden": !props.visible })}>
      <section class="section">
        <div class="container">
          <div class="columns">
            <div class="column">
              <NewTopic setNotification={props.setNotification} />
            </div>
            <div class="column">
              <Publish setNotification={props.setNotification} />
            </div>
          </div>
        </div>
      </section>

      <section class="section">
        <div class="container">
          <TopicsTable
            topics={Array.from(props.courierState.topicMap.values())}
            topic2subscriptions={props.courierState.topic2subscriptions}
            setNotification={props.setNotification}
            setDeleteConfirmation={props.setDeleteConfirmation}
          />
        </div>
      </section>
    </div>
  );
}
