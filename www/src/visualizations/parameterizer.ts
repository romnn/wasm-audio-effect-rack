import {sum} from "../utils/functions";
import {softmax} from "../utils/math";

export interface StartConfig {}
export interface Temporary {}
export interface Input {}

export interface Parameters {}

export interface Parameterizer<C extends StartConfig, I extends Input, T extends
                                   Temporary, P extends Parameters> {
  debug: boolean;
  update(frame: number): void;
  parameterize(frame: number, config: C, current: P|undefined,
               temp: T|undefined, input: I|null): [ P, T ];
}

export abstract class BaseParameterizer<
    C extends StartConfig, I extends Input, T extends Temporary, P extends
        Parameters> implements Parameterizer<C, I, T, P> {
  public debug = false;
  protected lastUpdateFrame?: number = undefined;
  protected timeBetweenUpdates: number[] = [];
  protected meanTimeBetweenUpdates = 0;

  public update = (frame: number):
      void => {
    this.timeBetweenUpdates = [
      frame - (this.lastUpdateFrame ?? frame),
      ...this.timeBetweenUpdates
    ].slice(0, Math.min(60, this.timeBetweenUpdates.length));
    this.meanTimeBetweenUpdates =
        sum(this.timeBetweenUpdates) / this.timeBetweenUpdates.length;
    this.lastUpdateFrame = frame;
      }

  protected animate =
      (current: number, target: number): number => { return current }
  //
  // this base class is only useful if we provide a lot of helpers in here
  public abstract parameterize(frame: number, config: C, current: P|undefined,
                               temp: T|undefined, input: I|null): [ P, T ];
}

export interface ParameterizerClass<
    C extends StartConfig, I extends Input, T extends Temporary,
                                                      P extends Parameters> {
  name: string;
  new(): Parameterizer<C, I, T, P>;
}
