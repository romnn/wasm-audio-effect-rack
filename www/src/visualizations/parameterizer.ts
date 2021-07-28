export interface Parameters {
  // all visualizations must have a debug mode
  debug: boolean;
}

// todo, also allow outputting intermediate values that can be useful to use for
// further processing
export interface Parameterizer<I, P extends Parameters> {

  parameterize(input: I): P;
}

//
export default abstract class BaseParameterizer<I, P extends Parameters>
    implements Parameterizer<I, P> {
  // todo: this is only useful if we provide a lot of helpers in here
  // but maybe mostly composed of utils functions
  // like math?
  public abstract parameterize(input: I): P;
}
