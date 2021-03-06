import "/TextEncoder.js";
// import init, {WasmBPMDetector} from "/wasm/bpm-detection/bpm_detection.js";

const nChannels = 2;
const sampleRateParam = 'sampleRate';
const windowSizeParam = 'windowSize';
const minIntervalSecParam = 'minIntervalSec';

class BPMDetectionNodeProcessor extends AudioWorkletProcessor {

  parameterDescriptors = [
    {
      name : minIntervalSecParam,
      defaultValue: 0.5,
      automationRate: "a-rate",
    },
  ];

  constructor() {
    super();
    // we cannot know the sampleRate etc.
    // we instantiate the wasm module when we receive the init message
    this.detector = null;
    this.samples = null;
    this.totalSamples = 0; // new Array(nChannels).fill(0);
    this.params =
        Object.assign({}, ...this.parameterDescriptors.map(
                              (desc) => ({[desc.name] : desc.defaultValue})));
    this.isShutdown = false;
    // this.isInitialized = false;

    this.port.onmessage = (ev) => {
      if (ev.data.type == "SHUTDOWN") {
        this.isShutdown = true;
        return;
        // } else if (ev.data.type == "LOAD_WASM") {
        //   this.initWasmModule(ev.data.data);
      } else {
        return this.handleMessage(ev);
      }
    };
  }

  handleMessage(ev) {
    console.log("event received", ev.data);
    if (ev.data.type == "INIT") {
      this.params = {...this.params, ...ev.data.data};
      this.init();
    }
  }

  init() {
    // console.log("init bpm detection", this.params);
    // this.samples = new Array(nChannels *
    // this.params[windowSizeParam]).fill(0);
    // this.detector = WasmBPMDetector.new(this.params[sampleRateParam],
    //                                     this.params[windowSizeParam]);
    // this.port.postMessage({type : "INIT_COMPLETE"});
    // this.isInitialized = true;
  }

  // async initWasmModule(data) {
  //   console.log("loading wasm module");
  //   const importObject = {
  //     env : {},
  //   };
  //   try {
  //     this.port.postMessage({type : "LOAD_WASM_COMPLETE"});
  //   } catch (e) {
  //     this.isShutdown = true;
  //     console.log(e);
  //     throw "could not load wasm module";
  //   }
  // }

  process(inputs, outputs, params) {
    if (this.isShutdown) {
      // tell the audio system to shutdown
      return false;
    }
    // wait for initialization
    // TODO: add more logic with pauses etc.
    // if (!this.isInitialized) {
    //   return true;
    // }

    // for now, just use the first input
    inputs = inputs[0];
    if (!inputs || inputs.length < 1)
      return true;

    let channels = Math.min(nChannels, inputs.length);
    let samples = new Array(channels * inputs[0].length).fill(0);
    // console.log("channels", channels);

    for (let channel = 0; channel < channels; channel++) {
      // if there are no channels to process for now, just wait
      let offset = channel * this.params[windowSizeParam];
      if (channel < inputs.length && inputs[channel].length > 0) {
        for (let sample = 0; sample < inputs[channel].length; sample++) {
          samples[(channel * inputs[channel].length) + sample] =
              inputs[channel][sample];
        }
        // let offset = channel * this.params[windowSizeParam];
        // // In the AudioWorklet spec, process() is called whenever exactly 128
        // // new audio samples have arrived. We simplify the logic for filling
        // up
        // // the buffer by making an assumption that the analysis size is 128
        // // samples or larger and is a power of 2.
        // if (this.totalSamples[channel] < this.params[windowSizeParam]) {
        //   for (const sampleValue of inputs[channel]) {
        //     this.samples[offset + this.totalSamples[channel]] = sampleValue;
        //     this.totalSamples[channel] = this.totalSamples[channel] += 1;
        //   }
        // } else {
        //   // Buffer is already full. We do not want the buffer to grow
        //   // continually, so instead will "cycle" the samples through it so
        //   that
        //   // it always holds the latest ordered samples of length equal to
        //   // numAudioSamplesPerAnalysis.

        //   // Shift the existing samples left by the length of new samples
        //   (128). const numNewSamples = inputs[channel].length; const
        //   numExistingSamples =
        //       this.params[windowSizeParam] - numNewSamples;
        //   for (let i = 0; i < numExistingSamples; i++) {
        //     this.samples[offset + i] = this.samples[offset + i +
        //     numNewSamples];
        //   }
        //   // Add the new samples onto the end, into the slot vacated by the
        //   // previous copy
        //   for (let i = 0; i < numNewSamples; i++) {
        //     this.samples[offset + numExistingSamples + i] =
        //     inputs[channel][i];
        //   }
        //   this.totalSamples[channel] += inputs[channel].length;
        // }
      }
    }

    // once our buffer has enough samples, pass them to the wasm bpm detector.
    // let minIntervalSamples =
    //     this.params[minIntervalSecParam] * this.params[sampleRateParam];
    // minIntervalSamples =
    //     Math.ceil(minIntervalSamples / inputs[0].length) * inputs[0].length;

    // if (this.totalSamples[0] % minIntervalSamples == 0 &&
    //     this.totalSamples[0] >= this.params[windowSizeParam] &&
    //     this.detector) {
    //   // console.log(`have ${this.params[windowSizeParam]} samples`);
    //   const result = this.detector.detect_bpm(
    //       this.totalSamples[0], channels, this.params[windowSizeParam],
    //       this.samples.slice(0, channels * this.params[windowSizeParam]));
    //   if (result !== 0) {
    //     this.port.postMessage({type : "BPM_UPDATE", data : {bpm : result}});
    //   }
    // }

    // if (this.detector) {
    //   const result =
    //       this.detector.detect_bpm(this.totalSamples, channels, samples);
    //   if (result !== 0)
    //     this.port.postMessage({type : "BPM_UPDATE", data : {bpm : result}});
    // }

    this.totalSamples += inputs[0].length;

    // tell the audio system to keep going
    return true;
  }
}

registerProcessor('bpm-detection-node-processor', BPMDetectionNodeProcessor);
