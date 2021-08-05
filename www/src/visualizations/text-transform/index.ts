import {
  AudioAnalysisResult
} from "../../generated/proto/audio/analysis/analysis_pb";
import {Parameterizer, ParameterizerClass} from "../parameterizer";
import {
  BaseVisualizationController,
  InternalVisualizationController,
  VisualizationController
} from "../visualization";

import {TTFParameterizer, TTFParams} from "./parameterizer";
import {default as Visualization} from "./visualization";

export {
  TTFParameterizer as Parameterizer,
  TTFParams as Parameters
} from "./parameterizer";
export {default as Visualization} from "./visualization";

export type ParameterizerType =
    Parameterizer<AudioAnalysisResult, any, TTFParams>;

export default class TextTransformVisualizaton extends
    BaseVisualizationController<AudioAnalysisResult, TTFParams, Visualization>
        implements InternalVisualizationController<AudioAnalysisResult,
                                                   TTFParams, Visualization> {
  public visualization = new Visualization();
  public parameterizers = [ TTFParameterizer ];
  public parameterizer: ParameterizerType|null = new this.parameterizers[0]();

  constructor() {
    super();
  }

  // getParameterizerNames = (): {idx: number; name : string}[] => {
  //   return this.parameterizers.map(
  //       (p, idx) => { return {idx, name : p.name}; });
  // };

  // parameterize = (frame: number, input: AudioAnalysisResult):
  //     void => {
  //       let parameterizer = this.currentParameterizer;
  //       if (parameterizer) {
  //         // console.log("parameterize with", frame, input);
  //         this.visualization.parameterize(frame, input, parameterizer,
  //                                         undefined)
  //       }
  //     }

  // getParameterizer =
  //     (): ParameterizerType|null => { return this.currentParameterizer; }

  // useParameterizerAtIndex = (idx: number):
  //     void => {
  //       if (0 <= idx && (this.parameterizers.length) < idx) {
  //         this.useParameterizer(this.parameterizers[idx]);
  //       }
  //     }

  // useParameterizerNamed = (name: string):
  //     void => {
  //       let parameterizer = this.parameterizers.find(p => p.name == name);
  //       if (parameterizer)
  //         this.useParameterizer(parameterizer);
  //     }

  // useParameterizer =
  //     (parameterizer:
  //          ParameterizerClass<AudioAnalysisResult, TTFTemp, TTFParams>):
  //         void => {}
}
