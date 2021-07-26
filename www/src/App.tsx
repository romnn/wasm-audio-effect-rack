import React from "react";
import Fractal from "./Fractal1";
import TextTransform from "./TextTransform";
import "./App.css";
import {
  HashRouter as Router,
  Switch,
  Route,
  Link,
  RouteComponentProps,
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
          <Route exact path="/fractal" component={Fractal} />
          <Route exact path="/texttransform" component={TextTransform} />
        </Router>
      </div>
    );
  };
}
