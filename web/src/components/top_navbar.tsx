import { Component } from "inferno";

export enum Tabs {
  Topics,
  Subscriptions,
  Docs,
}

interface Props {
  displayStats: boolean;
  tab: Tabs;
  handleStats: () => void;
  handleTopics: () => void;
  handleSubscriptions: () => void;
  handleDocs: () => void;
}

export function TopNavbar(props: Props) {
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
              Stats {props.displayStats ? <span class="down-arrow" /> : <span class="up-arrow" />}
            </a>
            <div class="navbar-item tabs is-boxed">
              <ul>
                <li class={props.tab === Tabs.Topics ? "is-active" : ""}>
                  <a onClick={props.handleTopics}>Topics</a>
                </li>
                <li class={props.tab === Tabs.Subscriptions ? "is-active" : ""}>
                  <a onClick={props.handleSubscriptions}>Subscriptions</a>
                </li>
                <li class={props.tab === Tabs.Docs ? "is-active" : ""}>
                  <a onClick={props.handleDocs}>Docs</a>
                </li>
              </ul>
            </div>
          </div>

          <div class="navbar-end" />
        </div>
      </div>
    </nav>
  );
}
