import React from "react";
import BPMDetection from "./nodes/bpm-detection";
import Fractal from "./Fractal1";
import "./App.css";

type AppState = {};
type AppProps = {};

export default class App extends React.Component<AppProps, AppState> {
  constructor(props: AppProps) {
    super(props);
    this.state = {};
  }

  play = () => {
    const ctx = new window.AudioContext();
    const audio = new Audio("mars_venus.mp3");
    audio.autoplay = true;
    audio.loop = true;
    // audio.muted = true;
    const source = ctx.createMediaElementSource(audio);
    const bpmDetector = new BPMDetection(
      ctx,
      "bpm-detection-node-processor",
      undefined,
      {
        onInitialized: (inst: BPMDetection) => {
          // bpmDetector.onBPMChanged = (bpm: number) => {
          //   console.log("bpm changed to ", bpm);
          // };
          source.connect(bpmDetector.workletHandle!);
          // bpmDetector.workletHandle!.connect(ctx.destination);
          source.connect(ctx.destination);
        },
      }
    );
  };

  componentDidMount = () => {};

  render = () => {
    return (
      <div className="App">
        <p onClick={this.play}>Welcome</p>
        <Fractal></Fractal>
      </div>
    );
  };
}
