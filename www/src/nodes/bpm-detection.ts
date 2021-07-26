import {
  AudioNodeProcessorControlMessageKind,
  AudioNodeProcessorLoadWASMControlMessage,
  BPMDetectionControlMessageKind,
  BPMDetectionInitControlMessage,
  BPMDetectionNodeProcessorParameters,
} from "./common";
// import {AsyncOnce, hasSIMDSupport} from "./utils";

export type BPMDetectionParameters = {
  onInitialized?: (inst: BPMDetection) => void
}

// todo: check if we really need this async once here
// const BPMDetectionWasm = new AsyncOnce(async () => {
//   const res =
//       fetch(hasSIMDSupport() ? '/wasm/bpm-detection/bpm_detection_bg.wasm'
//                              : '/wasm/bpm-detection/bpm_detection_bg.wasm');
//   return res.then(res => res.arrayBuffer());
// });

export interface AudioNode {
  name: string;
  ctx?: AudioContext;
  workletHandle?: AudioWorkletNode;
  worker: Worker;

  serialize(): {[key: string]: any};

  // typeName: string;
  // nodeType: string;
}

export default class BPMDetection implements AudioNode {
  public name: string;
  public ctx?: AudioContext;
  public workletHandle?: AudioWorkletNode;
  public worker: Worker;

  private params?: BPMDetectionNodeProcessorParameters;
  private options?: BPMDetectionParameters;

  public onBPMChanged?: (bpm: number) => void;

  static typeName = 'BPM Detection';
  public nodeType = 'customAudio/bpm-detection';

  // public paramOverrides: {
  //   [name: string]:
  //       {param: OverridableAudioParam; override : ConstantSourceNode};
  // } = {};

  constructor(ctx: AudioContext, name: string,
              params?: BPMDetectionNodeProcessorParameters,
              options?: BPMDetectionParameters) {
    this.ctx = ctx;
    this.name = name;
    this.options = options;
    this.params = params;
    this.worker =
        new Worker(`${process.env.PUBLIC_URL}/workers/bpm-detection-worker.js`);
    this.worker.onmessage = (event) => {
      console.log("message from worker: ", event);
    };

    this.initAudioWorkletProcessor().then(workletHandle => {
      // this.initWasmWorker();

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

  private async initWasmWorker() {
        // this.workletHandle?.port.postMessage({
    //   type : AudioNodeProcessorControlMessageKind.LOAD_WASM,
    //   data : await BPMDetectionWasm.get(),
    // });
  }

  private async initAudioWorkletProcessor() {
      if (!this.ctx)
        return;
      let result = await this.ctx.audioWorklet.addModule(
          '/workers/bpm-detection-node-processor.js');
      this.workletHandle = new AudioWorkletNode(this.ctx, this.name);
      return this.workletHandle;
    }
  }
