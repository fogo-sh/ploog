import "./index.css";
import { h, render } from "preact";
import { useState } from "preact/hooks";
import Logo from "./Logo.jsx";

const App = () => {
  const [state, setState] = useState(0);
  return (
    <div>
      <header>
        <Logo className="logo" />
      </header>
      <div className="editor-container">
        <div className="nav">
          <button onClick={() => setState((x) => x + 1)}>Posts</button>
          <button className="notActive" onClick={() => setState((x) => x + 1)}>
            Plugins
          </button>
        </div>
        <div className="editorNav">
          <button onClick={() => setState((x) => x + 1)}>
            Clicked {state} times
          </button>
        </div>
        <div className="editor">
          <button onClick={() => setState((x) => x + 1)}>
            Clicked {state} times
          </button>
        </div>
        <div className="publish">
          <button onClick={() => setState((x) => x + 1)}>
            Clicked {state} times
          </button>
        </div>
      </div>
    </div>
  );
};

render(<App />, document.body);
