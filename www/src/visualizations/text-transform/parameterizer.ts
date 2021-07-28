import BaseParameterizer, {Parameterizer, Parameters} from "../parameterizer";

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
}

// as input, maybe use user provided input and also music analysis in one
export default class TTFParameterizer extends
    BaseParameterizer<String, TTFParams> implements
        Parameterizer<String, TTFParams> {
  public parameterize(input: String): TTFParams { return new TTFParams(); }
}
