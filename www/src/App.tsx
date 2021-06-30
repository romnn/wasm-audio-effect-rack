import React from "react";
import BPMDetection from "./nodes/bpm-detection";
import logo from "./logo.svg";
import "./App.css";

type AppState = {
  // wasm?: typeof import("wasm-audio-effect-rack");
};

type AppProps = {};

export default class App extends React.Component<AppProps, AppState> {
  constructor(props: AppProps) {
    super(props);
    this.state = {
      wasm: undefined,
    };
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
      // { sampleRate: ctx.sampleRate },
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
      </div>
    );
  };
}
