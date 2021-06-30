import {
  AudioNodeProcessorLoadWASMControlMessage,
  AudioNodeProcessorShutdownControlMessage
} from "./common";

export default class AudioNodeProcessor<WasmMod, Parameters> extends
    AudioWorkletProcessor {

  protected wasmMod: WebAssembly.Instance|null = null;
  protected isShutdown = false;

  init = () => {
    this.port.onmessage = (event: MessageEvent<any>): any => {
      // if (event instanceof AudioNodeProcessorShutdownControlMessage) {
      //   this.isShutdown = true;
      //   return;
      // } else if (event instanceof AudioNodeProcessorLoadWASMControlMessage) {
      //   this.initWasmModule(event.data);
      // } else {
      //   return this.handleMessage(event);
      // }
    };
  }

  // constructor() {
  //   super();

  //   this.wasmMod = null;
  //   this.isShutdown = false;
  //   this.port.onmessage = (event: MessageEvent<any>): any => {
  //     if (event instanceof AudioNodeProcessorShutdownControlMessage) {
  //       this.isShutdown = true;
  //       return;
  //     } else if (event instanceof AudioNodeProcessorLoadWASMControlMessage) {
  //       this.initWasmModule(event.data);
  //     } else {
  //       return this.handleMessage(event);
  //     }
  //   };
  // }

  handleMessage = (event: MessageEvent<any>) => {
    console.log("unknown event received");
  }

  async initWasmModule(data: ArrayBuffer) {
    // const debug = (id, ...args) => console.log(id}]: ${args.join(' ')};
    const importObject = {
      env : {},
    };

    // const compiledModule = await WebAssembly.compile(data.arrayBuffer);
    const compiledModule = await WebAssembly.compile(data);
    this.wasmMod = await WebAssembly.instantiate(compiledModule, importObject);
    // this.postInitWasmModule();
    // return this.wasmMod
  }

  updateParameters = (parameters: Parameters) => {}
}
