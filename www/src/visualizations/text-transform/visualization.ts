// import typefaceFont from "@/fonts/Inter ExtraBold_Regular.json";
import seedrandom from "seedrandom";
import * as THREE from "three";
import {OrbitControls} from "three/examples/jsm/controls/OrbitControls";

import typefaceFont from "../../fonts/Inter ExtraBold_Regular.json";
import {gaussianProb, map, softmax, sum} from "../../utils";
import {DatGuiParameterControls} from "../controls"
import Stats from "../stats";
import {
  BaseVisualization,
  ParameterizedVisualization,
  UpdateParameterOptions
} from "../visualization"

import {TTFParams} from "./parameterizer";

const resolution = 30;

type TTFCharGeometry = {
  character: string; mesh : THREE.Mesh<THREE.BufferGeometry>;
  boundingBox : THREE.Box3 | null;
  width : number;
  height : number;
  positions : number[];
  transformed : number[];
  interpolated : number[];
  colors : number[];
  pointsPerSegment : number;
};

type WeightCenter = {
  idx: number; amplification : number; variance : number
};

export class TTFControls extends DatGuiParameterControls<TTFParams> {

  protected setup() {
    let parameterMenu = this.gui.addFolder("parameters");
    parameterMenu.close();
    parameterMenu.open();
    // this.gui.add(this.options, "algorithm", ["Algorithm 1"]);
    parameterMenu.add(this.ctrl, "speed", 1, 50, 1);
    parameterMenu.add(this.ctrl, "transformSpeed", 0, 50, 1);
    parameterMenu.add(this.ctrl, "updateIntervalFrames", 10, 10 * 60, 30);
    parameterMenu.add(this.ctrl, "spacing", 0, 20, 1);
    parameterMenu.add(this.ctrl, "weightCenters", 0, 5, 1);
    parameterMenu.add(this.ctrl, "amplification", 0, 1, 0.1);
    parameterMenu.add(this.ctrl, "fixedWidth");

    this.gui.add(this.ctrl, "debug").listen().onChange(() => {
      if (this.onChange)
        this.onChange();
    });
    // this.gui.close();
  }
}

// BaseVisualization<TTFParameterControls<TTFParameters>, TTFParameters>
// implements

