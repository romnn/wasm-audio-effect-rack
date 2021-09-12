import StatsJS from "stats.js";

export default class Stats {
  stats: StatsJS;
  container: HTMLElement;
  isVisible = true;

  constructor(container?: HTMLElement) {
    this.stats = new StatsJS();
    this.stats.showPanel(0);
    this.container = container ?? document.body;
    this.container.appendChild(this.stats.dom);
  }

  setVisible(visible: boolean): void {
    this.stats.dom.style.display = visible ? "block" : "none";
  }

  update(): void {
    this.stats.update();
  }

  start(): void {
    this.stats.begin();
  }

  end(): void {
    this.stats.end();
  }
}
