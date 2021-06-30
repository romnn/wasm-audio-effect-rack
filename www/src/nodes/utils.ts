export const hasSIMDSupport = () => WebAssembly.validate(new Uint8Array([
  0, 97, 115, 109, 1,  0, 0, 0, 1,  5, 1,   96, 0,   1,  123, 3,
  2, 1,  0,   10,  10, 1, 8, 0, 65, 0, 253, 15, 253, 98, 11
]))

export class AsyncOnce<T> {
  private getter: () => Promise<T>;
  private pending: Promise<T>|null = null;
  // private res: Option<T> = Option.none();
  private res: T|null = null

  constructor(getter: () => Promise<T>) { this.getter = getter; }

  public async get(): Promise<T> {
    if (this.res !== null) {
      return this.res;
    }
    if (this.pending) {
      return this.pending;
    }

    this.pending = new Promise(resolve => this.getter().then(res => {
      this.res = res;
      // this.res = Option.some(res);
      this.pending = null;
      resolve(res);
    }));
    return this.pending!;
  }
}
