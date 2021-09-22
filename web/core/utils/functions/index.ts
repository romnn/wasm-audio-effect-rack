import {
  Color,
  HSLColor,
  RGBColor,
} from "@disco/core/grpc";
import * as THREE from "three";

export const HSLToRGB =
    (h: number, s: number, l: number): [ number, number, number ] => {
      const a = s * Math.min(l, 1 - l);
      const f = (n: number, k = (n + h / 30) % 12) =>
          l - a * Math.max(Math.min(k - 3, 9 - k, 1), -1);
      return [ f(0), f(8), f(4) ];
    };

export const rgb = (r: number, g: number, b: number):
    Color => {
      const color = new Color();
      const rgbColor = new RGBColor();
      rgbColor.setR(r);
      rgbColor.setG(g);
      rgbColor.setB(b);
      color.setRgb(rgbColor);
      return color;
    }

export const threeColor =
    <C extends Color>(color: C): THREE.Color => {
      const rgbColor = color.getRgb();
      const hslColor = color.getHsl();
      let result = new THREE.Color("black");
      if (rgbColor) {
        result =
            new THREE.Color(rgbColor.getR(), rgbColor.getG(), rgbColor.getB());
      }
      if (hslColor) {
        let [r, g, b] =
            HSLToRGB(hslColor.getH(), hslColor.getS(), hslColor.getL());
        result = new THREE.Color(r, g, b);
      }
      return result;
    }

export const clip = (value: number, lower: number, upper: number): number =>
    Math.max(Math.min(value, upper), lower);

export const map =
    (value: number, x1: number, y1: number, x2: number, y2: number): number =>
        (value - x1) * (y2 - x2) / (y1 - x1) + x2;

export const sum = (arr: number[]): number =>
    arr.reduce((acc, val) => acc + val, 0);

export const mean = (arr: number[]): number => { return sum(arr) / arr.length;}

export const stddev = (arr: number[]): [ number, number ] => {
  const m = mean(arr);
  return [
    Math.sqrt(arr.reduce((acc, x) => acc.concat((x - m) ** 2), [] as number[])
                  .reduce((acc, x) => acc + x, 0) /
              arr.length),
    m
  ];
}

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
