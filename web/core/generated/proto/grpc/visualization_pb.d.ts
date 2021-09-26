import * as jspb from "google-protobuf"

import * as google_protobuf_any_pb from 'google-protobuf/google/protobuf/any_pb';

export class VisualizationStartConfig extends jspb.Message {
  getConfig(): google_protobuf_any_pb.Any | undefined;
  setConfig(value?: google_protobuf_any_pb.Any): VisualizationStartConfig;
  hasConfig(): boolean;
  clearConfig(): VisualizationStartConfig;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): VisualizationStartConfig.AsObject;
  static toObject(includeInstance: boolean, msg: VisualizationStartConfig): VisualizationStartConfig.AsObject;
  static serializeBinaryToWriter(message: VisualizationStartConfig, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): VisualizationStartConfig;
  static deserializeBinaryFromReader(message: VisualizationStartConfig, reader: jspb.BinaryReader): VisualizationStartConfig;
}

export namespace VisualizationStartConfig {
  export type AsObject = {
    config?: google_protobuf_any_pb.Any.AsObject,
  }
}

export class VisualizationParameters extends jspb.Message {
  getParameters(): google_protobuf_any_pb.Any | undefined;
  setParameters(value?: google_protobuf_any_pb.Any): VisualizationParameters;
  hasParameters(): boolean;
  clearParameters(): VisualizationParameters;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): VisualizationParameters.AsObject;
  static toObject(includeInstance: boolean, msg: VisualizationParameters): VisualizationParameters.AsObject;
  static serializeBinaryToWriter(message: VisualizationParameters, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): VisualizationParameters;
  static deserializeBinaryFromReader(message: VisualizationParameters, reader: jspb.BinaryReader): VisualizationParameters;
}

export namespace VisualizationParameters {
  export type AsObject = {
    parameters?: google_protobuf_any_pb.Any.AsObject,
  }
}

export class GenericVisualizationConfig extends jspb.Message {
  getIntsMap(): jspb.Map<string, number>;
  clearIntsMap(): GenericVisualizationConfig;

  getUintsMap(): jspb.Map<string, number>;
  clearUintsMap(): GenericVisualizationConfig;

  getFloatsMap(): jspb.Map<string, number>;
  clearFloatsMap(): GenericVisualizationConfig;

  getStringsMap(): jspb.Map<string, string>;
  clearStringsMap(): GenericVisualizationConfig;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GenericVisualizationConfig.AsObject;
  static toObject(includeInstance: boolean, msg: GenericVisualizationConfig): GenericVisualizationConfig.AsObject;
  static serializeBinaryToWriter(message: GenericVisualizationConfig, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GenericVisualizationConfig;
  static deserializeBinaryFromReader(message: GenericVisualizationConfig, reader: jspb.BinaryReader): GenericVisualizationConfig;
}

export namespace GenericVisualizationConfig {
  export type AsObject = {
    intsMap: Array<[string, number]>,
    uintsMap: Array<[string, number]>,
    floatsMap: Array<[string, number]>,
    stringsMap: Array<[string, string]>,
  }
}

export class FractalTunnelStartConfig extends jspb.Message {
  getNumPointsPerSubset(): number;
  setNumPointsPerSubset(value: number): FractalTunnelStartConfig;

  getNumSubsets(): number;
  setNumSubsets(value: number): FractalTunnelStartConfig;

  getNumLevels(): number;
  setNumLevels(value: number): FractalTunnelStartConfig;

  getLevelDepth(): number;
  setLevelDepth(value: number): FractalTunnelStartConfig;

  getScaleFactor(): number;
  setScaleFactor(value: number): FractalTunnelStartConfig;

  getSpriteSize(): number;
  setSpriteSize(value: number): FractalTunnelStartConfig;

