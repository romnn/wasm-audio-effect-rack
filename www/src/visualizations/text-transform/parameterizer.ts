import clone from 'just-clone';
import * as THREE from "three";

import {
  AudioAnalysisResult
} from "../../generated/proto/audio/analysis/analysis_pb";
import {gaussianFilter1d} from "../../utils/filters";
import {bin, map, sum} from "../../utils/functions";
import {gaussianProb, interpolate, log, softmax} from "../../utils/math";
import {indexedSort} from "../../utils/sort";
import {ease, EaseDirection, EaseStyle} from "../../utils/transitions";
import {BaseParameterizer, Parameterizer, Parameters} from "../parameterizer";

export class TTFTemp {
  params = new TTFParams();
}

// todo: make a proto so that it can be changed from the remote
export class TTFStartConfig {
  text = "ADRIANA";
  // text = "SOPHIE";
  resolution = 40;
  size = 80;
  // font = "lynojeanregular";
  font = "interextraboldregular";
  textResolution: number|undefined = 3;
}

const defaultConfig = new TTFStartConfig();

export interface TTFCharParams {
  widthFrac: number;
  depth: number;
  colors: number[];
  textLongitudinalVelocityFactor: number;
  textLateralVelocityFactor: number;
}

export class TTFParams implements Parameters {
  // speed = 6;
  // transformSpeed = 30;
  // updateIntervalFrames = 30;
  fixedWidth = true;
  spacing = 10;
  backgroundColor = new THREE.Color("black");
  // strobeSpeed = true;
  // weightCenters = 2;
  // amplification = 1;
  // weightCenterAmps = [ 20, 50 ];
  // weightCenterVariances = [ 0.1, 0.1 ];

  chars: TTFCharParams[] =
      new Array(defaultConfig.text.length).fill(0).map(() => {
        return {
          widthFrac: 1, depth: 500,
              colors: new Array(4 * defaultConfig.resolution).fill(1.0),
              // cannot really set that because we allow full customization over
              // the colors so it would be the users job to animate the colors
              // in a different speed colorLongitudinalVelocityFactor: 1.0,
              textLongitudinalVelocityFactor: 1.0,
              textLateralVelocityFactor: 1.0,
        }
      });

  // charWidthFrac = new Array(defaultConfig.text.length).fill(1);
  // depth = new Array(defaultConfig.text.length).fill(500);
  // color =
  //     array2d(defaultConfig.text.length, 3 * defaultConfig.resolution, 1.0);

  // colorLongitudinalVelocityFactor =
  //     new Array(defaultConfig.text.length).fill(1);
  // textLongitudinalVelocityFactor = new
  // Array(defaultConfig.text.length).fill(1); textLateralVelocityFactor = new
  // Array(defaultConfig.text.length).fill(1);
}

export class TTFParameterizerConfig {
  fadeoutEaseDirection: EaseDirection|undefined = EaseDirection.OUT;
  fadeoutEaseStyle: EaseStyle|undefined = EaseStyle.QUART;
}

