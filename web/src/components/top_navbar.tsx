import classNames from "classnames";

import { Tabs } from "../utils/types";
import { numberAsTimeStr, str2number } from "../utils/util";

interface Props {
  displayStats: boolean;
  tab: Tabs;
  interval: number | null;
  updating: boolean;
  startTime: Date;
  handleStats: () => void;
  handleTopics: () => void;
  handleSubscriptions: () => void;
  handleDocs: () => void;
  update: () => void;
  setUpdateInterval: (interval: number | null) => void;
}

export function TopNavbar(props: Props) {
  const uptime = (new Date().getTime() - props.startTime.getTime()) / 1000;
  return (
    <nav class="navbar is-fixed-top">
      <div class="container">
        <div class="navbar-brand">
          <a class="navbar-item is-size-3" href="https://github.com/davidMcneil/courier">
            Courier
          </a>
          <div class="navbar-burger" data-target="topNavbar">
            <span />
            <span />
            <span />
          </div>
        </div>

        <div id="topNavbar" class="navbar-menu">
          <div class="navbar-start">
            <a class="navbar-item" onClick={props.handleStats}>
              Stats {props.displayStats ? <span class="arrow-down" /> : <span class="arrow-up" />}
            </a>
            <div class="navbar-item tabs is-boxed">
              <ul>
                <li class={classNames({ "is-active": props.tab === Tabs.Topics })}>
                  <a onClick={props.handleTopics}>Topics</a>
                </li>
                <li class={classNames({ "is-active": props.tab === Tabs.Subscriptions })}>
                  <a onClick={props.handleSubscriptions}>Subscriptions</a>
                </li>
                {/* For now do not show the docs tab. */}
                {/* <li class={classNames({ "is-active": props.tab === Tabs.Docs })}>
                  <a onClick={props.handleDocs}>Docs</a>
                </li> */}
              </ul>
            </div>
          </div>
          <div class="navbar-end">
            <div class="navbar-item">
              <b>Uptime:&nbsp;</b> {numberAsTimeStr(uptime)}
            </div>
            <div class="navbar-item is-paddingless">
              <input
                class="button is-primary is-small"
                type="button"
                value="Update"
                onClick={props.update}
              />
            </div>
            <div class="navbar-item">
              <div class={classNames("select", "is-small", { "is-loading": props.updating })}>
                <select
                  value={String(props.interval)}
                  onChange={event => props.setUpdateInterval(str2number(event.currentTarget.value))}
                >
                  <option value={"null"}>Off</option>
                  <option value={"1000"}>1s</option>
                  <option value={"5000"}>5s</option>
                  <option value={"10000"}>10s</option>
                  <option value={"30000"}>30s</option>
                  <option value={"60000"}>1m</option>
                </select>
              </div>
            </div>
          </div>
        </div>
      </div>
    </nav>
  );
}
