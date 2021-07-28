import {ParameterControls} from "./controls"
import {Parameterizer, Parameters} from "./parameterizer";
import Stats from "./stats";

export interface UpdateParameterOptions {
  animated?: boolean;
  speed?: number;
}

export interface Visualization {
  readonly name: string;
  readonly isRunning: boolean;
  // public readonly isDebug: boolean;

  // adding and destroying
  init(container: HTMLElement): void;
  destroy(): void;

  // starting and stopping
  start(): void;
  pause(): void;

  // enabling and disabling of options
  // setDebug(enabled: boolean): void;
}

export interface ParameterizedVisualization<P extends Parameters> extends
    Visualization {
  readonly parameters: P;

  // get and update parameters
  updateParameters(parameters: P, options?: UpdateParameterOptions): void;
  getParameters(): P;
}

// <Controls, Parameters>
// implements Visualization {
export abstract class BaseVisualization<
    P extends Parameters, C extends ParameterControls<P>> {

  protected running = false;
  protected debug = false;
  protected container?: HTMLElement;
  protected frameCount = 0;
  protected stats!: Stats;
  protected params!: P;
  protected controls!: C;

  public get isRunning(): boolean { return this.running }
  public get isDebug() { return this.debug }
  public get parameters() { return this.params }

  public setDebug(enabled: boolean) {
    this.debug = enabled;
    // toggle stats and controls based on debug mode
  }
  protected abstract renderFrame(): void;

  protected animate = () => {
    requestAnimationFrame(this.animate);
    if (this.running) {
      this.renderFrame();
      this.frameCount++;
    }
    this.controls?.update();
    this.stats?.update();
  };

  public start() { this.running = true; }
  public pause() { this.running = false; }

  destroy() {
    // pause and remove container
    this.pause();
    if (this.container)
      this.container.innerHTML = "";
  }
}

// implements ParameterizedVisualization
export class ReactiveVisualization<V extends ParameterizedVisualization<P>, I,
                                             P extends Parameters> {
  public parameterizer?: Parameterizer<I, P>;
  public visualization!: V;
  constructor(visualization: V, paramerizer?: Parameterizer<I, P>) {
    this.visualization = visualization;
    this.parameterizer = paramerizer;
  }

  public useParameterizer(parameterizer: Parameterizer<I, P>): void {
    this.parameterizer = parameterizer;
  }

  public parameterize(input: I, options?: UpdateParameterOptions): P|undefined {
    if (this.parameterizer) {
      const parameters = this.parameterizer.parameterize(input);
      this.visualization.updateParameters(parameters, options);
      return parameters;
    }
  }
}

// todo: we need logic for websockets
// todo: we need to register at the backend that this effect is on?
// todo: we need a controller class (generic <AudioAnalyisResults,
// EffectParams>) todo todo: we need a generic viewer class ( react component
// ) that manages whatever effect is on
//
// todo: we need a wrapper that makes it reactive?
// public socket = new WebSocket(
//     "ws://" + window.location.hostname + "/ws"
//   );
