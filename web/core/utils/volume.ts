import {map, stddev} from "./functions";
import {RingBuffer} from "./ringbuffer";

export class RelativeVolume {

  protected volumeMean: number = 0.5;
  protected volumeStd: number = 0.5;
  protected windowSize: number;
  protected buffer: RingBuffer<number>;

  get mean(): number {
    return this.volumeMean;
  }

  get std(): number {
    return this.volumeStd;
  }

  constructor(options?: {windowSize?: number}) {
    this.windowSize = options?.windowSize ?? 1000;
    this.buffer = new RingBuffer<number>(this.windowSize);
  }

  update =
      (volume: number) => {
        this.buffer.add(volume);
        [this.volumeStd, this.volumeMean] = stddev(this.buffer.toArray());
        return this.scale(volume);
      }

  scale = (volume: number) => {
    return map(volume, Math.max(0, this.volumeMean - this.volumeStd),
               Math.min(1, this.volumeMean + this.volumeStd), 0, 1);
  }
}