  getCameraBound(): number;
  setCameraBound(value: number): FractalTunnelStartConfig;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FractalTunnelStartConfig.AsObject;
  static toObject(includeInstance: boolean, msg: FractalTunnelStartConfig): FractalTunnelStartConfig.AsObject;
  static serializeBinaryToWriter(message: FractalTunnelStartConfig, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FractalTunnelStartConfig;
  static deserializeBinaryFromReader(message: FractalTunnelStartConfig, reader: jspb.BinaryReader): FractalTunnelStartConfig;
}

export namespace FractalTunnelStartConfig {
  export type AsObject = {
    numPointsPerSubset: number,
    numSubsets: number,
    numLevels: number,
    levelDepth: number,
    scaleFactor: number,
    spriteSize: number,
    cameraBound: number,
  }
}

export class TextTransformStartConfig extends jspb.Message {
  getText(): string;
  setText(value: string): TextTransformStartConfig;

  getResolution(): number;
  setResolution(value: number): TextTransformStartConfig;

  getSize(): number;
  setSize(value: number): TextTransformStartConfig;

  getFont(): Font;
  setFont(value: Font): TextTransformStartConfig;

  getTextResolution(): number;
  setTextResolution(value: number): TextTransformStartConfig;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TextTransformStartConfig.AsObject;
  static toObject(includeInstance: boolean, msg: TextTransformStartConfig): TextTransformStartConfig.AsObject;
  static serializeBinaryToWriter(message: TextTransformStartConfig, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TextTransformStartConfig;
  static deserializeBinaryFromReader(message: TextTransformStartConfig, reader: jspb.BinaryReader): TextTransformStartConfig;
}

export namespace TextTransformStartConfig {
  export type AsObject = {
    text: string,
    resolution: number,
    size: number,
    font: Font,
    textResolution: number,
  }
}

export class RGBColor extends jspb.Message {
  getR(): number;
  setR(value: number): RGBColor;

  getG(): number;
  setG(value: number): RGBColor;

  getB(): number;
  setB(value: number): RGBColor;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RGBColor.AsObject;
  static toObject(includeInstance: boolean, msg: RGBColor): RGBColor.AsObject;
  static serializeBinaryToWriter(message: RGBColor, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RGBColor;
  static deserializeBinaryFromReader(message: RGBColor, reader: jspb.BinaryReader): RGBColor;
}

export namespace RGBColor {
  export type AsObject = {
    r: number,
    g: number,
    b: number,
  }
}

export class HSLColor extends jspb.Message {
  getH(): number;
  setH(value: number): HSLColor;

  getS(): number;
  setS(value: number): HSLColor;

  getL(): number;
  setL(value: number): HSLColor;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HSLColor.AsObject;
  static toObject(includeInstance: boolean, msg: HSLColor): HSLColor.AsObject;
  static serializeBinaryToWriter(message: HSLColor, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HSLColor;
  static deserializeBinaryFromReader(message: HSLColor, reader: jspb.BinaryReader): HSLColor;
}

export namespace HSLColor {
  export type AsObject = {
    h: number,
    s: number,
    l: number,
  }
}

export class Color extends jspb.Message {
  getRgb(): RGBColor | undefined;
  setRgb(value?: RGBColor): Color;
  hasRgb(): boolean;
  clearRgb(): Color;

  getHsl(): HSLColor | undefined;
  setHsl(value?: HSLColor): Color;
  hasHsl(): boolean;
  clearHsl(): Color;

  getColorCase(): Color.ColorCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Color.AsObject;
  static toObject(includeInstance: boolean, msg: Color): Color.AsObject;
  static serializeBinaryToWriter(message: Color, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Color;
  static deserializeBinaryFromReader(message: Color, reader: jspb.BinaryReader): Color;
}

export namespace Color {
  export type AsObject = {
    rgb?: RGBColor.AsObject,
    hsl?: HSLColor.AsObject,
  }

  export enum ColorCase { 
    COLOR_NOT_SET = 0,
    RGB = 1,
    HSL = 2,
  }
}

export class TextTransformChar extends jspb.Message {
  getWidthFrac(): number;
  setWidthFrac(value: number): TextTransformChar;

