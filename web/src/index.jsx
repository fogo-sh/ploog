import "./index.css";
import { h, render } from "preact";
import { useState } from "preact/hooks";
import { Editor, Logo, NavSidebar, PublishControls } from "./components";

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
          <Editor />
        </div>
        <div className="publish">
          <div className="controlCard">
            <div className="title">Post - ...</div>
            <div className="controls">
              <button
                className="notActive"
                onClick={() => setState((x) => x + 1)}
              >
                Save Draft
              </button>
              <button className="notActive">Preview</button>
            </div>
            <div className="statuses">
              <div className="title">☀️ Status: ...</div>
              <div className="title">🏙️ Visibility - ...</div>
              <div className="title">📅 Publish - ...</div>
            </div>
          </div>
          <div className="controlCard publishButton">
            <button>Publish</button>
          </div>
        </div>
      </div>
    </div>
  );
};

render(<App />, document.body);
