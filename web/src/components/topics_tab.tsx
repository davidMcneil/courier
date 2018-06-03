import { Component } from "inferno";

import { CourierState } from "../utils/data_parsers";
import { NewTopic } from "./new_topic";
import { Publish } from "./publish";
import { Topic } from "./topic";

interface Props {
  courierState: CourierState;
}

export function TopicsTab(props: Props) {
  const c = props.courierState;
  return (
    <div>
      <section class="section">
        <div class="container">
          <div class="columns">
            <div class="column">
              <NewTopic />
            </div>
            <div class="column">
              <Publish />
            </div>
          </div>
        </div>
      </section>

      <section class="section">
        <div class="container">
          <table class="table is-bordered is-striped is-narrow is-fullwidth">
            <thead>
              <tr>
                <th>Topic</th>
                <th>Messages</th>
                <th>Messages Interval</th>
                <th>Messages All Time</th>
                <th>Message TTL (s)</th>
              </tr>
            </thead>
            <tbody>{Array.from(c.topics.values()).map(t => <Topic metrics={t} />)}</tbody>
          </table>
        </div>
      </section>
    </div>
  );
}