  getDepth(): number;
  setDepth(value: number): TextTransformChar;

  getColorList(): Array<number>;
  setColorList(value: Array<number>): TextTransformChar;
  clearColorList(): TextTransformChar;
  addColor(value: number, index?: number): TextTransformChar;

  getTextLongitudinalVelocityFactor(): number;
  setTextLongitudinalVelocityFactor(value: number): TextTransformChar;

  getTextLateralVelocityFactor(): number;
  setTextLateralVelocityFactor(value: number): TextTransformChar;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TextTransformChar.AsObject;
  static toObject(includeInstance: boolean, msg: TextTransformChar): TextTransformChar.AsObject;
  static serializeBinaryToWriter(message: TextTransformChar, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TextTransformChar;
  static deserializeBinaryFromReader(message: TextTransformChar, reader: jspb.BinaryReader): TextTransformChar;
}

export namespace TextTransformChar {
  export type AsObject = {
    widthFrac: number,
    depth: number,
    colorList: Array<number>,
    textLongitudinalVelocityFactor: number,
    textLateralVelocityFactor: number,
  }
}

export class TextTransformParameters extends jspb.Message {
  getBpm(): number;
  setBpm(value: number): TextTransformParameters;

  getTransparency(): boolean;
  setTransparency(value: boolean): TextTransformParameters;

  getFixedWidth(): boolean;
  setFixedWidth(value: boolean): TextTransformParameters;

  getSpacing(): number;
  setSpacing(value: number): TextTransformParameters;

  getBackgroundColor(): Color | undefined;
  setBackgroundColor(value?: Color): TextTransformParameters;
  hasBackgroundColor(): boolean;
  clearBackgroundColor(): TextTransformParameters;

  getTextLateralVelocityIntervalSeconds(): number;
  setTextLateralVelocityIntervalSeconds(value: number): TextTransformParameters;

  getStrobeEnabled(): boolean;
  setStrobeEnabled(value: boolean): TextTransformParameters;

  getStrobeDuration(): number;
  setStrobeDuration(value: number): TextTransformParameters;

  getCharList(): Array<TextTransformChar>;
  setCharList(value: Array<TextTransformChar>): TextTransformParameters;
  clearCharList(): TextTransformParameters;
  addChar(value?: TextTransformChar, index?: number): TextTransformChar;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TextTransformParameters.AsObject;
  static toObject(includeInstance: boolean, msg: TextTransformParameters): TextTransformParameters.AsObject;
  static serializeBinaryToWriter(message: TextTransformParameters, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TextTransformParameters;
  static deserializeBinaryFromReader(message: TextTransformParameters, reader: jspb.BinaryReader): TextTransformParameters;
}

export namespace TextTransformParameters {
  export type AsObject = {
    bpm: number,
    transparency: boolean,
    fixedWidth: boolean,
    spacing: number,
    backgroundColor?: Color.AsObject,
    textLateralVelocityIntervalSeconds: number,
    strobeEnabled: boolean,
    strobeDuration: number,
    charList: Array<TextTransformChar.AsObject>,
  }
}

export class FractalTunnelOrbitConstraints extends jspb.Message {
  getAMin(): number;
  setAMin(value: number): FractalTunnelOrbitConstraints;

  getAMax(): number;
  setAMax(value: number): FractalTunnelOrbitConstraints;

  getBMin(): number;
  setBMin(value: number): FractalTunnelOrbitConstraints;

  getBMax(): number;
  setBMax(value: number): FractalTunnelOrbitConstraints;

  getCMin(): number;
  setCMin(value: number): FractalTunnelOrbitConstraints;

  getCMax(): number;
  setCMax(value: number): FractalTunnelOrbitConstraints;

  getDMin(): number;
  setDMin(value: number): FractalTunnelOrbitConstraints;

