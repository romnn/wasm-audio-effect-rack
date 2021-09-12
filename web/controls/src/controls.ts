import {Parameters} from "@disco/visuals/parameterizer";
import dat from "dat.gui";

export interface ParameterControls<T extends Parameters> {
  onChange?: (paramters: T) => void;
  update(): void;
  setVisible(visible: boolean): void;
}

export abstract class DatGuiParameterControls<T extends Parameters> implements
    ParameterControls<T> {
  protected container?: HTMLElement;
  protected subContainer?: HTMLElement;
  protected gui!: dat.GUI;
  protected ctrl!: T;
  protected visible = false;

  public onChange?: (parameters: T) => void;

  public didChange =
      (parameters: T) => {
        if (this.onChange)
          this.onChange(this.ctrl);
      }

  public update = () => { this.gui.updateDisplay(); };

  protected remove =
      () => {
        this.subContainer?.remove();
        this.visible = false;
      }

  protected place = () => {
    const subContainer = document.createElement("div");
    subContainer.style.position = "fixed";
    subContainer.style.top = "0";
    subContainer.style.backgroundColor = "black";
    subContainer.style.right = "0";
    subContainer.appendChild(this.gui.domElement);
    (this.container ?? document.body).appendChild(subContainer);
    this.subContainer = subContainer;
    this.visible = true;
  };

  constructor(opts: T, container?: HTMLElement) {
    this.ctrl = opts;
    this.container = container;
    this.gui = new dat.GUI({autoPlace : false});
    this.setup();
  }

  public setVisible = (visible: boolean) => {
    if (visible && !this.visible) {
      this.place();
    } else if (!visible && this.visible) {
      this.remove();
    }
  }

  protected abstract setup(): void;
}
