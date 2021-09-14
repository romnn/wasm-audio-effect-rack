import {AudioAnalysisResult} from "@disco/core/audio/analysis";
import {
  Color,
  Font,
  RGBColor,
  TextTransformChar as TTFChar,
  TextTransformParameters as TTFParams,
  TextTransformStartConfig as TTFStartConfig,
} from "@disco/core/grpc";
import {gaussianFilter1d} from "@disco/core/utils/filters";
import {bin, map, rgb, sum} from "@disco/core/utils/functions";
import {
  gaussianProb,
  interpolate,
  log,
  mod,
  softmax
} from "@disco/core/utils/math";
import {indexedSort} from "@disco/core/utils/sort";
import {ease, EaseDirection, EaseStyle} from "@disco/core/utils/transitions";
import clone from 'just-clone';
import * as THREE from "three";

import {BaseParameterizer, Parameterizer, Parameters} from "../parameterizer";

export {
  TextTransformParameters as TTFParams,
  TextTransformStartConfig as TTFStartConfig,
} from "@disco/core/grpc";

// todo: make a proto so that it can be changed from the remote
// export class TTFStartConfig {
//   text = "GEORGE";
//   // text = "ADRIANA";
//   // text = "SOPHIE";
//   resolution = 50;
//   size = 80;
//   // font = "lynojeanregular";
//   font = "interextraboldregular";
//   textResolution: number|undefined = 3;
// }

export const defaultConfig = (() => {
  const c = new TTFStartConfig();
  c.setText("DISCO");
  c.setResolution(20);
  c.setSize(80);
  c.setFont(Font.INTER_EXTRA_BOLD_REGULAR);
  c.setTextResolution(2);
  return c;
})();

export const defaultParams = (() => {
  const c = new TTFParams();
  c.setBpm(120);
  c.setTransparency(false);
  c.setFixedWidth(true);
  c.setSpacing(10);
  c.setBackgroundColor(rgb(0, 0, 0));
  c.setTextLateralVelocityIntervalSeconds(20);
  c.setStrobeEnabled(false);
  c.setStrobeDuration(1);
  c.setCharList(new Array(defaultConfig.getText().length).fill(0).map(() => {
    const ch = new TTFChar();
    ch.setWidthFrac(1.0);
    ch.setDepth(700.0);
    ch.setColorList(new Array(4 * defaultConfig.getResolution()).fill(1.0));
    ch.setTextLongitudinalVelocityFactor(1.0);
    ch.setTextLateralVelocityFactor(1.0);
    return ch;
  }));
  return c;
})();

class TTFTemp {
  params = defaultParams;
  targetState = defaultParams;
}

export {TTFTemp as TextTransformTemporary};

// export class TTFParams implements Parameters {
//  bpm = 120;
//  // transformSpeed = 30;
//  // updateIntervalFrames = 30;
//  transparency = false;
//  fixedWidth = true;
//  spacing = 10;
//  backgroundColor = new THREE.Color("black");
//  textLateralVelocityIntervalSeconds = 20;
//  strobeEnabled = true;
//  strobeDuration = 1;

//  chars: TTFCharParams[] =
//      new Array(defaultConfig.text.length).fill(0).map(() => {
//        return {
//          widthFrac: 1, depth: 700,
//              colors: new Array(4 * defaultConfig.resolution).fill(1.0),
//              // cannot really set that because we allow full customization
//              // over the colors so it would be the users job to animate the
//              // colors in a different speed
//              // colorLongitudinalVelocityFactor: 1.0,
//              textLongitudinalVelocityFactor: 1.0,
//              textLateralVelocityFactor: 1.0,
//        }
//      });
//  //
//  // strobeSpeed = true;
//  // weightCenters = 2;
//  // amplification = 1;
//  // weightCenterAmps = [ 20, 50 ];
//  // weightCenterVariances = [ 0.1, 0.1 ];

//  // charWidthFrac = new Array(defaultConfig.text.length).fill(1);
//  // depth = new Array(defaultConfig.text.length).fill(500);
//  // color =
//  //     array2d(defaultConfig.text.length, 3 *
//  //     defaultConfig.resolution, 1.0);

