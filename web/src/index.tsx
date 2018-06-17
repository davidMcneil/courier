import "./index.scss";

import { Component, render, version } from "inferno";
import { AllSingleStats } from "./components/all_single_stats";
import { DeleteConfirmation } from "./components/delete_confirmation";
import { Notification } from "./components/notification";
import { SubscriptionsTab } from "./components/subscriptions_tab";
import { TopNavbar } from "./components/top_navbar";
import { TopicsTab } from "./components/topics_tab";
import { CourierState, courierStateFromAny, newCourierState } from "./utils/data_parsers";
import { NotificationType, Tabs } from "./utils/types";
import { fetchError2message, metricsUrl } from "./utils/util";

interface NotificationState {
  type: NotificationType;
  message: string;
}

interface DeleteConfirmationState {
  message: string;
  action: () => void;
}

interface UiState {
  interval: number | null;
  updating: boolean;
  displayStats: boolean;
  tab: Tabs;
  notification: NotificationState;
  delete_confirmation: DeleteConfirmationState;
}

interface State {
  uiState: UiState;
  courierState: CourierState;
  previousCourierState: CourierState;
}

class App extends Component<null, State> {
  public state = {
    uiState: {
      interval: 5000,
      updating: false,
      displayStats: false,
      tab: Tabs.Topics,
      notification: {
        type: NotificationType.Success,
        message: "",
      },
      delete_confirmation: {
        message: "",
        action: () => undefined,
      },
    },
    courierState: newCourierState(),
    previousCourierState: newCourierState(),
  };
  private updateTimeout: number = 0;

  constructor(props: null, context: null) {
    super(props, context);

    this.updater = this.updater.bind(this);
    this.update = this.update.bind(this);
    this.showStats = this.showStats.bind(this);
    this.hideStats = this.hideStats.bind(this);
    this.toggleStats = this.toggleStats.bind(this);
    this.showTopics = this.showTopics.bind(this);
    this.showSubscriptions = this.showSubscriptions.bind(this);
    this.showDocs = this.showDocs.bind(this);
    this.setNotification = this.setNotification.bind(this);
    this.clearNotification = this.clearNotification.bind(this);
    this.setDeleteConfirmation = this.setDeleteConfirmation.bind(this);
    this.clearDeleteConfirmation = this.clearDeleteConfirmation.bind(this);
    this.setUpdateInterval = this.setUpdateInterval.bind(this);

    this.updater();
  }

  public render() {
    const c = this.state.courierState;
    const ui = this.state.uiState;
    return (
      <div>
        <TopNavbar
          displayStats={ui.displayStats}
          tab={ui.tab}
          interval={ui.interval}
          updating={ui.updating}
          startTime={c.startTime}
          handleStats={this.toggleStats}
          handleTopics={this.showTopics}
          handleSubscriptions={this.showSubscriptions}
          handleDocs={this.showDocs}
          update={this.update}
          setUpdateInterval={this.setUpdateInterval}
        />

        {ui.displayStats ? <AllSingleStats courierState={c} /> : null}

        <Notification
          type={ui.notification.type}
          message={ui.notification.message}
          clear={this.clearNotification}
        />

        <TopicsTab
          visible={ui.tab === Tabs.Topics}
          courierState={c}
          setNotification={this.setNotification}
          setDeleteConfirmation={this.setDeleteConfirmation}
        />

        <SubscriptionsTab
          visible={ui.tab === Tabs.Subscriptions}
          courierState={c}
          setNotification={this.setNotification}
        />

        <DeleteConfirmation
          message={ui.delete_confirmation.message}
          action={ui.delete_confirmation.action}
          clearDeleteConfirmation={this.clearDeleteConfirmation}
        />
      </div>
    );
  }

  private updater() {
    const helper = () => {
      setTimeout(
        () =>
          this.setState(previousState => ({
            uiState: { ...previousState.uiState, updating: false },
          })),
        300,
      );
      if (this.state.uiState.interval !== null) {
        this.updateTimeout = setTimeout(this.updater, this.state.uiState.interval);
      }
    };
    this.setState({ uiState: { ...this.state.uiState, updating: true } });
    fetch(metricsUrl())
      .then(response => {
        if (response.ok) {
          return response.json();
        }
        throw response;
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
        const message = fetchError2message(error);
        this.setNotification(NotificationType.Failure, `Unable to fetch metrics! (${message})`);
      });
  }

  private showStats() {
    this.setState({ uiState: { ...this.state.uiState, displayStats: true } });
  }

  private hideStats() {
    this.setState({ uiState: { ...this.state.uiState, displayStats: false } });
  }

  private toggleStats() {
    const ui = this.state.uiState;
    this.setState({ uiState: { ...ui, displayStats: !ui.displayStats } });
  }

  private showTopics() {
    this.setState({ uiState: { ...this.state.uiState, tab: Tabs.Topics } });
  }

  private showSubscriptions() {
    this.setState({ uiState: { ...this.state.uiState, tab: Tabs.Subscriptions } });
  }

  private showDocs() {
    this.setState({ uiState: { ...this.state.uiState, tab: Tabs.Docs } });
  }

  private setNotification(type: NotificationType, message: string) {
    this.setState({ uiState: { ...this.state.uiState, notification: { type, message } } });
  }

  private clearNotification() {
    this.setNotification(NotificationType.Success, "");
  }

  private setDeleteConfirmation(message: string, action: () => void) {
    this.setState({ uiState: { ...this.state.uiState, delete_confirmation: { message, action } } });
  }

  private clearDeleteConfirmation() {
    this.setDeleteConfirmation("", () => undefined);
  }

  private setUpdateInterval(interval: number | null) {
    this.setState(previousState => {
      const ui = previousState.uiState;
      return {
        uiState: { ...ui, interval },
      };
    }, this.update);
  }

  private update() {
    clearTimeout(this.updateTimeout);
    this.updater();
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
