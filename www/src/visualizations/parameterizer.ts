export interface StartParameters {
}

export interface Parameters {
  // all visualizations must have a debug mode
  debug: boolean;
}

export interface Parameterizer<I, H, P extends Parameters> {
  parameterize(frame: number, input: I): [ P, H ];
}

//
export abstract class BaseParameterizer<I, H, P extends Parameters> implements
    Parameterizer<I, H, P> {
  // todo: this is only useful if we provide a lot of helpers in here
  // but maybe mostly composed of utils functions
  // like math?
  public abstract parameterize(frame: number, input: I): [ P, H ];
}

export interface ParameterizerClass<I, H, P extends Parameters> {
  name: string;
  new(): Parameterizer<I, H, P>;
}