// todo: add user provided input, e.g. midi
export class TTFParameterizer extends BaseParameterizer<
    TTFStartConfig, AudioAnalysisResult, TTFTemp, TTFParams> implements
    Parameterizer<TTFStartConfig, AudioAnalysisResult, TTFTemp, TTFParams> {

  protected minVolumeThreshold = 1e-2;
  protected strobeIntervalFrames = 2;
  protected prideColors!: THREE.Color[];
  protected effects: {
    pride?: {start: number},
    strobe?: {start: number},
  } = {};
  // {
  // r: number[],
  // g: number[],
  // b: number[],
  // };
  public config = new TTFParameterizerConfig();

  constructor() {
    super();
    this.prideColors = [
      // "#fd1a2d", "#ff6f24", "#ff6f24", "#0e9861", "#0c50f5", "#3839b7"
      "#e50000",
      "#ff8d00",
      "#ff8d00",
      "#008122",
      "#014cff",
      "#760388",
    ].map((hex) => new THREE.Color(hex));
    this.effects.pride = {start : 0};
    // this.prideColors = {
    //   r : prideColors.map((c) => c.r),
    //   g : prideColors.map((c) => c.g),
    //   b : prideColors.map((c) => c.b),
    // };
  }

  public parameterize =
      (frame: number, config: TTFStartConfig, current: TTFParams|undefined,
       temp: TTFTemp|undefined,
       input: AudioAnalysisResult|null): [ TTFParams, TTFTemp ] => {
        const inParams = current ?? new TTFParams();
        const inTemp = temp ?? new TTFTemp();

        const outParams = new TTFParams();
        const outTemp = new TTFTemp();

        // strobe light effect
        let strobe = false;
        if (strobe) {
          if (this.effects.strobe) {
            outParams.backgroundColor = new THREE.Color(
                (frame % this.strobeIntervalFrames === 0) ? "black" : "white");
          } else {
            this.effects.strobe = {
              start : frame,
            }
          }
        } else {
          outParams.backgroundColor = new THREE.Color("black");
        }

        if (!input)
          return [ inParams, inTemp ];

        const spectral = input?.getSpectral();

        outParams.spacing = 10;
        outParams.fixedWidth = true;

        if (spectral) {
          const volume = spectral.getVolume();
          const melBands = spectral.getMelBandsList();
          let binSums = bin(melBands, config.text.length).map((b) => sum(b));
          binSums = binSums.map((s) => Math.sqrt(s));
          // bin(melBands, config.text.length).map((b) => Math.max(...b));
          const binFracs =
              softmax(binSums).map((prob) => prob * config.text.length);
          const spectrum = softmax(interpolate(melBands, config.text.length))
                               .map((prob) => prob * config.text.length);
          const charsWithGreatestEnergy =
              indexedSort(spectrum, true).slice(0, 1).map((c, centerIdx) => {
                return {
                  idx : c.idx,
                  amplification :
                      Math.pow((1 - volume) * c.value * (centerIdx + 1), 3),
                  variance : Math.sqrt((1 - volume) * config.text.length),
                };
              });
          let targetCharWidthFracs = Array.from(config.text).map((_, chIdx) => {
            return charsWithGreatestEnergy.reduce(
                (acc, center) => acc + gaussianProb(chIdx, {
                                         mu : center.idx,
                                         sigma : center.variance,
                                       }) * center.amplification,
                0);
          })

          targetCharWidthFracs = softmax(targetCharWidthFracs)
                                     .map((prob) => prob * config.text.length);
          // console.log("computed", targetCharWidthFracs);

          let [r, g, b, a] = [ 0, 0, 0, 1.0];
          if (this.effects.pride) {
            const color = this.prideColors[this.effects.pride.start %
                                           this.prideColors.length];
            [r, g, b] = [ color.r, color.g, color.b ];
            this.effects.pride.start += 1;
          } else {
            if (volume > this.minVolumeThreshold) {
              const split = Math.ceil(melBands.length / 3);
              r = (Math.max(...melBands.slice(0, split)));
              g = (Math.max(...melBands.slice(split, 2 * split)));
              b = (Math.max(...melBands.slice(2 * split, 3 * split)));
              // scale by volume
              let colorStrength = map(volume, 0.2, 0.8, 0.2, 1);
              r *= colorStrength;
              g *= colorStrength;
              b *= colorStrength;
            }
          }
          // outParams.depth = 500 * spectral.getVolume();
          // console.log(frame - (this.lastFrame ?? frame));

          // this needs smoothing and slower updates
          // colorLongitudinalVelocityFactor = new
          // Array(initParams.text.length).fill(1);
          // textLongitudinalVelocityFactor = new
          // Array(initParams.text.length).fill(1); textLateralVelocityFactor
          // = new Array(initParams.text.length).fill(1);

          Array.from(config.text).forEach((_, chIdx) => {
            const previous = inTemp.params.chars[chIdx].colors.slice(
                0, (config.resolution - 1) * 4);
            // console.assert(previous.length ==
            //                inTemp.params.chars[chIdx].colors.length - 4);

            outParams.chars[chIdx].colors =
                gaussianFilter1d(outParams.chars[chIdx].colors, 0.3);
            outParams.chars[chIdx].colors.splice(4, config.resolution * 4,
                                                 ...previous);
            outParams.chars[chIdx].colors.splice(0, 4, r, g, b, a);

            // set the char width frac
            // console.log(bins[chIdx]);
            // outParams.chars[chIdx].widthFrac = binFracs[chIdx];
            // outParams.chars[chIdx].widthFrac =
            // clone(targetCharWidthFracs[chIdx]);
            // console.log(chIdx,
            // outParams.chars[chIdx].widthFrac = targetCharWidthFracs[chIdx];
            outParams.chars[chIdx].widthFrac = binFracs[chIdx];
            // outParams.chars[chIdx].widthFrac = 2;
          })
        }

        outTemp.params = clone(outParams);

        // apply final parameter changes that are not temporary saved
        const easeFunc =
            (this.config.fadeoutEaseDirection || this.config.fadeoutEaseStyle)
            ? ease(this.config.fadeoutEaseDirection ?? EaseDirection.IN,
                   this.config.fadeoutEaseStyle ?? EaseStyle.QUINT) : undefined;

        Array.from(config.text).forEach((_, chIdx) => {
          for (let segment = 0; segment < config.resolution; segment++) {
            const fadeout =
                easeFunc ? 1 - easeFunc(segment / config.resolution) : 1;
            // console.log(segment, fadeout);
            outParams.chars[chIdx].colors[4 * segment + 3] = fadeout;
            // outParams.chars[chIdx].colors[3 * segment + 0] *= fadeout;
            // outParams.chars[chIdx].colors[3 * segment + 1] *= fadeout;
            // outParams.chars[chIdx].colors[3 * segment + 2] *= fadeout;
          }
          // debugger;
        });

        return [ outParams, outTemp ];
      }
}
