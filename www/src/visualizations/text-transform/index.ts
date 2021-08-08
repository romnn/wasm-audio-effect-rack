import {
  AudioAnalysisResult
} from "../../generated/proto/audio/analysis/analysis_pb";
import {Parameterizer} from "../parameterizer";
import {
  BaseVisualizationController,
  InternalVisualizationController,
} from "../visualization";

import {TTFParameterizer, TTFParams, TTFStartConfig} from "./parameterizer";
import {default as Visualization} from "./visualization";

export {
  TTFParameterizer as Parameterizer,
  TTFParams as Parameters
} from "./parameterizer";
export {default as Visualization} from "./visualization";

export type ParameterizerType =
    Parameterizer<TTFStartConfig, AudioAnalysisResult, any, TTFParams>;

export default class TextTransformVisualizaton extends
    BaseVisualizationController<TTFStartConfig, AudioAnalysisResult, any,
                                TTFParams, Visualization> implements
        InternalVisualizationController<TTFStartConfig, AudioAnalysisResult,
                                        TTFParams> {
  public visualization = new Visualization();
  public parameterizers = [ TTFParameterizer ];
  public parameterizer: ParameterizerType|null = new this.parameterizers[0]();
}
