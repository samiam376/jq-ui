import { createResource, createSignal } from "solid-js";
import "./App.css";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [jsonInput, setJsonInput] = createSignal("");
  const [query, setQuery] = createSignal("");
  const invokeInput = () => ({ input: jsonInput(), query: query() });

  const [data] = createResource(invokeInput, async (input) => {
    if (input.query === "") {
      return input.input;
    }
    try {
      const result = await invoke("jq", {
        json: input.input,
        query: input.query,
      });

      return result;
    } catch (e) {
      return JSON.stringify(e);
    }
  });

  return (
    <div class="container">
      <div class="json-boxes">
        <textarea
          spellcheck={false}
          class="json-input"
          value={jsonInput()}
          onInput={(e) => setJsonInput(e.target.value)}
          placeholder="Enter your JSON-like message here..."
        />
        <textarea class="json-input" value={data() as string} readOnly />
      </div>
      <div class="query-bar">
        <input
          type="text"
          class="query-input"
          value={query()}
          onInput={(e) => setQuery(e.target.value)}
          placeholder="Enter your query here..."
        />
      </div>
    </div>
  );
}

export default App;
