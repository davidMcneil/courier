import { Component } from "inferno";

import { CourierState } from "../utils/data_parsers";
import { NewSubscription } from "./new_subscription";
import { Pull } from "./pull";
import { Subscription } from "./subscription";

interface Props {
  courierState: CourierState;
}

export function SubscriptionsTab(props: Props) {
  const c = props.courierState;
  return (
    <div>
      <section class="section">
        <div class="container">
          <div class="columns">
            <div class="column">
              <NewSubscription />
            </div>
            <div class="column">
              <Pull />
            </div>
          </div>
        </div>
      </section>

      <section class="section">
        <div class="container">
          <table class="table is-bordered is-striped is-narrow is-fullwidth">
            <thead>
              <tr>
                <th>Subscription</th>
                <th>Topic</th>
                <th>Ack Deadline</th>
                <th>Message Index</th>
                <th>Num Pulled All Time</th>
                <th>Num Acked All Time</th>
                <th>Num Pulled Interval</th>
                <th>Num Acked Interval</th>
                <th>Pending</th>
                <th>Processed</th>
                <th>Orphaned</th>
              </tr>
            </thead>
            <tbody>
              {Array.from(c.subscriptions.values()).map(s => <Subscription metrics={s} />)}
            </tbody>
          </table>
        </div>
      </section>
    </div>
  );
}
