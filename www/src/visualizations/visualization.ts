import {ParameterControls} from "./controls"
import {
  Parameterizer,
  ParameterizerClass,
  Parameters,
  StartParameters
} from "./parameterizer";
import Stats from "./stats";

export interface UpdateParameterOptions {
  animated?: boolean;
  speed?: number;
}

export interface Visualization<SP extends StartParameters> {
  readonly name: string;
  readonly isDebug: boolean;

  init(container: HTMLElement, parameters: StartParameters): void;
  destroy(): void;
  updateUI(): void;
  renderFrame(frame: number): void;
  setDebug(enabled: boolean): void;
}

export class BaseVisualization<P extends Parameters,
                                         C extends ParameterControls<P>> {

  protected debug = false;
  protected container?: HTMLElement;
  protected stats!: Stats;
  protected controls!: C;

  constructor() {}

  public get isDebug() { return this.debug }

  public setDebug(enabled: boolean) {
    this.debug = enabled;
    this.stats?.setVisible(this.debug);
    this.controls?.setVisible(this.debug);
  }

  public updateUI() {
    this.controls?.update();
    this.stats?.update();
  }

  // protected abstract renderFrame(): void;
  public destroy(): void {
    if (this.container)
      for (let c of this.container.children) {
        c.remove();
      }
  }
}

export interface ParameterizedVisualization<P extends Parameters> extends
    Visualization {
  parameterize<I>(frame: number, input: I,
                  parameterizer: Parameterizer<I, any, P>,
                  options?: UpdateParameterOptions): P|undefined;
  updateParameters(parameters: P, options?: UpdateParameterOptions): void;
  getParameters(): P;
}

// implements ParameterizedVisualization
// export class ReactiveVisualization<V extends ParameterizedVisualization<P>,
// I,
//                                              H, P extends Parameters> {
// export interface ReactiveVisualization<I, H, P extends Parameters> extends
// ParameterizedVisualization<P> {
// public parameterizer?: Parameterizer<I, H, P>;
// public visualization!: V;
// constructor(visualization: V, paramerizer?: Parameterizer<I, H, P>) {
//   this.visualization = visualization;
//   this.parameterizer = paramerizer;
// }
// constructor(visualization: V) { this.visualization = visualization; }

// public useParameterizer(parameterizer: Parameterizer<I, H, P>): void {
//   this.parameterizer = parameterizer;
// }

// parameterize(frame: number, input: I, parameterizer: Parameterizer<I, H, P>,
//              options?: UpdateParameterOptions): P|undefined;

// public parameterize(frame: number, input: I,
//                     options?: UpdateParameterOptions): P|undefined {
//   if (this.parameterizer) {
//     const [parameters, temp] = this.parameterizer.parameterize(frame,
//     input); this.visualization.updateParameters(parameters, options);
//     return parameters;
//   }
// }
// }

export class BaseParameterizedVisualization<
    // I, H, P extends Parameters, C extends ParameterControls<P>> extends
    P extends Parameters, C extends ParameterControls<P>> extends
    BaseVisualization<P, C> {
  // protected abstract parameters: P;
  protected parameters!: P;

  constructor() { super(); }

  public updateParameters = (parameters: P, options?: UpdateParameterOptions):
      void => { this.parameters = parameters; }

  public getParameters = (): P => { return this.parameters; }

  parameterize<I>(frame: number, input: I,
                  parameterizer: Parameterizer<I, any, P>,
                  options?: UpdateParameterOptions): P|undefined {
    // public parameterize =
    //     (frame: number, input: I, parameterizer: Parameterizer<I, H, P>,
    //      options?: UpdateParameterOptions): P|undefined => {
    const [parameters, temp] = parameterizer.parameterize(frame, input);
    this.updateParameters(parameters, options);
    return parameters;
  }
}

export interface VisualizationController<I> {
  frame: number;
  visualization: Visualization;
  start(): void;
  pause(): void;
  init(container: HTMLElement): void;
  getParameterizerNames(): {idx: number; name : string}[];
  parameterize(input: I): void;
  useParameterizerAtIndex(idx: number): void;
  useParameterizerNamed(name: string): void;
}

// we dont want to restrict to use reactive visualization
// for parameterized ones, just use an empty parameterizer list
export interface InternalVisualizationController<
    // I, H, P extends Parameters, V extends ParameterizedVisualization<P>>
    // extends
    I, P extends Parameters, V extends Visualization> extends
    VisualizationController<I> {
  // visualization: V;
  parameterizers: ParameterizerClass<I, any, P>[];
  parameterizer: Parameterizer<I, any, P>|null;
  // getParameterizer(): Parameterizer<I, H, P>|null;
  // useParameterizer(parameterizer: ParameterizerClass<I, any, P>): void;
}

export abstract class BaseVisualizationController<
    I, P extends Parameters, V extends ParameterizedVisualization<P>> implements
    InternalVisualizationController<I, P, V> {
  public frame = 0;

  protected running = false;
  public get isRunning(): boolean { return this.running }

  public abstract visualization: V;
  public abstract parameterizers: ParameterizerClass<I, any, P>[];
  public abstract parameterizer: Parameterizer<I, any, P>|null;

  public start() { this.running = true; }
  public pause() { this.running = false; }

  protected animate = () => {
    requestAnimationFrame(this.animate);
    if (this.running) {
      this.visualization.renderFrame(this.frame);
      this.visualization.updateUI();
      this.frame++;
    }
  };

  public init = (container: HTMLElement):
      void => {
        this.visualization.init(container);
        this.animate();
      }

  getParameterizerNames = ():
      {idx: number; name : string}[] => {
        return this.parameterizers.map(
            (p, idx) => { return {idx, name : p.name}; });
      }

  parameterize = (input: I):
      void => {
        let parameterizer = this.parameterizer;
        if (parameterizer) {
          // console.log("parameterize with", frame, input);
          this.visualization.parameterize(this.frame, input, parameterizer,
                                          undefined)
        }
      }

  // getParameterizer =
  //     (): ParameterizerType|null => { return this.currentParameterizer; }

  useParameterizerAtIndex = (idx: number):
      void => {
        if (0 <= idx && (this.parameterizers.length) < idx) {
          this.parameterizer = new this.parameterizers[idx]();
          // this.useParameterizer(this.parameterizers[idx]);
        }
      }

  useParameterizerNamed = (name: string): void => {
    let parameterizer = this.parameterizers.find(p => p.name == name);
    if (parameterizer)
      // this.useParameterizer(parameterizer);
      this.parameterizer = new parameterizer();
  }

  // useParameterizer =
  //     (parameterizer:
  //          ParameterizerClass<I, any, P>):
  //         void => {
  //           this.parameterizer
  //         }

  // parameterize(frame: number, input: I): void;
  // useParameterizerAtIndex(idx: number): void;
  // useParameterizerNamed(name: string): void;
}