  getDMax(): number;
  setDMax(value: number): FractalTunnelOrbitConstraints;

  getEMin(): number;
  setEMin(value: number): FractalTunnelOrbitConstraints;

  getEMax(): number;
  setEMax(value: number): FractalTunnelOrbitConstraints;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FractalTunnelOrbitConstraints.AsObject;
  static toObject(includeInstance: boolean, msg: FractalTunnelOrbitConstraints): FractalTunnelOrbitConstraints.AsObject;
  static serializeBinaryToWriter(message: FractalTunnelOrbitConstraints, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FractalTunnelOrbitConstraints;
  static deserializeBinaryFromReader(message: FractalTunnelOrbitConstraints, reader: jspb.BinaryReader): FractalTunnelOrbitConstraints;
}

export namespace FractalTunnelOrbitConstraints {
  export type AsObject = {
    aMin: number,
    aMax: number,
    bMin: number,
    bMax: number,
    cMin: number,
    cMax: number,
    dMin: number,
    dMax: number,
    eMin: number,
    eMax: number,
  }
}

export class FractalTunnelParameters extends jspb.Message {
  getA(): number;
  setA(value: number): FractalTunnelParameters;

  getB(): number;
  setB(value: number): FractalTunnelParameters;

  getC(): number;
  setC(value: number): FractalTunnelParameters;

  getD(): number;
  setD(value: number): FractalTunnelParameters;

  getE(): number;
  setE(value: number): FractalTunnelParameters;

  getSpeed(): number;
  setSpeed(value: number): FractalTunnelParameters;

  getRotationSpeed(): number;
  setRotationSpeed(value: number): FractalTunnelParameters;

  getLevelHueList(): Array<number>;
  setLevelHueList(value: Array<number>): FractalTunnelParameters;
  clearLevelHueList(): FractalTunnelParameters;
  addLevelHue(value: number, index?: number): FractalTunnelParameters;

  getLevelBrightnessList(): Array<number>;
  setLevelBrightnessList(value: Array<number>): FractalTunnelParameters;
  clearLevelBrightnessList(): FractalTunnelParameters;
  addLevelBrightness(value: number, index?: number): FractalTunnelParameters;

  getLevelSaturationList(): Array<number>;
  setLevelSaturationList(value: Array<number>): FractalTunnelParameters;
  clearLevelSaturationList(): FractalTunnelParameters;
  addLevelSaturation(value: number, index?: number): FractalTunnelParameters;

  getOrbitConstraints(): FractalTunnelOrbitConstraints | undefined;
  setOrbitConstraints(value?: FractalTunnelOrbitConstraints): FractalTunnelParameters;
  hasOrbitConstraints(): boolean;
  clearOrbitConstraints(): FractalTunnelParameters;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FractalTunnelParameters.AsObject;
  static toObject(includeInstance: boolean, msg: FractalTunnelParameters): FractalTunnelParameters.AsObject;
  static serializeBinaryToWriter(message: FractalTunnelParameters, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FractalTunnelParameters;
  static deserializeBinaryFromReader(message: FractalTunnelParameters, reader: jspb.BinaryReader): FractalTunnelParameters;
}

export namespace FractalTunnelParameters {
  export type AsObject = {
    a: number,
    b: number,
    c: number,
    d: number,
    e: number,
    speed: number,
    rotationSpeed: number,
    levelHueList: Array<number>,
    levelBrightnessList: Array<number>,
    levelSaturationList: Array<number>,
    orbitConstraints?: FractalTunnelOrbitConstraints.AsObject,
  }
}

export enum Font { 
  MOTOYALMARU_W3_MONO = 0,
  LYNO_WALT_REGULAR = 1,
  LYNO_JEAN_REGULAR = 2,
  LYNO_STAN_REGULAR = 3,
  LYNO_ULYS_REGULAR = 4,
  INTER_EXTRA_BOLD_REGULAR = 5,
}
