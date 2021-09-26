import {AudioAnalysisResult} from '@disco/core/audio/analysis';
import {Color, Font, FractalTunnelOrbitConstraints as FTOrbitConstraints, FractalTunnelParameters as FTParams, FractalTunnelStartConfig as FTStartConfig, RGBColor,} from '@disco/core/grpc';
import {gaussianFilter1d} from '@disco/core/utils/filters';
import {bin, clip, map, rgb, sum} from '@disco/core/utils/functions';
import {gaussianProb, interpolate, log, mod, softmax} from '@disco/core/utils/math';
import {indexedSort} from '@disco/core/utils/sort';
import {ease, EaseDirection, EaseStyle} from '@disco/core/utils/transitions';
import {RelativeVolume} from '@disco/core/utils/volume';
import clone from 'just-clone';
import * as THREE from 'three';

import {BaseParameterizer, Parameterizer, Parameters} from '../parameterizer';

export {FractalTunnelParameters as FTParams, FractalTunnelStartConfig as FTStartConfig,} from '@disco/core/grpc';

export const defaultConfig = (() => {
  const c = new FTStartConfig();
  c.setNumPointsPerSubset(2000);
  c.setNumSubsets(10);
  c.setNumLevels(4);
  c.setLevelDepth(800);
  c.setScaleFactor(1500);
  c.setSpriteSize((3 * window.innerWidth) / c.getScaleFactor());
  c.setCameraBound(400);
  return c;
})();

export const defaultParams = (() => {
  const c = new FTParams();
  c.setSpeed(0.1);
  c.setSpeed(5);
  c.setRotationSpeed(0.001);
  c.setLevelHueList(new Array(defaultConfig.getNumSubsets())
                        .fill(0)
                        .map((v) => 255 * Math.random()));
  c.setLevelBrightnessList(new Array(defaultConfig.getNumSubsets()).fill(0.8));
  c.setLevelSaturationList(new Array(defaultConfig.getNumSubsets()).fill(0.5));
  let constraints = new FTOrbitConstraints();
  constraints.setAMin(-30);
  constraints.setAMax(30);
  constraints.setBMin(0.2);
  constraints.setBMax(1.8);
  constraints.setCMin(5);
  constraints.setCMax(17);
  constraints.setDMin(0);
  constraints.setDMax(10);
  constraints.setEMin(0);
  constraints.setEMax(12);
  c.setOrbitConstraints(constraints);
  return c;
})();

class FTTemp {
  params = defaultParams;
  targetState = defaultParams;
}

export {FTTemp as FractalTunnelTemporary};

export class FractalTunnelParameterizerConfig {
  // fadeoutEaseDirection: EaseDirection|undefined = EaseDirection.OUT;
  // fadeoutEaseStyle: EaseStyle|undefined = EaseStyle.QUART;
}

export class FractalTunnelParameterizer extends BaseParameterizer<
    FTStartConfig, AudioAnalysisResult, FTTemp, FTParams> implements
    Parameterizer<FTStartConfig, AudioAnalysisResult, FTTemp, FTParams> {
  public config = new FractalTunnelParameterizerConfig();
  protected relativeVolume = new RelativeVolume({windowSize: 100});

  constructor() {
    super();
  }

  public parameterize =
      (frame: number, config: FTStartConfig, previous: FTParams[],
       current: FTParams|undefined, temp: FTTemp|undefined,
       input: AudioAnalysisResult|null): [FTParams, FTTemp] => {
        const inParams = current ?? defaultParams;
        const inTemp = temp ?? new FTTemp();

        const outParams = defaultParams;
        const outTemp = new FTTemp();

        const spectral = input?.getSpectral();
        const bpm = input?.getBpm();

        let baseHue = 0;
        if (spectral) {
          // const volume = map(spectral.getVolume(), 0, 1, 0.9, 1);
          // let volume = this.relativeVolume.mean +
          //              0.9 * (this.relativeVolume.mean -
          //                     this.relativeVolume.update(spectral.getVolume()));
          baseHue = mod(frame, 60 * 60) / 60 * 60;
          // console.log(baseHue);
          let volume = spectral.getVolume();
          if (volume > 0.05) {
            this.relativeVolume.update(spectral.getVolume());
            // let volume = this.relativeVolume.update(spectral.getVolume());
            volume = this.relativeVolume.scale(
                spectral.getVolume() *
                Math.pow(
                    1 +
                        Math.max(
                            0, spectral.getVolume() - this.relativeVolume.mean),
                    3));

            volume = clip(volume, 0.4, 1.0);
          }

          const melBands = spectral.getMelBandsList();

          let numSubsets = config.getNumSubsets();
          let spectrum = interpolate(melBands, config.getNumSubsets());
          spectrum = softmax(spectrum);  //.map((s) => s);
          console.assert(
              spectrum.length === outParams.getLevelHueList().length);

          outParams.setLevelHueList(spectrum);
          outParams.setLevelHueList(
              new Array(defaultConfig.getNumSubsets())
                  .fill(0)
                  .map(
                      (_, i) =>
                          255 * (baseHue + 0.02 * i + 0.4 * spectrum[i])));
          outParams.setLevelBrightnessList(
              new Array(defaultConfig.getNumSubsets()).fill(0).map((_, i) => {
                // return 0.5;
                return map(volume * Math.pow(0.999, i), 0.4, 1, 0.3, 0.6);
              }));
          // console.log(volume * Math.pow(0.5, 1));
          // console.log(
          //     "mapped", map(volume * Math.pow(0.5, 1), 0.4, 1, 0.5, 1.0));
          outParams.setLevelSaturationList(
              new Array(defaultConfig.getNumSubsets()).fill(0).map((_, i) => {
                let test = map(
                    volume * Math.pow(0.999, numSubsets - i), 0.0, 1, 0.8, 1.0);
                // console.log(test);
                return test;
              }));
        }

        outTemp.params = outParams.cloneMessage();

        // apply final parameter changes that are not temporary saved
        return [outParams, outTemp];
      }
}
