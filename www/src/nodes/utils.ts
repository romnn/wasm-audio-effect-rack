export const hasSIMDSupport = () => WebAssembly.validate(new Uint8Array([
  0, 97, 115, 109, 1,  0, 0, 0, 1,  5, 1,   96, 0,   1,  123, 3,
  2, 1,  0,   10,  10, 1, 8, 0, 65, 0, 253, 15, 253, 98, 11,
]));

export class AsyncOnce<T> {
  private getter: () => Promise<T>;
  private pending: Promise<T>|null = null;
  private res: T|null = null;

  constructor(getter: () => Promise<T>) { this.getter = getter; }

  public async get(): Promise<T> {
    if (this.res !== null) {
      return this.res;
    }
    if (this.pending) {
      return this.pending;
    }

    this.pending = new Promise((resolve) => this.getter().then((res) => {
      this.res = res;
      this.pending = null;
      resolve(res);
    }));
    return this.pending!;
  }
}

export const hslToRGB =
    (h: number, s: number, l: number): [ number, number, number ] => {
      const a = s * Math.min(l, 1 - l);
      const f = (n: number, k = (n + h / 30) % 12) =>
          l - a * Math.max(Math.min(k - 3, 9 - k, 1), -1);
      return [ f(0), f(8), f(4) ];
    };


export const mod = (x: number, m: number): number => {
  while (x < 0) x += m;
  return x % m;
};
