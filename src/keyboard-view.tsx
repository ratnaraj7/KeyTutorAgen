import React from "react";
import ReactDOM from "react-dom/client";
import KeyboardApp from "./KeyboardApp";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <KeyboardApp />
  </React.StrictMode>,
);
