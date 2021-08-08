export const gaussianFilter1d = (arr: number[], sigma: number): number[] => {
  const out = new Array(arr.length).fill(0.0);
  gaussBlur_4(arr, out, arr.length, 1, sigma)
  return out;
};

export const boxesForGauss = (sigma: number, n: number):
    number[] => {
      const wIdeal = Math.sqrt((12 * sigma * sigma / n) + 1);
      let wl = Math.floor(wIdeal);
      if (wl % 2 == 0)
        wl--;
      const wu = wl + 2;

      const mIdeal = (12 * sigma * sigma - n * wl * wl - 4 * n * wl - 3 * n) /
                     (-4 * wl - 4);
      const m = Math.round(mIdeal);

      const sizes = [];
      for (let i = 0; i < n; i++)
        sizes.push(i < m ? wl : wu);
      return sizes;
    }

export const gaussBlur_4 =
    (scl: number[], tcl: number[], w: number, h: number, r: number):
        void => {
          const bxs = boxesForGauss(r, 3);
          boxBlur_4(scl, tcl, w, h, (bxs[0] - 1) / 2);
          boxBlur_4(tcl, scl, w, h, (bxs[1] - 1) / 2);
          boxBlur_4(scl, tcl, w, h, (bxs[2] - 1) / 2);
        }

export const boxBlur_4 =
    (scl: number[], tcl: number[], w: number, h: number, r: number):
        void => {
          for (let i = 0; i < scl.length; i++)
            tcl[i] = scl[i];
          boxBlurH_4(tcl, scl, w, h, r);
          boxBlurT_4(scl, tcl, w, h, r);
        }

export const boxBlurH_4 =
    (scl: number[], tcl: number[], w: number, h: number, r: number):
        void => {
          const iarr = 1 / (r + r + 1);
          for (let i = 0; i < h; i++) {
            let ti = i * w;
            let li = ti;
            let ri = ti + r;
            const fv = scl[ti];
            const lv = scl[ti + w - 1];
            let val = (r + 1) * fv;
            for (let j = 0; j < r; j++)
              val += scl[ti + j];
            for (let j = 0; j <= r; j++) {
              val += scl[ri++] - fv;
              tcl[ti++] = Math.round(val * iarr);
            }
            for (let j = r + 1; j < w - r; j++) {
              val += scl[ri++] - scl[li++];
              tcl[ti++] = Math.round(val * iarr);
            }
            for (let j = w - r; j < w; j++) {
              val += lv - scl[li++];
              tcl[ti++] = Math.round(val * iarr);
            }
          }
        }

export const boxBlurT_4 =
    (scl: number[], tcl: number[], w: number, h: number, r: number): void => {
      const iarr = 1 / (r + r + 1);
      for (let i = 0; i < w; i++) {
        let ti = i;
        let li = ti;
        let ri = ti + r * w;
        const fv = scl[ti];
        const lv = scl[ti + w * (h - 1)];
        let val = (r + 1) * fv;
        for (let j = 0; j < r; j++)
          val += scl[ti + j * w];
        for (let j = 0; j <= r; j++) {
          val += scl[ri] - fv;
          tcl[ti] = Math.round(val * iarr);
          ri += w;
          ti += w;
        }
        for (let j = r + 1; j < h - r; j++) {
          val += scl[ri] - scl[li];
          tcl[ti] = Math.round(val * iarr);
          li += w;
          ri += w;
          ti += w;
        }
        for (let j = h - r; j < h; j++) {
          val += lv - scl[li];
          tcl[ti] = Math.round(val * iarr);
          li += w;
          ti += w;
        }
      }
    }
