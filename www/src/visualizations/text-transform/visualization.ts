// import seedrandom from "seedrandom";
import * as THREE from "three";
import {OrbitControls} from "three/examples/jsm/controls/OrbitControls";
import Fonts from "../../fonts";
// import {
//   AudioAnalysisResult
// } from "../../generated/proto/audio/analysis/analysis_pb";
import {map, sum} from "../../utils/functions";
// import {gaussianProb, softmax} from "../../utils/math";
import {DatGuiParameterControls} from "../controls"
import Stats from "../stats";
import {
  BaseParameterizedVisualization,
  ParameterizedVisualization,
  // UpdateParameterOptions
} from "../visualization"

import {TTFParams, TTFStartConfig} from "./parameterizer";

type TTFCharGeometry = {
  character: string; mesh : THREE.Mesh<THREE.BufferGeometry>;
  boundingBox : THREE.Box3 | null;
  width : number;
  height : number;
  positions : number[];
  transformed : number[];
  interpolated : number[];
  // colors : number[];
  pointsPerSegment : number;
};

// type WeightCenter = {
//   idx: number; amplification : number; variance : number
// };

export class TTFControls extends DatGuiParameterControls<TTFParams> {
  protected setup() {
    let parameterMenu = this.gui.addFolder("parameters");
    parameterMenu.close();
    parameterMenu.open();
    // this.gui.add(this.options, "algorithm", ["Algorithm 1"]);
    // parameterMenu.add(this.ctrl, "speed", 1, 50, 1);
    // parameterMenu.add(this.ctrl, "transformSpeed", 0, 50, 1);
    // parameterMenu.add(this.ctrl, "updateIntervalFrames", 10, 10 * 60, 30);
    // parameterMenu.add(this.ctrl, "chars");
    parameterMenu.add(this.ctrl, "spacing", 0, 20, 1)
        .listen()
        .onChange(this.didChange);
    // parameterMenu.add(this.ctrl, "weightCenters", 0, 5, 1);
    // parameterMenu.add(this.ctrl, "amplification", 0, 1, 0.1);
    parameterMenu.add(this.ctrl, "fixedWidth")
        .listen()
        .onChange(this.didChange);

    // this.gui.add(this.ctrl, "debug").listen().onChange(() => {
    //   if (this.onChange)
    //     this.onChange();
    // });
    // this.gui.close();
  }
}