export default class TTFVisualization extends
    BaseVisualization<TTFParams, TTFControls> implements
        ParameterizedVisualization<TTFParams> {
  public readonly name = "Text Transform";

  public get isRunning() { return super.isRunning };
  public get isDebug() { return super.isDebug };
  public get parameters() { return super.parameters };

  protected params = new TTFParams();
  protected controls!: TTFControls;

  protected camera?: any;
  protected scene?: any;
  protected renderer?: any;
  protected composer?: any;
  protected orbiter?: any;
  protected fontLoader = new THREE.FontLoader();
  protected text!: THREE.Group;
  protected characters!: TTFCharGeometry[];
  protected currentCharWidthFracs: number[] = [];
  protected targetCharWidthFracs: number[] = [];
  protected lastUpdate = 0;
  protected weightCenters: WeightCenter[] = [];

  updateParameters(parameters: TTFParams, options?: UpdateParameterOptions) {
    console.log("updating parameters to", parameters);
  }

  getParameters(): TTFParams { return this.params; }

  renderFrame = () => {
    console.log("frame");
    this.camera.lookAt(this.scene.position);
    if (!this.characters || !this.text)
      return;

    const baseCharWidths = this.characters.map((ch) => ch.width);
    const targetWidth = sum(baseCharWidths);
    // const gen = seedrandom("42");
    const gen = seedrandom((100 * Math.random()).toString());
    // gen = seedrandom(Math.floor(this.frameCount/ (0.5 * 60)).toString());

    // check if it is time to update the weight centers
    if (this.frameCount - this.lastUpdate >= this.params.updateIntervalFrames) {
      // update the weight centers
      this.weightCenters =
          new Array(this.params.weightCenters).fill(0).map((_, idx) => {
            return {
              idx : Math.random() * this.characters.length,
              amplification : this.params.amplification *
                                  this.params.weightCenterAmps[idx] *
                                  Math.random(),
              variance : Math.sqrt(this.params.weightCenterVariances[idx] *
                                   this.characters.length),
            };
          });
      this.targetCharWidthFracs =
          softmax(this.characters.map(
                      (_, chIdx) =>
                          gen() *
                          this.weightCenters.reduce(
                              (acc, center) => acc + gaussianProb(chIdx, {
                                                       mu : center.idx,
                                                       sigma : center.variance,
                                                     }) * center.amplification,
                              0)))
              .map((prob) => prob * this.characters.length);
      // do not transform
      // this.targetCharWidthFracs = this.targetCharWidthFracs.map(() => 1.0);

      this.lastUpdate = this.frameCount;
    }

    this.characters?.forEach((ch, chIdx) => {
      // move the char width fraction a bit closer to the target value
      this.currentCharWidthFracs[chIdx] += (this.targetCharWidthFracs[chIdx] -
                                            this.currentCharWidthFracs[chIdx]) *
                                           0.001 * this.params.transformSpeed;
    });

    const currentCharWidths = this.characters.map(
        (ch,
         chIdx) => { return ch.width * this.currentCharWidthFracs[chIdx]; });
    const newTotalWidth = sum(currentCharWidths);
    const correction = this.params.fixedWidth ? targetWidth / newTotalWidth : 1;
    const extrudeDistance = this.params.depth / resolution;

    let width = 0;
    this.characters?.forEach((ch, chIdx) => {
      console.assert(ch.positions.length == ch.colors.length);
      console.assert(ch.positions.length / resolution ==
                     3 * ch.pointsPerSegment);
      for (let pointIdx = ch.positions.length - 3; pointIdx >= 0;
           pointIdx -= 3) {
        const segment = Math.floor(pointIdx / (3 * ch.pointsPerSegment));
        let [x, y, z] = ch.positions.slice(pointIdx, pointIdx + 3);
        if (segment == 0) {
          const valid =
            (ch.boundingBox?.min?.x ?? 0) <= x &&
            x <= (ch.boundingBox?.max?.x ?? 0);
          console.assert(valid);

          const charPosFrac = map(
            x,
            ch.boundingBox?.min?.x ?? 0,
            ch.boundingBox?.max?.x ?? 0,
            0,
            1
          );
          x = width + currentCharWidths[chIdx] * charPosFrac * correction;
          ch.interpolated[pointIdx + 0] = x;
          ch.interpolated[pointIdx + 1] = y;
        } else {
          const prevPointIdx = pointIdx - 3 * ch.pointsPerSegment;
          [x, y, z] = ch.interpolated.slice(pointIdx, pointIdx + 3);
          const [xPrev, yPrev, zPrev] =
              ch.interpolated.slice(prevPointIdx, prevPointIdx + 3);
          const interp =
              ((this.frameCount % this.params.speed) + 1) / this.params.speed;
          if (interp == 1) {
            ch.interpolated[pointIdx + 0] = xPrev;
            ch.interpolated[pointIdx + 1] = yPrev;
          }
          x += (xPrev - x) * interp;
          y += (yPrev - y) * interp;
        }

        z = -segment * extrudeDistance;
        ch.transformed[pointIdx + 0] = x;
        ch.transformed[pointIdx + 1] = y;
        ch.transformed[pointIdx + 2] = z;
        // ch.colors[pointIdx + 0] = 255;
        // ch.colors[pointIdx + 1] = 255;
        // ch.colors[pointIdx + 2] = 255;
      }
      width += this.params.spacing + currentCharWidths[chIdx] * correction;

      ch.mesh.geometry.setAttribute(
          "position", new THREE.Float32BufferAttribute(ch.transformed, 3));
      ch.mesh.geometry.setAttribute(
          "color", new THREE.Float32BufferAttribute(ch.colors, 3));
    });
    this.text.position.x = -width / 2;
    this.renderer.render(this.scene, this.camera);
  };

  init = (container: HTMLElement) => {
    this.container = container;
    this.camera = new THREE.OrthographicCamera(
        -window.innerWidth / 2, window.innerWidth / 2, window.innerHeight / 2,
        -window.innerHeight / 2, 0.1, 3000);
    // this.camera = new THREE.PerspectiveCamera(
    //   90,
    //   window.innerWidth / window.innerHeight,
    //   1,
    //   2000
    // );
    this.camera.position.z = 1000;
    // this.camera.position.x = 100;
    this.camera.position.y = 1000;
    this.scene = new THREE.Scene();
    // this.scene.fog = new THREE.FogExp2(0x000000, 0.001);

    // Setup renderer and effects
    this.renderer = new THREE.WebGLRenderer({
      antialias : true,
    });
    this.renderer.setClearColor(new THREE.Color(0x000000), 1.0);
    this.renderer.setSize(window.innerWidth, window.innerHeight);

    const font = this.fontLoader.parse(typefaceFont);
    this.buildText("ADRIANA", {font, size : 80, resolution})
        .then(({text, characters}) => {
          this.text = text;
          this.characters = characters;
          this.currentCharWidthFracs =
              new Array(this.characters.length).fill(1.0);
          this.targetCharWidthFracs =
              new Array(this.characters.length).fill(1.0);
          this.scene.add(this.text);
        });
    this.container?.appendChild(this.renderer.domElement);

    this.stats = new Stats(this.container);
    this.stats?.setVisible(true);

    // this.controls = new TTFControls();
    this.controls = new TTFControls(this.params, this.container);
    // this.controls.init(this.params, this.container);
    this.controls.onChange =
        () => { this.stats?.setVisible(this.params.debug); };

    this.orbiter = new OrbitControls(this.camera, this.renderer.domElement);

    this.animate();
  };

  buildCharacterGeometry = (character: string, config: {
    font: THREE.Font; size : number; resolution : number;
  },
                            options
                            ?: {curveSegments?: number}): TTFCharGeometry => {
    if (character.length != 1)
      throw new Error("character must be of length 1");
    let textGeometry = new THREE.TextGeometry(character, {
      font : config.font,
      size : config.size,
      height : 0,
      curveSegments : options?.curveSegments ?? 1,
      bevelEnabled
      : false,
    });

    textGeometry.computeBoundingBox();
    textGeometry.center();
    const vertices2d = Array.from(textGeometry.attributes.position.array);
    console.assert(vertices2d.length % 3 == 0);
    const numPoints2d = vertices2d.length / 3;
    const geometry = new THREE.BufferGeometry();
    const color = new THREE.Color();

    const indices = [];
    const vertices3d = [];
    const colors = [];

    // add all faces of the planar char
    for (let idx = 0; idx < numPoints2d; idx++) {
      indices.push(idx);
    }

    // extend the planar char
    for (let extrudeIdx = 0; extrudeIdx < config.resolution; extrudeIdx++) {
      // console.log("extrude", extrudeIdx);
      color.setHSL(0.1 * extrudeIdx, 1.0, 0.5);
      for (let pointIdx = 0; pointIdx < numPoints2d; pointIdx++) {
        let [x, y, z] = vertices2d.slice(3 * pointIdx, 3 * (pointIdx + 1));
        // no need to change any vertices here
        // z -= extrudeDistance * extrudeIdx;
        // we are not allowed to change x here as then the x values might not
        // fit into the bounding box of the planar char anymore! x = x - 50 *
        // Math.sin(20 * (extrudeIdx / config.resolution));
        vertices3d.push(x, y, z);
        colors.push(color.r, color.g, color.b);
      }
      console.assert(vertices3d.length == (extrudeIdx + 1) * (numPoints2d * 3));

      if (extrudeIdx > 0) {
        console.assert(numPoints2d % 3 == 0);
        const numFaces = numPoints2d / 3;

        for (let faceIdx = 0; faceIdx < numFaces; faceIdx++) {
          // console.log("face", faceIdx);
          const triangle = [ 0, 1, 2, 0 ];
          for (let pointIdx = 0; pointIdx < 3; pointIdx++) {
            const leftP = (extrudeIdx - 1) * numPoints2d + 3 * faceIdx +
                          triangle[pointIdx];
            const leftPprev = (extrudeIdx - 1) * numPoints2d + 3 * faceIdx +
                              triangle[pointIdx + 1];
            const rightP = (extrudeIdx + 0) * numPoints2d + 3 * faceIdx +
                           triangle[pointIdx];
            const rightPprev = (extrudeIdx + 0) * numPoints2d + 3 * faceIdx +
                               triangle[pointIdx + 1];
            const valid =
                [ leftP, leftPprev, rightP, rightPprev ].every((idx) => {
                  return ((extrudeIdx - 1) * numPoints2d <= idx &&
                          idx < (extrudeIdx + 1) * numPoints2d);
                });
            console.assert(valid);
            indices.push(leftP, leftPprev, rightPprev); // face one
            indices.push(leftP, rightPprev, rightP);    // face two
          }
        }
      }
    }

    geometry.setIndex(Array.from(indices));
    geometry.setAttribute("position",
                          new THREE.Float32BufferAttribute(vertices3d, 3));
    geometry.setAttribute("color", new THREE.Float32BufferAttribute(colors, 3));

    geometry.computeVertexNormals();

    let textMaterial = new THREE.MeshBasicMaterial({
      color : new THREE.Color(0xffffff),
      depthTest : true,
      wireframe : false,
      vertexColors : true,
    });

    const text = new THREE.Mesh(geometry, textMaterial);
    return {
      character,
      mesh: text,
      boundingBox: textGeometry.boundingBox,
      width:
        (textGeometry.boundingBox?.max?.x ?? 0) -
        (textGeometry.boundingBox?.min?.x ?? 0),
      height:
        (textGeometry.boundingBox?.max?.y ?? 0) -
        (textGeometry.boundingBox?.min?.y ?? 0),
      positions: vertices3d,
      transformed: new Array(vertices3d.length).fill(0.0),
      interpolated: new Array(vertices3d.length).fill(0.0),
      colors,
      pointsPerSegment: numPoints2d,
    };
  };

  buildText = async(text: string, config: {
    font: THREE.Font; size : number; resolution : number
  }): Promise<{text : THREE.Group; characters : TTFCharGeometry[];}> => {
    const group = new THREE.Group();
    const characters = [];
    for (const character of text) {
      const charGeometry = this.buildCharacterGeometry(character, {
        font: config.font,
        size: config.size,
        resolution: config.resolution ?? 10,
      });
      characters.push(charGeometry);
      group.add(charGeometry.mesh);
    }
    return {characters, text : group};
  };
}
