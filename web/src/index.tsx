import "./index.scss";

import { Component, render, version } from "inferno";
import { AllSingleStats } from "./components/all_single_stats";
import { SubscriptionsTab } from "./components/subscriptions_tab";
import { Tabs, TopNavbar } from "./components/top_navbar";
import { TopicsTab } from "./components/topics_tab";
import { CourierState, courierStateFromAny, newCourierState } from "./utils/data_parsers";
import { logError, metricsUrl } from "./utils/util";

interface UiState {
  interval: number;
  displayStats: boolean;
  tab: Tabs;
}

interface State {
  uiState: UiState;
  courierState: CourierState;
  previousCourierState: CourierState;
}

class App extends Component<null, State> {
  public state = {
    uiState: {
      interval: 2500,
      displayStats: false,
      tab: Tabs.Subscriptions,
    },
    courierState: newCourierState(),
    previousCourierState: newCourierState(),
  };

  constructor(props: null, context: null) {
    super(props, context);

    this.updateCourierState = this.updateCourierState.bind(this);
    this.showStats = this.showStats.bind(this);
    this.hideStats = this.hideStats.bind(this);
    this.toggleStats = this.toggleStats.bind(this);
    this.showTopics = this.showTopics.bind(this);
    this.showSubscriptions = this.showSubscriptions.bind(this);
    this.showDocs = this.showDocs.bind(this);

    this.updateCourierState();
  }

  public render() {
    const c = this.state.courierState;
    const ui = this.state.uiState;
    return (
      <div>
        <TopNavbar
          displayStats={ui.displayStats}
          tab={ui.tab}
          handleStats={this.toggleStats}
          handleTopics={this.showTopics}
          handleSubscriptions={this.showSubscriptions}
          handleDocs={this.showDocs}
        />

        {ui.displayStats ? <AllSingleStats courierState={c} /> : null}

        {ui.tab === Tabs.Topics ? <TopicsTab courierState={c} /> : null}

        {ui.tab === Tabs.Subscriptions ? <SubscriptionsTab courierState={c} /> : null}
      </div>
    );
  }

  private updateCourierState() {
    const helper = () => {
      setTimeout(this.updateCourierState, this.state.uiState.interval);
    };
    fetch(metricsUrl())
      .then(response => {
        if (response.ok) {
          return response.json();
        }
        throw new Error(`Response was ${response.status}.`);
      })
      .then(json => {
        this.setState(previousState => {
          const previousCourierState = previousState.courierState;
          const courierState = courierStateFromAny(json, previousCourierState);
          return {
            courierState,
            previousCourierState,
          };
        }, helper);
      })
      .catch(error => {
        this.setState(previousState => {
          const courierState = newCourierState();
          return {
            courierState,
            previousCourierState: newCourierState,
          };
        }, helper);
        logError("Failed to fetch courier state!", error);
      });
  }

  private showStats() {
    const ui = this.state.uiState;
    this.setState({ uiState: { ...ui, displayStats: true } });
  }

  private hideStats() {
    const ui = this.state.uiState;
    this.setState({ uiState: { ...ui, displayStats: false } });
  }

  private toggleStats() {
    const ui = this.state.uiState;
    this.setState({ uiState: { ...ui, displayStats: !ui.displayStats } });
  }

  private showTopics() {
    const ui = this.state.uiState;
    this.setState({ uiState: { ...ui, tab: Tabs.Topics } });
  }

  private showSubscriptions() {
    const ui = this.state.uiState;
    this.setState({ uiState: { ...ui, tab: Tabs.Subscriptions } });
  }

  private showDocs() {
    const ui = this.state.uiState;
    this.setState({ uiState: { ...ui, tab: Tabs.Docs } });
  }
}

// Main
document.addEventListener("DOMContentLoaded", () => {
  // Render the app
  render(<App />, document.getElementById("root"));

  // Add navbar burger toggle
  // Get all "navbar-burger" elements
  const $navbarBurgers = Array.prototype.slice.call(document.querySelectorAll(".navbar-burger"), 0);
  if ($navbarBurgers.length > 0) {
    // Add a click event on each of them
    $navbarBurgers.forEach($el => {
      $el.addEventListener("click", () => {
        // Get the target from the "data-target" attribute
        const target = $el.dataset.target;
        const $target = document.getElementById(target);

        // Toggle the class on both the "navbar-burger" and the "navbar-menu"
        $el.classList.toggle("is-active");
        $target.classList.toggle("is-active");
      });
    });
  }
});
