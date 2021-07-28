import React from "react";
import Viewer from "./Viewer";
import Controller from "./Controller";
import "./App.css";
import {
  HashRouter as Router,
  Route,
} from "react-router-dom";

type AppState = {};
type AppProps = {};

export default class App extends React.Component<AppProps, AppState> {
  constructor(props: AppProps) {
    super(props);
    this.state = {};
  }

  componentDidMount = () => {};

  render = () => {
    return (
      <div className="App">
        <Router>
          <Route exact path="/" component={Viewer} />
          <Route exact path="/viewer/:token?" component={Viewer} />
          <Route exact path="/controller/:token?" component={Controller} />
        </Router>
      </div>
    );
  };
}
