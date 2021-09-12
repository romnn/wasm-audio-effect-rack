export enum MidiDeviceType {
  UNKNOWN,
  INPUT,
  OUTPUT,
}

export enum MidiDeviceState {
  UNKNOWN,
  DISCONNECTED,
  CONNECTED,
}

export enum MidiDeviceConnectionState {
  UNKNOWN,
  OPEN,
  CLOSED,
  PENDING,
}

export enum MidiSystemMessage {
  // System common messages
  SYSEX = 0xF0,         // 240
  TIMECODE = 0xF1,      // 241
  SONGPOSITION = 0xF2,  // 242
  SONGSELECT = 0xF3,    // 243
  TUNINGREQUEST = 0xF6, // 246
  SYSEXEND = 0xF7,      // 247
  // System real-time messages
  CLOCK = 0xF8,         // 248
  START = 0xFA,         // 250
  CONTINUE = 0xFB,      // 251
  STOP = 0xFC,          // 252
  ACTIVESENSING = 0xFE, // 254
  RESET = 0xFF,         // 255
}

export enum MidiCommand {
  PAD_DOWN = 153,
  PAD_HOLD = 169,
  PAD_UP = 137,
  KNOB_TURN = 176,
}

// add more manufacturers:
// https://www.midi.org/specifications-old/item/manufacturer-id-numbers
export const MANUFACTURERS = {
  "arturia" : [ 0x00, 0x20, 0x6B ],
}

export type MidiDevice = {
  id : string; name : string; manufacturer : string; kind : MidiDeviceType;
  deviceState : MidiDeviceState;
  connectionState : MidiDeviceConnectionState;
}

export interface MidiControllerDevice {
  inputDevice?: WebMidi.MIDIInput;
  outputDevice?: WebMidi.MIDIOutput;
  setPadColor(r: number, g: number, b: number): void;
  sendSysexMessage(manufacturer: keyof typeof MANUFACTURERS, data: number[],
                   options: {time: string}|undefined): void;
}

export abstract class BaseMidiControllerDevice {
  public sysExEnabled = true;
  public inputDevice?: WebMidi.MIDIInput;
  public outputDevice?: WebMidi.MIDIOutput;

  public sendSysexMessage = (manufacturer: keyof typeof MANUFACTURERS,
                             data: number[],
                             options: {time: string}|undefined =
                                 undefined): void => {
    if (!this.sysExEnabled) {
      throw new Error("sys ex messages are disabled");
    }

    if (data.some((n) => (n < 0 || n > 127))) {
      throw new RangeError(
          "The data bytes of a sysex message must be integers between 0 (0x00) and 127 (0x7F).");
    };

    // data = manufacturer.concat(data, MidiSystemMessage.SYSEXEND);
    data =
        [...MANUFACTURERS[manufacturer], ...data, MidiSystemMessage.SYSEXEND ];
    console.log(data);
    // this.send(wm.MIDI_SYSTEM_MESSAGES.sysex, data,
    // this._parseTimeParameter(options.time));
  };
}

export class GenericMidiControllerDevice extends BaseMidiControllerDevice
    implements MidiControllerDevice {
  public setPadColor = (r: number, g: number, b: number): void => {}
}

export class ArturiaMiniLabmkII extends BaseMidiControllerDevice implements
    MidiControllerDevice {
  public manufacturer: keyof typeof MANUFACTURERS = "arturia";

  public setPadColor = (r: number, g: number, b: number):
      void => { this.sendSysexMessage(this.manufacturer, []); }
}

export default class MidiController {
  protected inputs?: WebMidi.MIDIInputMap;
  protected outputs?: WebMidi.MIDIOutputMap;

  constructor() {
    if (!navigator.requestMIDIAccess) {
      throw new Error("MIDI is not supported by this browser");
    }
  }

  public get devices(): MidiDevice[] {
    const devices: WebMidi.MIDIPort[] = [...this.inputs?.values() ?? [],
                                         ...this.outputs?.values() ?? [] ];
    return devices.map((device) => {
      let deviceType = MidiDeviceType.UNKNOWN;
      switch (device.type) {
      case 'input':
        deviceType = MidiDeviceType.INPUT;
        break;
      case 'output':
        deviceType = MidiDeviceType.OUTPUT;
        break;
      }

      let deviceState = MidiDeviceState.UNKNOWN;
      switch (device.state) {
      case 'connected':
        deviceState = MidiDeviceState.CONNECTED;
        break;
      case 'disconnected':
        deviceState = MidiDeviceState.DISCONNECTED;
        break;
      }

      let connectionState = MidiDeviceConnectionState.UNKNOWN;
      switch (device.connection) {
      case 'open':
        connectionState = MidiDeviceConnectionState.OPEN;
        break;
      case 'closed':
        connectionState = MidiDeviceConnectionState.CLOSED;
        break;
      case 'pending':
        connectionState = MidiDeviceConnectionState.PENDING;
        break;
      }

      return {
        id: device.id ?? "unknown", name : device.name ?? "unknown",
            kind : deviceType, manufacturer: device.manufacturer ?? "unknown",
            deviceState : deviceState, connectionState: connectionState,
      }
    });
  }

  public init =
      async () => {
    let midi = await navigator.requestMIDIAccess({sysex : true});
    this.inputs = midi.inputs;
    this.outputs = midi.outputs;
    console.log("available midi inputs:", this.inputs);
    console.log("available midi outputs:", this.outputs);
    console.log("devices:", this.devices);
    for (let input of this.inputs.values())
      input.onmidimessage = this.handleMIDIMessage;
  }

  protected handleMIDIMessage = (message: WebMidi.MIDIMessageEvent) => {
    console.log(message);
    let command = message.data[0];
    let note = message.data[1];
    let velocity = (message.data.length > 2) ? message.data[2] : 0;
    console.log(command, note, velocity);

    switch (command) {
    case MidiCommand.PAD_DOWN:
      break;
    case MidiCommand.PAD_HOLD:
      break;
    case MidiCommand.PAD_UP:
      break;
    }
  }
}
