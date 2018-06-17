import { Component } from "inferno";

import { CourierState } from "../utils/data_parsers";
import { NotificationType } from "../utils/types";
import { NewTopic } from "./new_topic";
import { Publish } from "./publish";
import { Topic } from "./topic";

interface Props {
  visible: boolean;
  courierState: CourierState;
  setNotification: (type: NotificationType, message: string) => void;
  setDeleteConfirmation: (message: string, action: () => void) => void;
}

export function TopicsTab(props: Props) {
  const c = props.courierState;
  return (
    <div class={props.visible ? "" : "is-hidden"}>
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
          <table
            class="table table-with-bottom-border is-hoverable is-narrow is-fullwidth"
            $HasKeyedChildren
          >
            {[
              <thead key="header">
                <th />
                <th>Name</th>
                <th>Messages</th>
                <th>Subscriptions</th>
                <th>Processed</th>
                <th>Age</th>
                <th />
              </thead>,
              ...Array.from(c.topics.values()).map(t => (
                <Topic
                  metrics={t}
                  setNotification={props.setNotification}
                  setDeleteConfirmation={props.setDeleteConfirmation}
                />
              )),
            ]}
          </table>
        </div>
      </section>
    </div>
  );
}
