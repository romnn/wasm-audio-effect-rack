import {binary_search_with_guess} from "./search";

export const softmax = (arr: number[]):
    number[] => {
      return arr.map((value, index) => {
        return Math.exp(value) /
               arr.map((y) => Math.exp(y)).reduce((a, b) => a + b, 0);
      })
    }

export const gaussianProb = (x: number, params: {mu: number, sigma: number}):
    number => {
      let ans = 1.0 / (Math.sqrt(2 * Math.PI) * params.sigma);
      ans = ans * Math.pow(Math.E, (-Math.pow(x - params.mu, 2)) /
                                       (2 * Math.pow(params.sigma, 2)));
      return ans;
    }

export const log =
    (base: number, x: number) => { return Math.log(x) / Math.log(base);}

export const interp =
    (x: number[], xp: number[], fp: number[],
     left: number|undefined = undefined, right: number|undefined = undefined):
        number[] => {
          console.assert(xp.length == fp.length,
                         "xp and fp must have the same length");
          console.assert(xp.length > 0, "xp must not be empty");
          const ans = new Array(x.length).fill(0.0);
          const l = left ?? fp[0];
          const r = right ?? fp[-1];

          // binary_search_with_guess needs at least a 3 item long array
          if (xp.length == 1) {
            const xpv = xp[0];
            const fpv = fp[0];
            for (let i = 0; i < x.length; ++i) {
              const xv = x[i];
              ans[i] = (xv < xpv) ? l : ((xv > xpv) ? r : fpv);
            }
          } else {
            let j = 0;

            // only pre-calculate slopes if there are relatively few of them
            let slopes: number[]|null = null;
            if (xp.length <= x.length) {
              slopes = new Array(xp.length - 1).fill(0.0);
              if (slopes) {
                for (let i = 0; i < slopes.length - 1; ++i) {
                  slopes[i] = (fp[i + 1] - fp[i]) / (xp[i + 1] - xp[i]);
                };
              };
            }

            for (let i = 0; i < x.length; ++i) {
              const xv = x[i];

              if (Number.isNaN(xv)) {
                ans[i] = xv;
                continue;
              }

              j = binary_search_with_guess(xv, xp, j);
              if (j == -1) {
                ans[i] = l;
              } else if (j == xp.length) {
                ans[i] = r;
              } else if (j == xp.length - 1) {
                ans[i] = fp[j];
              } else if (xp[j] == xv) {
                // avoid potential non-finite interpolation
                ans[i] = fp[j];
              } else {
                const slope = (slopes != null)
                                  ? slopes[j]
                                  : (fp[j + 1] - fp[j]) / (xp[j + 1] - xp[j]);

                // if we get nan in one direction, try the other
                ans[i] = slope * (xv - xp[j]) + fp[j];
                // if (NPY_UNLIKELY(npy_isnan(dres[i]))) {
                if (Number.isNaN(ans[i])) {
                  ans[i] = slope * (xv - xp[j + 1]) + fp[j + 1];
                  // if (NPY_UNLIKELY(npy_isnan(dres[i])) && dy[j] == dy[j+1]) {
                  if (Number.isNaN(ans[i]) && fp[j] == fp[j + 1]) {
                    ans[i] = fp[j];
                  }
                }
              }
            }
          }
          return ans;
        }

export const linspace =
    (a: number, b: number, len: number|undefined = undefined) => {
      // if(typeof n === "undefined") n = Math.max(Math.round(b-a)+1,1);
      let n = len ?? Math.max(Math.round(b - a) + 1, 1);
      if (n < 2) {
        return n === 1 ? [ a ] : [];
      }
      const ans = new Array(n).fill(0.0);
      n--;
      for (let i = n; i >= 0; i--) {
        ans[i] = (i * b + (n - i) * a) / n;
      }
      return ans;
    }

export const norm_linspace = (len: number) => { return linspace(0, 1, len);}

export const interpolate = (y: number[], new_length: number): number[] => {
  const xold = norm_linspace(y.length);
  const xnew = norm_linspace(new_length);
  return interp(xnew, xold, y);
}
