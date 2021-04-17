import "./index.css";
import logo from "./ploog.svg";
import { h, render } from "preact";
import { useState } from "preact/hooks";

const App = () => {
  const [state, setState] = useState(0);
  return (
    <div>
      <div className="logo">
        <span dangerouslySetInnerHTML={{ __html: logo }} />
      </div>
      <br />
      <button onClick={() => setState((x) => x + 1)}>
        Clicked {state} times{" "}
      </button>
    </div>
  );
};

render(<App />, document.body);
