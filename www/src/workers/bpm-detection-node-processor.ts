// import {
//   BPMDetectionNodeProcessorParameters,
//   WASMBPMDetectionModule
// } from "./common";
// import AudioNodeProcessor from "./node-processor.worker";

// "target": "es5",
// "lib": [
//     "dom",
//     "dom.iterable",
//     "esnext",
//     "webworker"
// ],
// "allowJs": true,
// "moduleResolution": "node",
// "esModuleInterop": false,
// "skipLibCheck": true,
// "module": "none",

// export default class BPMDetectionNodeProcessor extends
// AudioNodeProcessor<WASMBPMDetectionModule,
// BPMDetectionNodeProcessorParameters> {
// import {
//   AudioNodeProcessorControlMessageKind,
//   AudioNodeProcessorShutdownControlMessage,
//   // isControlMessage
// } from "../nodes/common";

// NOTE TO SELF: cannot import in the worklet processor at all!

class BPMDetectionNodeProcessor extends AudioWorkletProcessor {

  parameterDescriptors: AudioParamDescriptor[] = [
    {
      name : 'frequency',
      defaultValue: 440,
      automationRate: "a-rate",
    },
    // ...Array(MAX_DIMENSION_COUNT)
    //   .fill(null)
    //   .map((_, i) => ({
    //     name:
    //     defaultValue: 0.0,
    //     minValue: 0.0,
    //     maxValue: 1.0,
    //     automationRate: 'a-rate',
    //   })),
  ];

  protected wasmMod: WebAssembly.Instance|null = null;
  protected isShutdown = false;

  init() {
    this.port.onmessage = (ev: MessageEvent<any>) => {
      if (ev.type == "SHUTDOWN") {
        this.isShutdown = true;
        return;
      } else if (ev.type == "WASM") {
        this.initWasmModule(ev.data);
      } else {
        return this.handleMessage(ev);
      }
    };
  }

  handleMessage(ev: MessageEvent<any>) {
    console.log("unknown event received");
  }

  async initWasmModule(data: ArrayBuffer) {
    console.log("loading wasm module");
    return
    // we compile and instantiate the wasm module into this.wasmMod
    // const debug = (id, ...args) => console.log(id}]: ${args.join(' ')};
    const importObject = {
      env : {},
    };

    // const compiledModule = await WebAssembly.compile(data.arrayBuffer);
    const compiledModule = await WebAssembly.compile(data);
    this.wasmMod = await WebAssembly.instantiate(compiledModule, importObject);

    // // const debug = (id, ...args) => console.log(id}]: ${args.join(' ')};
    // const importObject = {
    //   env : {},
    // };

    // const compiledModule = await WebAssembly.compile(data);
    // // const compiledModule = await WebAssembly.compile(data.arrayBuffer);
    // this.wasmMod = await WebAssembly.instantiate(compiledModule,
    // importObject);

    // this.waveTablePtr = this.wasmInstance.exports.init_wavetable(
    //     data.waveformsPerDimension, data.dimensionCount, data.waveformLength,
    //     data.baseFrequency);

    // // Wasm memory doesn't become available until after some function in the
    // // Wasm module has been called, apparently, so we wait to set this
    // reference
    // // until after calling one of the Wasm functions.
    // this.float32WasmMemory =
    //     new Float32Array(this.wasmInstance.exports.memory.buffer);

    // const wavetableDataPtr =
    //     this.wasmInstance.exports.get_data_table_ptr(this.waveTablePtr);
    // const wavetableDataArrayOffset = wavetableDataPtr / BYTES_PER_F32;
    // if (wavetableDataPtr % 4 !== 0) {
    //   throw new Error('Wavetable data array pointer is not 32-bit aligned');
    // }

    // // We set a marker value into the data table on the Wasm side; we check
    // that
    // // it matches here to ensure that we've got the correct pointer;
    // if (this.float32WasmMemory[wavetableDataArrayOffset] !== -1) {
    //   throw new Error(
    //       'Marker value not set at initial wavetable sample data table
    //       pointer retrieved from Wasm');
    // }

    // // Write the table's data into the Wasm heap
    // this.float32WasmMemory.set(data.tableSamples, wavetableDataArrayOffset);

    // this.waveTableHandlePtr =
    //     this.wasmInstance.exports.init_wavetable_handle(this.waveTablePtr);

    // this.dimensionCount = data.dimensionCount;
    // const mixesPtr = this.wasmInstance.exports.get_mixes_ptr(
    //     this.waveTableHandlePtr, FRAME_SIZE);
    // if (mixesPtr % 4 !== 0) {
    //   throw new Error("Mixes array pointer isn't 4-byte aligned");
    // }
    // this.mixesArrayOffset = mixesPtr / BYTES_PER_F32;

    // const frequencyBufPtr = this.wasmInstance.exports.get_frequencies_ptr(
    //     this.waveTableHandlePtr, FRAME_SIZE);
    // if (frequencyBufPtr % 4 !== 0) {
    //   throw new Error("Frequency buffer pointer isn't 4-byte aligned");
    // }
    // this.frequencyBufArrayOffset = frequencyBufPtr / BYTES_PER_F32;
  }

  // {[key: string]: Float32Array}
  process(inputs: Float32Array[][], outputs: Float32Array[][],
          params: Record<string, Float32Array>): boolean {
    // process(inputs, outputs, params) {
    if (this.isShutdown) {
      return false;
    }
    // else if (!this.waveTableHandlePtr) {
    //   return true;
    // }
    return true;
  }
}

registerProcessor('bpm-detection-node-processor', BPMDetectionNodeProcessor);