//  // colorLongitudinalVelocityFactor =
//  //     new Array(defaultConfig.text.length).fill(1);
//  // textLongitudinalVelocityFactor = new
//  // Array(defaultConfig.text.length).fill(1); textLateralVelocityFactor = new
//  // Array(defaultConfig.text.length).fill(1);
//}

// export interface TTFCharParams {
//   widthFrac: number;
//   depth: number;
//   colors: number[];
//   textLongitudinalVelocityFactor: number;
//   textLateralVelocityFactor: number;
//   // textLateralVelocityIntervalSeconds: number;
// }

export class TextTransformParameterizerConfig {
  fadeoutEaseDirection: EaseDirection|undefined = EaseDirection.OUT;
  fadeoutEaseStyle: EaseStyle|undefined = EaseStyle.QUART;
}

// todo: add user provided input, e.g. midi
export class TextTransformParameterizer extends BaseParameterizer<
    TTFStartConfig, AudioAnalysisResult, TTFTemp, TTFParams> implements
    Parameterizer<TTFStartConfig, AudioAnalysisResult, TTFTemp, TTFParams> {

  protected minVolumeThreshold = 1e-2;
  protected strobeIntervalFrames = 4;
  protected lastCharWidthFracUpdate = 0;
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
  // public config = defaultParameters;new TTFParameterizerConfig();
  public config = new TextTransformParameterizerConfig();

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
    // this.effects.pride = {start : 0};
    // this.prideColors = {
    //   r : prideColors.map((c) => c.r),
    //   g : prideColors.map((c) => c.g),
    //   b : prideColors.map((c) => c.b),
    // };
  }
  // protected modeDuration = 60 * 60 * 0.1; // 5 minutes
  protected modeDurations = [ 60 * 60 * 0.5, 60 * 60 * 0.5, 60 * 10 ];
  protected modeFrames = 0;
  protected mode = 0;

  public parameterizeFast =
      (frame: number, config: TTFStartConfig, previous: TTFParams[],
       current: TTFParams|undefined, temp: TTFTemp|undefined,
       input: AudioAnalysisResult|null): [ TTFParams, TTFTemp ] => {
        const inParams = current ?? defaultParams;
        const inTemp = temp ?? new TTFTemp();

        const outParams = defaultParams;
        const outTemp = new TTFTemp();
        return [ inParams, inTemp ];
      }

  public parameterize = (frame: number, config: TTFStartConfig,
                         previous: TTFParams[], current: TTFParams|undefined,
                         temp: TTFTemp|undefined,
                         input: AudioAnalysisResult|
                         null): [ TTFParams, TTFTemp ] => {
    const inParams = current ?? defaultParams;
    const inTemp = temp ?? new TTFTemp();

    const outParams = defaultParams;
    const outTemp = new TTFTemp();

    // toggle mode
    if (this.modeFrames >= this.modeDurations[this.mode]) {
      this.mode = mod(this.mode + 1, this.modeDurations.length);
      this.modeFrames = 0;
    }
    this.modeFrames += 1;

    // strobe light effect
    let transparency = false;
    if (inParams.getStrobeEnabled()) {
      const shouldStrobe = Math.random() < 0.001;
      if (!this.effects.strobe && shouldStrobe) {
        this.effects.strobe = {
          start : frame,
        }
      }
      if (this.effects.strobe) {
        transparency = true;
        outParams.setBackgroundColor((frame % this.strobeIntervalFrames === 0)
                                         ? rgb(0, 0, 0)
                                         : rgb(255, 255, 255));
        if (frame - this.effects.strobe.start > inParams.getStrobeDuration()) {
          this.effects.strobe = undefined;
        }
      }
      // outParams.backgroundColor = new THREE.Color("black");
    }

    if (!input)
      return [ inParams, inTemp ];

    const spectral = input?.getSpectral();
    const bpm = input?.getBpm();
    // if (bpm) {
    //   console.log(bpm.getBpm());
    //   // Array.from(config.text)
    //   //     .forEach((_, chIdx) => {
    //   // outParams.chars[chIdx].textLongitudinalVelocityFactor =
    //   //                      bpm.getBpm()});

    //   outParams.bpm = bpm.getBpm();
    // } else {
    //   outParams.bpm = inParams.bpm
    // }
    outParams.setBpm(bpm ? bpm.getBpm() : inParams.getBpm());

    outParams.setSpacing(10);
    outParams.setFixedWidth(true);
    const text = config.getText();

    if (spectral) {
      const volume = spectral.getVolume();
      // console.log(volume);
      const melBands = spectral.getMelBandsList();
      let binSums = bin(melBands, text.length).map((b) => sum(b));
      // binSums = binSums.map((s) => Math.sqrt(s));
      // binSums = binSums.map((s) => Math.sqrt(s));
      // bin(melBands, text.length).map((b) => Math.max(...b));

      const binFracs = softmax(binSums).map(
          (prob) => 1 + (volume * ((prob * text.length) - 1)));

      const spectrum = softmax(interpolate(melBands, text.length))
                           .map((prob) => prob * text.length);
      const perChannelSpectrum = (interpolate(melBands, 3 * text.length));
      // .map((prob) => prob * 3 * text.length);

      const charsWithGreatestEnergy =
          indexedSort(spectrum, true).slice(0, 1).map((c, centerIdx) => {
            return {
              idx : c.idx,
              amplification : volume * Math.pow(c.value * (centerIdx + 1), 3),
              // (1 - volume) * Math.pow(c.value * (centerIdx + 1), 3),
              variance : Math.sqrt((1 - volume) * text.length),
            };
          });

      if (frame - this.lastCharWidthFracUpdate >=
          inParams.getTextLateralVelocityIntervalSeconds()) {

        let targetCharWidthFracs = Array.from(text).map((_, chIdx) => {
          return charsWithGreatestEnergy.reduce(
              (acc, center) => acc + gaussianProb(chIdx, {
                                       mu : center.idx,
                                       sigma : center.variance,
                                     }) * center.amplification,
              0);
        })
        targetCharWidthFracs =
            softmax(targetCharWidthFracs).map((prob) => prob * text.length);

        // console.log("computed", targetCharWidthFracs);
        Array.from(text).forEach((_, chIdx) => {
          outTemp.targetState.getCharList()[chIdx].setWidthFrac(
              binFracs[chIdx]);
          // targetCharWidthFracs[chIdx];
        });
        // console.log("updated char width fracs");
        this.lastCharWidthFracUpdate = frame;
      } else {
        Array.from(text).forEach((_, chIdx) => {
          outTemp.targetState.getCharList()[chIdx].setWidthFrac(
              inTemp.targetState.getCharList()[chIdx].getWidthFrac());
        });
      }

      let [r, g, b, a] = [ 0, 0, 0, 1.0 ];
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
      Array.from(text).forEach((_, chIdx) => {
        // todo: implement speed here
        const previous =
            inTemp.params.getCharList()[chIdx].getColorList().slice(
                0, (config.getResolution() - 1) * 4);
        // console.assert(previous.length ==
        //                inTemp.params.getCharList()[chIdx].getColorList().length
        //                - 4);

        outParams.getCharList()[chIdx].setColorList(gaussianFilter1d(
            outParams.getCharList()[chIdx].getColorList(), 0.3));
        // outParams.getCharList()[chIdx].setColorList(
        outParams.getCharList()[chIdx].getColorList().splice(
            4, config.getResolution() * 4, ...previous);
        let cr = perChannelSpectrum[chIdx * 3 + 0];
        let cg = perChannelSpectrum[chIdx * 3 + 1];
        let cb = perChannelSpectrum[chIdx * 3 + 2];
        if (this.mode == 0) {
          // leave r g b unchanged
        } else if (this.mode == 1) {
          r = Math.max(0.5 * r, cr);
          g = Math.max(0.5 * g, cg);
          b = Math.max(0.5 * b, cb);
        } else if (this.mode == 2) {
          const color = this.prideColors[frame % this.prideColors.length];
          [r, g, b] = [ color.r, color.g, color.b ];
        }
        // g = 0.2 * g + g;
        // b = 0.2 * b + cb;
        // g = (Math.max(...melBands.slice(split, 2 * split)));
        // b = (Math.max(...melBands.slice(2 * split, 3 * split)));
        // let colorStrength = spectrum[chIdx];
        // r *= colorStrength;
        // g *= colorStrength;
        // b *= colorStrength;
        // outParams.getCharList()[chIdx].setColorList(
        outParams.getCharList()[chIdx].getColorList().splice(0, 4, r, g, b, a);

        // set the char width frac
        // console.log(bins[chIdx]);
        // outParams.getCharList()[chIdx].widthFrac = binFracs[chIdx];
        // outParams.getCharList()[chIdx].widthFrac =
        // clone(targetCharWidthFracs[chIdx]);
        // console.log(chIdx,
        // if (targetCharWidthFracs) {
        //   outParams.getCharList()[chIdx].widthFrac =
        //   targetCharWidthFracs[chIdx];
        // }
        // console.log(binFracs);
        // outParams.getCharList()[chIdx].widthFrac = binFracs[chIdx];
        // outParams.getCharList()[chIdx].widthFrac = 2;
      })
    }

    outTemp.params = clone(outParams);

    // apply final parameter changes that are not temporary saved
    const easeFunc =
        (this.config.fadeoutEaseDirection || this.config.fadeoutEaseStyle)
        ? ease(this.config.fadeoutEaseDirection ?? EaseDirection.IN,
               this.config.fadeoutEaseStyle ?? EaseStyle.QUINT) : undefined;

    Array.from(text).forEach((_, chIdx) => {
      // interpolate the width fracs
      let widthFracUpdateProgress =
          (frame - this.lastCharWidthFracUpdate) /
          inParams.getTextLateralVelocityIntervalSeconds();
      let widthFrac = outParams.getCharList()[chIdx].getWidthFrac();
      widthFrac += (inTemp.targetState.getCharList()[chIdx].getWidthFrac() -
                    outTemp.params.getCharList()[chIdx].getWidthFrac()) *
                   widthFracUpdateProgress;
      outParams.getCharList()[chIdx].setWidthFrac(widthFrac);

      for (let segment = 0; segment < config.getResolution(); segment++) {
        const fadeout =
            easeFunc ? 1 - easeFunc(segment / config.getResolution()) : 1;
        // console.log(segment, fadeout);
        if (transparency) {
          outParams.getCharList()[chIdx].getColorList()[4 * segment + 3] =
              fadeout;
        } else {
          outParams.getCharList()[chIdx].getColorList()[4 * segment + 0] *=
              fadeout;
          outParams.getCharList()[chIdx].getColorList()[4 * segment + 1] *=
              fadeout;
          outParams.getCharList()[chIdx].getColorList()[4 * segment + 2] *=
              fadeout;
          outParams.getCharList()[chIdx].getColorList()[4 * segment + 3] = 1.0;
        }
        if (segment == 0) {
          outParams.getCharList()[chIdx].getColorList()[4 * segment + 0] += 0.1;
          outParams.getCharList()[chIdx].getColorList()[4 * segment + 1] += 0.1;
          outParams.getCharList()[chIdx].getColorList()[4 * segment + 2] += 0.1;
        }

        outParams.getCharList()[chIdx].getColorList()[4 * segment + 0] =
            Math.min(
                outParams.getCharList()[chIdx].getColorList()[4 * segment + 0],
                1.0);
        outParams.getCharList()[chIdx].getColorList()[4 * segment + 1] =
            Math.min(
                outParams.getCharList()[chIdx].getColorList()[4 * segment + 1],
                1.0);
        outParams.getCharList()[chIdx].getColorList()[4 * segment + 2] =
            Math.min(
                outParams.getCharList()[chIdx].getColorList()[4 * segment + 2],
                1.0);
      }
    });

    return [ outParams, outTemp ];
  }
}
