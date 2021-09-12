import {AudioAnalysisResult} from "@disco/core/audio/analysis";
import {Parameterizer} from "../parameterizer";
import {
  BaseVisualizationController,
  InternalVisualizationController,
} from "../visualization";

import {
  TextTransformParameterizer as TTFParameterizer,
  // TextTransformParameterizer as TTFParameterizer,
  TTFParams,
  TTFStartConfig
} from "./parameterizer";
import Visualization from "./visualization";

export {
  // TTFParams as Parameterizer,
  TextTransformParameterizer as Parameterizer,
  TTFParams as Parameters
  // TextTransformParameters as Parameters
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
