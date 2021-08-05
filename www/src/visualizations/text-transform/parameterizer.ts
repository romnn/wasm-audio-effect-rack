import * as THREE from "three";
import {
  AudioAnalysisResult
} from "../../generated/proto/audio/analysis/analysis_pb";

import {BaseParameterizer, Parameterizer, Parameters} from "../parameterizer";

export class TTFTemp {
  // todo
  anyvariableiwant = 500;
}

export class TTFInitParams {
  text = "ADRIANA";
  resolution = 40;
  colors = new Array(this.resolution).fill([ 0, 0, 0 ]);
}

const initParams = new TTFInitParams();
const array2d = <T>(nrows: number, ncols: number, fill: T):
    T[][] => { return new Array(nrows).fill(new Array(ncols).fill(fill)); }

export class TTFParams implements Parameters {
  debug = true;
  speed = 6;
  transformSpeed = 30;
  updateIntervalFrames = 30;
  fixedWidth = true;
  spacing = 10;
  weightCenters = 2;
  amplification = 0.2;
  weightCenterAmps = [ 20, 50 ];
  weightCenterVariances = [ 0.1, 0.1 ];
  depth = 500;
  colors = array2d(initParams.text.length, initParams.resolution,
                   new THREE.Color(0x000000));
}

// as input, maybe use user provided input and also music analysis in one
export class TTFParameterizer extends
    BaseParameterizer<AudioAnalysisResult, TTFTemp, TTFParams> implements
        Parameterizer<AudioAnalysisResult, TTFTemp, TTFParams> {
  public parameterize =
      (frame: number, input: AudioAnalysisResult): [ TTFParams, TTFTemp ] => {
        // assume we also get the init params
        let initParams = new TTFInitParams();
        let params = new TTFParams();
        let temp = new TTFTemp();
        let spectral = input.getSpectral();

        if (spectral) {
          // params.spacing = 100 * spectral.getVolume();
          params.amplification = 2 * spectral.getVolume();
          let melBands = spectral.getMelBandsList();
          // const r = Math.max(y[: len(y) // 3]))
          const split = Math.ceil(melBands.length / 3);
          const r = Math.ceil(Math.max(...melBands.slice(0, split)));
          const g = Math.ceil(Math.max(...melBands.slice(split, 2 * split)));
          const b =
              Math.ceil(Math.max(...melBands.slice(3 * split, 3 * split)));
          // params.colors.fill([ r, g, b ]);
          params.depth = 500 * spectral.getVolume();
          console.log([ r, g, b ]);

          params.colors = array2d(initParams.text.length, initParams.resolution,
                                  new THREE.Color(0x000000));
          // params.colors = new Array(initParams.text.length)
          //                     .fill(new Array(initParams.resolution)
          //                               .fill(new THREE.Color(0x000000)));
          // .fill(new Array(initParams.resolution).fill([ 0, 0, 0 ]));
          //         // .fill(new Array(this.resolution).fill([ 0, 0, 0 ]));
          // Array.from(initParams.text).forEach((ch, chIdx) => {
          //   colors = new Array(this.resolution).fill([ 0, 0, 0 ]);
          // })
          // for (const character of text) {
          // }
        }

        return [ params, temp ];
      }
}
