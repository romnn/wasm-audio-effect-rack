import React from "react";
import Viewer from "./Viewer";
import Controller from "./Controller";
import Landing from "./Landing";
import "./App.sass";
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
          <Route exact path="/" component={Landing} />
          <Route exact path="/viewer/:token?/:instance?" component={Viewer} />
          <Route exact path="/controller/:token?/:instance?" component={Controller} />
        </Router>
      </div>
    );
  };
}
