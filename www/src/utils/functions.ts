export const map =
    (value: number, x1: number, y1: number, x2: number, y2: number): number =>
        (value - x1) * (y2 - x2) / (y1 - x1) + x2;

export const sum = (arr: number[]): number =>
    arr.reduce((acc, val) => acc + val, 0);

export const bin = (arr: number[], nbins: number):
    number[][] => {
      const binSize = Math.floor(arr.length / (nbins + 1));
      return new Array(nbins).fill(0).map((_, bin) => {
        return arr.slice(bin * binSize,
                         Math.min(arr.length, (bin + 1) * binSize));
      });
    }

const array2d = <T>(nrows: number, ncols: number, fill: T):
    T[][] => { return new Array(nrows).fill(new Array(ncols).fill(fill));}
