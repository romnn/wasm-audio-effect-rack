export const softmax = (arr: number[]):
    number[] => {
      return arr.map((value, index) => {
        return Math.exp(value) /
               arr.map((y) => Math.exp(y)).reduce((a, b) => a + b, 0);
      })
    }

export const map =
    (value: number, x1: number, y1: number, x2: number, y2: number): number =>
        (value - x1) * (y2 - x2) / (y1 - x1) + x2;

export const sum = (arr: number[]): number =>
    arr.reduce((acc, val) => acc + val, 0);

export const gaussianProb =
    (x: number, params: {mu: number, sigma: number}): number =>
        (1.0 / Math.sqrt(2 * Math.PI * Math.pow(params.sigma, 2)));
