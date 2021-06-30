import {
  AudioNodeProcessorControlMessageKind,
  AudioNodeProcessorLoadWASMControlMessage,
  BPMDetectionControlMessageKind,
  BPMDetectionInitControlMessage,
  BPMDetectionNodeProcessorParameters,
} from "./common";
import {AsyncOnce, hasSIMDSupport} from "./utils";

export type BPMDetectionParameters = {
  onInitialized?: (inst: BPMDetection) => void
}

// todo: check if we really need this async once here
const BPMDetectionWasm = new AsyncOnce(async () => {
  const res =
      fetch(hasSIMDSupport() ? '/wasm/bpm-detection/bpm_detection_bg.wasm'
                             : '/wasm/bpm-detection/bpm_detection_bg.wasm');
  return res.then(res => res.arrayBuffer());
});

export interface AudioNode {
  name: string;
  ctx?: AudioContext;
  workletHandle?: AudioWorkletNode;

  serialize(): {[key: string]: any};
  // buildConnectables(): AudioConnectables&{node : ForeignNode};
  // nodeType: string;
  // paramOverrides: {
  //   [name: string]:
  //       {param: OverridableAudioParam; override : ConstantSourceNode};
  // };
  // renderSmallView?: (domId: string) => void;
  // cleanupSmallView?: (domId: string) => void;
}

export default class BPMDetection implements AudioNode {
  public name: string;
  public ctx?: AudioContext;
  public workletHandle?: AudioWorkletNode;
  // private wavetableDef: Float32Array[][] = getDefaultWavetableDef();

  private params?: BPMDetectionNodeProcessorParameters;
  private options?: BPMDetectionParameters;

  // public callbacks
  public onBPMChanged?: (bpm: number) => void;

  static typeName = 'BPM Detection';
  public nodeType = 'customAudio/bpm-detection';

  // public paramOverrides: {
  //   [name: string]:
  //       {param: OverridableAudioParam; override : ConstantSourceNode};
  // } = {};

  // the default constructor is fine for us
  // new(context: BaseAudioContext, name: string, options?:
  // AudioWorkletNodeOptions): AudioWorkletNode;
  constructor(ctx: AudioContext, name: string,
              params?: BPMDetectionNodeProcessorParameters,
              options?: BPMDetectionParameters) {
    this.ctx = ctx;
    this.name = name;
    this.options = options;
    this.params = params;
    // if (params?.wavetableDef) {
    //   this.wavetableDef = params.wavetableDef;
    // }

    this.initWorklet().then(workletHandle => {
      this.initWasmModule();

      // handle messages from the processor
      if (this.workletHandle)
        this.workletHandle.port.onmessage = (ev) => {
          if (ev.data.type ==
              AudioNodeProcessorControlMessageKind.LOAD_WASM_COMPLETE) {
            // proceed to initialize the wasm module
            const sampleRate = this.ctx?.sampleRate ?? 44100;
            this.workletHandle?.port.postMessage({
              type : BPMDetectionControlMessageKind.INIT,
              data : {
                sampleRate : sampleRate,
                windowSize : 128, // get rid of the memory as fast as possible
              }
            } as BPMDetectionInitControlMessage);
          } else if (ev.data.type ==
                     BPMDetectionControlMessageKind.INIT_COMPLETE) {
            if (this.options?.onInitialized) {
              this.options?.onInitialized(this);
            }
          } else if (ev.data.type ==
                     BPMDetectionControlMessageKind.BPM_UPDATE) {
            if (this.onBPMChanged) {
              this.onBPMChanged(ev.data.data.bpm);
            }
          } else {
            console.log("event received from processor", ev.data);
          }
        };

      // console.log("initialized now");
      // this.paramOverrides =
      // this.buildParamOverrides(workletHandle);

      // if (params) {
      //   this.deserialize(params);
      // }

      // if (this.vcId.length > 0) {
      //   updateConnectables(this.vcId, this.buildConnectables());
      // }
    });
  }

  public serialize() {
    return {};
    // return Object.entries(this.paramOverrides)
    //     .reduce((acc, [ key, val ]) =>
    //                 ({...acc, [key] : val.override.offset.value}),
    //             {} as {[key: string] : number});
  }

  private async initWasmModule() {
    // const dimensionCount = this.wavetableDef.length;
    // const waveformsPerDimension = this.wavetableDef[0].length;
    // const samplesPerDimension = waveformLength * waveformsPerDimension;

    // const tableSamples = new Float32Array(
    //     dimensionCount * waveformsPerDimension * waveformLength);
    // for (let dimensionIx = 0; dimensionIx < dimensionCount; dimensionIx++) {
    //   for (let waveformIx = 0; waveformIx < waveformsPerDimension;
    //        waveformIx++) {
    //     for (let sampleIx = 0; sampleIx < waveformLength; sampleIx++) {
    //       tableSamples[samplesPerDimension * dimensionIx +
    //                    waveformLength * waveformIx + sampleIx] =
    //           this.wavetableDef[dimensionIx][waveformIx][sampleIx];
    //     }
    //   }
    // }

    // this.workletHandle?.port.postMessage(new
    // AudioNodeProcessorLoadWASMControlMessage());
    // console.log(await BPMDetectionWasm.get());
    // console.log("posting message", {
    //   type : AudioNodeProcessorControlMessageKind.LOAD_WASM,
    //   data : await BPMDetectionWasm.get(),
    // });
    // console.log(this.workletHandle);
    this.workletHandle?.port.postMessage({
      type : AudioNodeProcessorControlMessageKind.LOAD_WASM,
      data : await BPMDetectionWasm.get(),
    });
    // after loading the module, initialize it
    //
    //
    // this.workletHandle?.port.postMessage({
    //   type: AudioNodeProcessorControlMessageKind.LOAD_WASM,
    //   data: BPMDetectionWasm.get(),
    // });
    // this.workletHandle?.port.postMessage({wasm : BPMDetectionWasm.get()});
    // this.workletHandle?.port.postMessage({
    //   parameters : {
    //     windowSize : 10,
    //     // waveformsPerDimension,
    //     // dimensionCount,
    //     // waveformLength,
    //     // baseFrequency,
    //     // tableSamples,
    //   }
    // });
  }

  private async initWorklet() {
    // await this.ctx.audioWorklet.addModule(
    // '/WaveTableNodeProcessor.js?cacheBust=' +
    // btoa(Math.random().toString()));
    if (!this.ctx)
      return;
    // debugger;
    let result = await this.ctx.audioWorklet.addModule(
        '/workers/bpm-detection-node-processor.js');
    // console.log(result);
    // debugger;
    // console.log(this.name);
    this.workletHandle = new AudioWorkletNode(this.ctx, this.name);
    // console.log(this.workletHandle);
    // debugger;

    return this.workletHandle;
  }
}
