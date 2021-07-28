import dat from "dat.gui";
import {Parameters} from "./parameterizer";

export interface ParameterControls<T extends Parameters> {
  onChange?: () => void;
  update(): void;
  // init(opts: T, container?: HTMLElement): void;
}

export abstract class DatGuiParameterControls<T extends Parameters>
    implements ParameterControls<T> {
  protected container?: HTMLElement;
  protected subContainer?: HTMLElement;
  protected gui!: dat.GUI;
  protected ctrl!: T;
  public onChange?: () => void;

  public update = () => { this.gui.updateDisplay(); };

  protected place = () => {
    const subContainer = document.createElement("div");
    subContainer.style.position = "fixed";
    subContainer.style.top = "0";
    subContainer.style.backgroundColor = "black";
    subContainer.style.right = "0";
    subContainer.appendChild(this.gui.domElement);
    (this.container ?? document.body).appendChild(subContainer);
    this.subContainer = subContainer;
  };

  // public init = (opts: T, container?: HTMLElement) => {
  constructor(opts: T, container?: HTMLElement) {
    this.ctrl = opts;
    this.container = container;
    this.gui = new dat.GUI({autoPlace : false});
    this.setup();
    this.place();
  }

  protected abstract setup(): void;
}
