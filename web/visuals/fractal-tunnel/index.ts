import {AudioAnalysisResult} from "@disco/core/audio/analysis";
import {Parameterizer, ParameterizerClass} from "../parameterizer";
import {
  BaseVisualizationController,
  InternalVisualizationController,
} from "../visualization";

import {
  FractalTunnelParameterizer as FTParameterizer,
  FTParams,
  FTStartConfig
} from "./parameterizer";
import Visualization from "./visualization";

export {
  FractalTunnelParameterizer as Parameterizer,
  FTParams as Parameters
} from "./parameterizer";
export {default as Visualization} from "./visualization";

export type ParameterizerType =
    Parameterizer<FTStartConfig, AudioAnalysisResult, any, FTParams>;

export default class FractalTunnelVisualizaton extends
    BaseVisualizationController<FTStartConfig, AudioAnalysisResult, any,
                                FTParams, Visualization> implements
        InternalVisualizationController<FTStartConfig, AudioAnalysisResult,
                                        FTParams> {
  public visualization = new Visualization();
  public parameterizers = [ FTParameterizer ];
  // public parameterizers:
  //     ParameterizerClass<FTStartConfig, AudioAnalysisResult, any, FTParams>[]
  //     =
  //         [];
  // public parameterizer: ParameterizerType|null = null;
  public parameterizer: ParameterizerType|null = new this.parameterizers[0]();
}
