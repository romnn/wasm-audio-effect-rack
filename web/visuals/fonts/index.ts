import InterExtraBoldRegular from "./inter/Inter ExtraBold_Regular.json";
import LynoJeanRegular from "./Lyno/Lyno Jean_Regular.json";
import LynoStanRegular from "./Lyno/Lyno Stan_Regular.json";
import LynoUlysRegular from "./Lyno/Lyno Ulys_Regular.json";
import LynoWaltRegular from "./Lyno/Lyno Walt_Regular.json";
import MotoyaLMaruW3Mono from "./MotoyaLMaru/MotoyaLMaru_W3 mono.json";

export interface Font {
  name: string;
  typeface: any;
}

export interface FontGallery {
  [key: string]: Font;
}

const fontGallery: FontGallery = {
  "motoyalmaruw3mono" :
      {name : "MotoyaLMaru W3 Mono", typeface : MotoyaLMaruW3Mono},
  "lynowaltregular" : {name : "Lyno Walt Regular", typeface : LynoWaltRegular},
  "lynojeanregular" : {name : "Lyno Jean Regular", typeface : LynoJeanRegular},
  "lynostanregular" : {name : "Lyno Stan Regular", typeface : LynoStanRegular},
  "lynoulysregular" : {name : "Lyno Ulys Regular", typeface : LynoUlysRegular},
  "interextraboldregular" :
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
