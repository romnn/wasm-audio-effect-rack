// import AudioNodeProcessor from "./node-processor.worker";
// {AudioNodeProcessorControlMessage} from "./node-processor";

export type BPMDetectionNodeProcessorParameters = {
  sampleRate?: number;
  windowSize?: number; [key: string] : any
}

// export type BPMDetectionNodeProcessorControlMessage =
// AudioNodeProcessorControlMessage<BPMDetectionNodeProcessorParameters>;
// export type BPMDetectionNodeProcessorControlMessages extends
// AudioNodeProcessorControlMessages { todo: add the possible paramter update
// stuff here

export type WASMBPMDetectionModule =
    {
        // todo: define the interface functions here

}

export enum AudioNodeProcessorControlMessageKind {
  NOOP = "NOOP",
  SHUTDOWN = "SHUTDOWN",
  LOAD_WASM = "LOAD_WASM",
  LOAD_WASM_COMPLETE = "LOAD_WASM_COMPLETE",
}

export enum BPMDetectionControlMessageKind {
  INIT = "INIT",
  INIT_COMPLETE = "INIT_COMPLETE",
  BPM_UPDATE = "BPM_UPDATE",
}

export interface AudioNodeProcessorShutdownControlMessage extends MessageEvent {
  type: AudioNodeProcessorControlMessageKind.SHUTDOWN, data: boolean;
}

export interface AudioNodeProcessorLoadWASMControlMessage extends MessageEvent {
  type: AudioNodeProcessorControlMessageKind.LOAD_WASM, data: ArrayBuffer;
}

export interface AudioNodeProcessorLoadWASMCompleteControlMessage extends
    MessageEvent {
  type: AudioNodeProcessorControlMessageKind.LOAD_WASM_COMPLETE
}

export interface BPMDetectionInitControlMessage extends MessageEvent {
  type: BPMDetectionControlMessageKind.INIT;
  data: {sampleRate: number; windowSize : number;}
}

export interface BPMDetectionInitCompleteControlMessage extends MessageEvent {
  type: BPMDetectionControlMessageKind.INIT_COMPLETE;
}

export interface BPMDetectionBPMUpdateControlMessage extends MessageEvent {
  type: BPMDetectionControlMessageKind.BPM_UPDATE;
  data: {bpm: number; sampleRate?: number; windowSize?: number;}
}

// export const isControlMessage<CtrlMessage> =
//     (msg: MessageEvent<any>, kind: AudioNodeProcessorControlMessageKind):
//         (msg is CtrlMessage) => { return "type" in msg && msg.type == kind}

// export class AudioNodeProcessorControlMessage<T> extends MessageEvent<T> {
//   type = AudioNodeProcessorControlMessageKind.NOOP;
//   // constructor(data: T) {
//   //   super(AudioNodeProcessorControlMessageKind.NOOP);
//   //   this.data = data;
//   // }
// }

// export interface AudioNodeProcessorLoadWASMControlMessage { // extends
// MessageEvent<ArrayBuffer> {
//   type: AudioNodeProcessorControlMessageKind.LOAD_WASM;
//   data: ArrayBuffer;
// }

// export interface AudioNodeProcessorLoadWASMControlMessage extends
// MessageEvent<ArrayBuffer> {
//   type: AudioNodeProcessorControlMessageKind.LOAD_WASM;
// }

// export class AudioNodeProcessorShutdownControlMessage extends
//     MessageEvent<boolean> {
//   type = AudioNodeProcessorControlMessageKind.SHUTDOWN;
// }

// export class AudioNodeProcessorLoadWASMControlMessage extends
//     MessageEvent<ArrayBuffer> {
//   type = AudioNodeProcessorControlMessageKind.LOAD_WASM;
// }

// export class AudioNodeProcessorShutdownControlMessage extends
//     AudioNodeProcessorControlMessage<boolean> {
//   type = AudioNodeProcessorControlMessageKind.SHUTDOWN;
// }

// export interface AudioNodeProcessorControlMessage<Params> {
//   parameters?: Params;
//   shutdown?: boolean;
// }
