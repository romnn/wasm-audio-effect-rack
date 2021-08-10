import clone from 'just-clone';
import {ParameterControls} from "./controls"
import {
  Input,
  Parameterizer,
  ParameterizerClass,
  Parameters,
  StartConfig,
  Temporary
} from "./parameterizer";
import Stats from "./stats";

export interface UpdateParameterOptions {
  animated?: boolean;
  speed?: number;
}

export interface Visualization<C extends StartConfig> {
  readonly name: string;
  readonly isDebug: boolean;

  init(container: HTMLElement): void;
  configure(config: C): void;

  getConfig(): C;
  destroy(): void;
  updateUI(): void;
  renderFrame(frame: number): void;
  setDebug(enabled: boolean): void;
  toggleStats(enabled: boolean): void;
  toggleControls(enabled: boolean): void;
}

export class BaseVisualization<
    C extends StartConfig, P extends Parameters,
                                     PC extends ParameterControls<P>> {

  protected debug = false;
  protected statsVisible = false;
  protected controlsVisible = false;
  protected container?: HTMLElement;
  protected stats!: Stats;
  protected controls!: PC;
  protected config!: C;

  public get isDebug() { return this.debug }

  public getConfig() { return this.config }

  public toggleStats(enabled: boolean) { this.stats?.setVisible(enabled); }

  public toggleControls(enabled: boolean) {
    this.controls?.setVisible(enabled);
  }

  public setDebug(enabled: boolean) { this.debug = enabled; }

  public updateUI() {
    this.controls?.update();
    this.stats?.update();
  }

  public destroy(): void {
    for (let c of this.container?.children ?? []) {
      c.remove();
    }
  }
}

export interface ParameterizedVisualization<
    C extends StartConfig, T extends Temporary, P extends Parameters> extends
    Visualization<C> {
  parameterize<I>(frame: number, input: I|null,
                  parameterizer: Parameterizer<C, I, T, P>,
                  options
                  ?: UpdateParameterOptions): [ P|undefined, T|undefined ];
  updateParameters(parameters: P, options?: UpdateParameterOptions): void;
  getParameters(): P;
  getTemp(): T;
}

export class BaseParameterizedVisualization<
    C extends StartConfig, T extends Temporary, P extends Parameters, PC extends
        ParameterControls<P>> extends BaseVisualization<C, P, PC> {

  protected parameters!: P;
  protected temp!: T;

  public getTemp = (): T => { return this.temp; }

  public updateParameters = (parameters: P, options?: UpdateParameterOptions):
      void => {
        this.parameters = parameters;
        this.controls?.update();
      }

  public getParameters = (): P => { return this.parameters; }

  parameterize<I>(
      frame: number, input: I|null, parameterizer: Parameterizer<C, I, T, P>,
      options?: UpdateParameterOptions): [ P|undefined, any|undefined ] {
    const [parameters, temp] = parameterizer.parameterize(
        frame, this.config, this.parameters, this.temp, input);
    this.temp = clone(temp);
    this.updateParameters(clone(parameters), options);
    return [ parameters, temp ];
  }
}

export interface VisualizationController<C, I extends Input> {
  frame: number;
  visualization: Visualization<C>;
  start(): void;
  pause(): void;
  init(container: HTMLElement): void;
  configure(config: C): void;
  getParameterizerNames(): {idx: number; name : string}[];
  parameterize(input: I|null): void;
  useParameterizerAtIndex(idx: number): void;
  useParameterizerNamed(name: string): void;
  setDebug(enabled: boolean): void;
  // controls and stats are only relevant for the visualization
  // toggleControls(enabled: boolean): void;
  // toggleStats(enabled: boolean): void;
}

// we dont want to restrict to use reactive visualization
// for parameterized ones, just use an empty parameterizer list
export interface InternalVisualizationController<
    C, I, P extends Parameters> extends VisualizationController<C, I> {
  parameterizers: ParameterizerClass<C, I, any, P>[];
  parameterizer: Parameterizer<C, I, any, P>|null;
}

export abstract class BaseVisualizationController<
    C extends StartConfig, I extends Input, T extends Temporary, P extends
        Parameters, V extends ParameterizedVisualization<C, T, P>> implements
    InternalVisualizationController<C, I, P> {
  public frame = 0;

  protected debug = false;
  protected running = false;
  protected animating = false;
  public get isRunning(): boolean { return this.running }

  public abstract visualization: V;
  public abstract parameterizers: ParameterizerClass<C, I, any, P>[];
  public abstract parameterizer: Parameterizer<C, I, any, P>|null;

  public start() { this.running = true; }
  public pause() { this.running = false; }

  public setDebug(enabled: boolean) {
    this.debug = enabled;
    if (this.parameterizer)
      this.parameterizer.debug = enabled;
  }

  protected animate = () => {
    requestAnimationFrame(this.animate);
    if (this.running) {
      this.visualization.renderFrame(this.frame);
      // call the parameterizer so that it has a change to animate in between
      // receiving updates
      this.parameterize(null);
      this.visualization.updateUI();
      this.frame++;
    }
  };

  public init = (container: HTMLElement):
      void => {
        this.visualization.init(container);
        this.visualization.setDebug(this.debug);
        if (this.parameterizer)
          this.parameterizer.debug = this.debug;
        if (!this.animating) {
          this.animate();
          this.animating = true
        }
      }

  public configure =
      (config: C): void => { this.visualization.configure(config); }

  getParameterizerNames = ():
      {idx: number; name : string}[] => {
        return this.parameterizers.map(
            (p, idx) => { return {idx, name : p.name}; });
      }

  parameterize = (input: I|null):
      void => {
        if (this.parameterizer) {
          if (input)
            // signal that this is really new data
            this.parameterizer.update(this.frame);
          this.visualization.parameterize(this.frame, input, this.parameterizer,
                                          undefined)
        }
      }

  useParameterizerAtIndex = (idx: number):
      void => {
        if (0 <= idx && (this.parameterizers.length) < idx) {
          this.useParameterizer(this.parameterizers[idx]);
        }
      }

  useParameterizerNamed = (name: string):
      void => {
        let parameterizer = this.parameterizers.find(p => p.name === name);
        if (parameterizer)
          this.useParameterizer(parameterizer);
      }

  useParameterizer =
      (parameterizer: ParameterizerClass<C, I, any, P>): void => {
        this.parameterizer = new parameterizer();
        this.parameterizer.debug = this.debug;
      }
}
