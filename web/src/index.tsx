import { render, version, Component } from "inferno";
import { Incrementer } from "./components/incrementer";

class MyComponent extends Component<undefined, undefined> {
  private tsxVersion: number;

  constructor(props, context) {
    super(props, context);

    this.tsxVersion = 2.71;
  }

  public render() {
    return (
      <div>
        <h1>{`Welcome to Inferno ${version} TSX ${this.tsxVersion}`}</h1>
        <Incrementer name={"Crazy button"} />
      </div>
    );
  }
}

render(<MyComponent />, document.getElementById("root"));
