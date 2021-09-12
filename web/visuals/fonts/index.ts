import {Font} from "@disco/core/grpc";

import InterExtraBoldRegular from "./inter/Inter ExtraBold_Regular.json";
import LynoJeanRegular from "./Lyno/Lyno Jean_Regular.json";
import LynoStanRegular from "./Lyno/Lyno Stan_Regular.json";
import LynoUlysRegular from "./Lyno/Lyno Ulys_Regular.json";
import LynoWaltRegular from "./Lyno/Lyno Walt_Regular.json";
import MotoyaLMaruW3Mono from "./MotoyaLMaru/MotoyaLMaru_W3 mono.json";

export interface FontDescriptor {
  name: string;
  typeface: any;
}

export type FontGallery = Record<Font, FontDescriptor>;

const fontGallery: FontGallery = {
  [Font.MOTOYALMARU_W3_MONO] :
      {name : "MotoyaLMaru W3 Mono", typeface : MotoyaLMaruW3Mono},
  [Font.LYNO_WALT_REGULAR] :
      {name : "Lyno Walt Regular", typeface : LynoWaltRegular},
  [Font.LYNO_JEAN_REGULAR] :
      {name : "Lyno Jean Regular", typeface : LynoJeanRegular},
  [Font.LYNO_STAN_REGULAR] :
      {name : "Lyno Stan Regular", typeface : LynoStanRegular},
  [Font.LYNO_ULYS_REGULAR] :
      {name : "Lyno Ulys Regular", typeface : LynoUlysRegular},
  [Font.INTER_EXTRA_BOLD_REGULAR] :
      {name : "Inter Extra Bold Regular", typeface : InterExtraBoldRegular},
}

export default fontGallery;
export {
  LynoUlysRegular,
  MotoyaLMaruW3Mono,
  InterExtraBoldRegular,
  LynoWaltRegular,
  LynoStanRegular,
  LynoJeanRegular,
};