export default class TTFVisualization extends
    BaseParameterizedVisualization<TTFStartConfig, any, TTFParams, TTFControls>
        implements ParameterizedVisualization<TTFStartConfig, any, TTFParams> {

  public readonly name = "Text Transform";
  protected parameters = new TTFParams();
  protected config = new TTFStartConfig();

  public get isDebug() { return super.isDebug };

  protected controls!: TTFControls;
  protected configured = false;
  protected camera?: any;
  protected scene?: any;
  protected renderer?: any;
  protected composer?: any;
  protected orbiter?: any;
  protected fontLoader = new THREE.FontLoader();
  protected text!: THREE.Group;
  protected characters!: TTFCharGeometry[];
  protected background!:
      THREE.Mesh<THREE.PlaneGeometry, THREE.MeshBasicMaterial>;

  // protected currentCharWidthFracs: number[] = [];
  // protected targetCharWidthFracs: number[] = [];
  // protected lastUpdate = 0;
  // protected weightCenters: WeightCenter[] = [];

  public frame = 0;

  renderFrame = (frame: number) => {
    if (!this.configured)
      return;
    // console.log("rendering frame");
    this.camera.lookAt(this.scene.position);
    // this.orbiter.autoRotate = true;
    // this.orbiter.autoRotateSpeed = 2;

    // if (this.orbiter.getAzimuthalAngle() >= Math.PI / 5 ||
    //     this.orbiter.getAzimuthalAngle() <= -Math.PI / 5) {
    //   this.orbiter.autoRotateSpeed *= -1;
    // }
    // console.log(this.orbiter.getAzimuthalAngle());
    this.orbiter.update();

    // if (!this.characters || !this.text)
    //   return;

    const baseCharWidths = this.characters.map((ch) => ch.width);
    const targetWidth = sum(baseCharWidths);

    this.background.material.color.set(this.parameters.backgroundColor);
    // let color = this.background.material.color;
    // const gen = seedrandom("42");
    // const gen = seedrandom((100 * Math.random()).toString());
    // gen = seedrandom(Math.floor(frame/ (0.5 * 60)).toString());

    // check if it is time to update the weight centers
    // if (frame - this.lastUpdate >=
    // this.parameters.updateIntervalFrames) { update the weight centers
    // this.weightCenters =
    //     new Array(this.parameters.weightCenters).fill(0).map((_, idx)
    //     => {
    //       return {
    //         idx : Math.random() * this.characters.length,
    //         amplification : this.parameters.amplification *
    //                             this.parameters.weightCenterAmps[idx] *
    //                             Math.random(),
    //         variance :
    //         Math.sqrt(this.parameters.weightCenterVariances[idx] *
    //                              this.characters.length),
    //       };
    //     });
    // this.targetCharWidthFracs =
    //     softmax(this.characters.map(
    //                 (_, chIdx) =>
    //                     gen() *
    //                     this.weightCenters.reduce(
    //                         (acc, center) => acc + gaussianProb(chIdx, {
    //                                                  mu : center.idx,
    //                                                  sigma :
    //                                                  center.variance,
    //                                                }) *
    //                                                center.amplification,
    //                         0)))
    //         .map((prob) => prob * this.characters.length);
    // do not transform
    // this.targetCharWidthFracs = this.targetCharWidthFracs.map(() => 1.0);

    // this.lastUpdate = frame;
    // }

    // console.log("actual", this.parameters.chars.map((c) => c.widthFrac));
    this.characters?.forEach(
        (ch, chIdx) => {
            // move the char width fraction a bit closer to the
            // target value
            // this.currentCharWidthFracs[chIdx] +=
            // (this.targetCharWidthFracs[chIdx] -
            //                                       this.currentCharWidthFracs[chIdx])
            //                                       *
            //                                      0.001 *
            //                                      this.parameters.transformSpeed;
        });

    // const currentCharWidths = this.characters.map(
    //     (ch,
    //      chIdx) => { return ch.width * this.currentCharWidthFracs[chIdx]; });
    const currentCharWidths = this.characters.map((ch, chIdx) => {
      return ch.width * this.parameters.chars[chIdx].widthFrac;
    });
    // console.log(currentCharWidths);
    const newTotalWidth = sum(currentCharWidths);
    const correction =
        this.parameters.fixedWidth ? targetWidth / newTotalWidth : 1;

    // sanity checks
    console.assert(this.characters?.every((ch, chIdx) => {
      // if (JSON.stringify(this.parameters.chars[chIdx].colors) !=
      //     JSON.stringify(this.parameters.chars[0].colors))
      //   debugger;
      if (this.parameters.chars[chIdx].colors.length !==
          4 * this.config.resolution)
        debugger;
      if (ch.positions.length / this.config.resolution !==
          3 * ch.pointsPerSegment)
        debugger;
      return true;
    }));

    let width = 0;
    let depth = 0;
    this.characters?.forEach((ch, chIdx) => {
      const segmentSize =
          this.parameters.chars[chIdx].depth / this.config.resolution;
      depth = Math.max(depth, this.parameters.chars[chIdx].depth);
      const baseSpeed = 1.0 / segmentSize;
      const colors: number[] = [];
      for (let pointIdx = ch.positions.length - 3; pointIdx >= 0;
           pointIdx -= 3) {
        const segment = Math.floor(pointIdx / (3 * ch.pointsPerSegment));
        const fsegment = this.config.resolution - segment - 1;

        let [x, y, z] = ch.positions.slice(pointIdx, pointIdx + 3);
        if (segment === 0) {
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
          // ch.transformed[pointIdx + 0] = x;
          // ch.transformed[pointIdx + 1] = y;
        } else {
          const prevPointIdx = pointIdx - 3 * ch.pointsPerSegment;
          [x, y, z] = ch.interpolated.slice(pointIdx, pointIdx + 3);
          const [xPrev, yPrev] =
              ch.interpolated.slice(prevPointIdx, prevPointIdx + 2);
          let speed =
              100 * baseSpeed *
              this.parameters.chars[chIdx].textLongitudinalVelocityFactor;
          speed = 20;
          const interp = ((frame % speed) + 1) / speed;
          // ((frame % this.parameters.speed) + 1) / this.parameters.speed;
          // const interp = 1;
          if (interp === 1) {
            ch.interpolated[pointIdx + 0] = xPrev;
            ch.interpolated[pointIdx + 1] = yPrev;
          }
          x += (xPrev - x) * interp;
          y += (yPrev - y) * interp;
        }

        z = -segment * segmentSize;
        ch.transformed[pointIdx + 0] = x;
        ch.transformed[pointIdx + 1] = y;
        ch.transformed[pointIdx + 2] = z;
        let [r, g, b, a] = this.parameters.chars[chIdx].colors.slice(
            4 * fsegment, 4 * (fsegment + 1));
        colors.push(r, g, b, a);
      }
      width += this.parameters.spacing + currentCharWidths[chIdx] * correction;

      console.assert(ch.transformed.length === ch.positions.length);
      // console.assert(colors.length === ch.transformed.length);
      ch.mesh.geometry.setAttribute(
          "position", new THREE.Float32BufferAttribute(ch.transformed, 3));

      ch.mesh.geometry.setAttribute(
          "color", new THREE.Float32BufferAttribute(colors, 4));
    });
    this.background.position.z = -(depth + 10);
    this.text.position.x = -width / 2;
    this.renderer.render(this.scene, this.camera);
  };

  init =
      (container: HTMLElement) => {
        this.destroy();
        this.container = container;

        // create renderer
        this.renderer = new THREE.WebGLRenderer({
          antialias : true,
        });

        this.scene = new THREE.Scene();
        const [near, far] = [ 0.1, 3000 ];
        this.camera = new THREE.OrthographicCamera(
            -window.innerWidth / 2, window.innerWidth / 2,
            window.innerHeight / 2, -window.innerHeight / 2, near, far);
        // this.camera = new THREE.PerspectiveCamera(
        //   90,
        //   window.innerWidth / window.innerHeight,
        //   near,
        //   far
        // );
        this.camera.position.z = 1000;
        // this.camera.position.x = 100;
        this.camera.position.y = 1000;
        this.orbiter = new OrbitControls(this.camera, this.renderer.domElement);
        // this.scene.fog = new THREE.FogExp2(0x000000, 0.001);

        let backgroundGeometry =
            // new THREE.PlaneGeometry(window.innerWidth / 2, 400, 100, 100);
            new THREE.PlaneGeometry(window.innerWidth, window.innerHeight, 1,
                                    1);
        backgroundGeometry.center();
        // let backgroundMaterial = new THREE.ShaderMaterial({
        let backgroundMaterial = new THREE.MeshBasicMaterial({
          color : new THREE.Color("black"),
          side : THREE.DoubleSide,
          // uniforms: {
          //     u_bg: {type: 'v3', value: rgb(162, 138, 241)},
          //     u_bgMain: {type: 'v3', value: rgb(162, 138, 241)},
          //     u_color1: {type: 'v3', value: rgb(162, 138, 241)},
          //     u_color2: {type: 'v3', value: rgb(82, 31, 241)},
          //     u_time: {type: 'f', value: 30},
          //     u_randomisePosition: { type: 'v2', value: randomisePosition }
          // },
          // fragmentShader: sNoise +
          // document.querySelector('#fragment-shader').textContent,
          // vertexShader: sNoise +
          // document.querySelector('#vertex-shader').textContent,
        });

        this.background =
            new THREE.Mesh(backgroundGeometry, backgroundMaterial);
        // background.position.set(-200, 270, -280);
        // background.position.set(0, 270, -280);
        this.background.scale.multiplyScalar(5);
        // background.rotationX = -1.0;
        // background.rotationY = 0.0;
        // background.rotationZ = 0.1;
        this.scene.add(this.background);

        this.renderer.setClearColor(new THREE.Color(0x000000), 1.0);
        this.renderer.setSize(window.innerWidth, window.innerHeight);
        this.container?.appendChild(this.renderer.domElement);

        // create stats
        this.stats = new Stats(this.container);
        // this.stats?.setVisible(this.parameters.debug);

        // create controls
        this.controls = new TTFControls(this.parameters, this.container);
        // this.controls.onChange =
        //     () => { this.stats?.setVisible(this.parameters.debug); };
        // this.controls?.setVisible(this.parameters.debug);
      }

  public configure = (config: TTFStartConfig): void => {
    this.config = config;
    this.configured = false;

    const fontDescriptor = Fonts[config.font];
    // debugger;
    if (!fontDescriptor) {
      throw new Error("font not found");
    }
    const font = this.fontLoader.parse(fontDescriptor.typeface);
    this.buildText(config.text,
                   {font, size : config.size, resolution : config.resolution})
        .then(({text, characters}) => {
          this.text = text;
          this.characters = characters;
          // this.currentCharWidthFracs =
          //     new Array(this.characters.length).fill(1.0);
          // this.targetCharWidthFracs =
          //     new Array(this.characters.length).fill(1.0);
          this.scene.add(this.text);
        })
        .finally(() => { this.configured = true; });
  };

  buildCharacterGeometry = (character: string, config: {
    font: THREE.Font; size : number;
    resolution?: number;
    curveSegments?: number
  }): TTFCharGeometry => {
    if (character.length !== 1)
      throw new Error("character must be of length 1");
    let textGeometry = new THREE.TextGeometry(character, {
      font : config.font,
      size : config.size,
      height : 0,
      curveSegments : config?.curveSegments ?? 1,
      bevelEnabled
      : false,
    });
    // const resolution = config.resolution ?? 40;

    textGeometry.computeBoundingBox();
    textGeometry.center();
    const vertices2d = Array.from(textGeometry.attributes.position.array);
    console.assert(vertices2d.length % 3 === 0);
    const numPoints2d = vertices2d.length / 3;
    const geometry = new THREE.BufferGeometry();
    // const color = new THREE.Color();

    const indices = [];
    const vertices3d = [];
    // const colors = [];

    // add all faces of the planar char geometry
    for (let idx = 0; idx < numPoints2d; idx++) {
      indices.push(idx);
    }

    // extend the planar character geometry
    for (let extrudeIdx = 0; extrudeIdx < this.config.resolution;
         extrudeIdx++) {
      // console.log("extrude", extrudeIdx);
      // color.setHSL(0.1 * extrudeIdx, 1.0, 0.5);
      // color = params.colors[
      for (let pointIdx = 0; pointIdx < numPoints2d; pointIdx++) {
        let [x, y, z] = vertices2d.slice(3 * pointIdx, 3 * (pointIdx + 1));
        // no need to change any vertices here
        // z -= extrudeDistance * extrudeIdx;
        // we are not allowed to change x here as then the x values might
        // not fit into the bounding box of the planar char anymore! x = x -
        // 50 * Math.sin(20 * (extrudeIdx / config.resolution));
        vertices3d.push(x, y, z);
        // colors.push(color.r, color.g, color.b);
        // colors.push(color.r, color.g, color.b);
      }
      console.assert(vertices3d.length ===
                     (extrudeIdx + 1) * (numPoints2d * 3));

      if (extrudeIdx > 0) {
        console.assert(numPoints2d % 3 === 0);
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
    // const colors = new Array(3 * numPoints2d *
    // config.resolution).fill(255); const colors = new Array(3 *
    // numPoints2d
    // * config.resolution).fill(255); geometry.setAttribute("color", new
    // THREE.Float32BufferAttribute(colors, 3));

    geometry.computeVertexNormals();
    geometry.normalizeNormals();

    let textMaterial = new THREE.MeshBasicMaterial({
      color : new THREE.Color(0xffffff),
      side : THREE.DoubleSide,
      depthTest : true,
      wireframe : false,
      vertexColors : true,
      transparent : true,
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
      // colors,
      pointsPerSegment: numPoints2d,
    };
  };

  buildText = async(text: string, config: {
    font: THREE.Font; size : number;
    resolution?: number,
    textResolution?: number,
  }): Promise<{text : THREE.Group; characters : TTFCharGeometry[];}> => {
    const group = new THREE.Group();
    const characters = [];
    for (const character of text) {
      const charGeometry = this.buildCharacterGeometry(character, {
        font : config.font,
        size : config.size,
        resolution : config.resolution,
        curveSegments : config.textResolution,
      });
      characters.push(charGeometry);
      group.add(charGeometry.mesh);
    }
    return {characters, text : group};
  };
}
