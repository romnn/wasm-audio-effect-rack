import React from "react";
import * as THREE from "three";
import Stats from "@disco/visuals/stats";
import dat from "dat.gui";
import spriteTexture from "./george_face.png";
import { HSLToRGB } from "@disco/core/utils/functions";
import { mod } from "@disco/core/utils/math";
// import BPMDetection from "./nodes/bpm-detection";
import { RouteComponentProps } from "react-router-dom";

const clock = new THREE.Clock();
let delta = 0;
let maxFPS = 1 / 35;
const targetFPS = 1 / 60;
const limitFPS = true;

interface FractalOrbitConstraints {
  aMin?: number;
  aMax?: number;
  bMin?: number;
  bMax?: number;
  cMin?: number;
  cMax?: number;
  dMin?: number;
  dMax?: number;
  eMin?: number;
  eMax?: number;
  spriteSize?: number;
}

interface FractalOrbitParameters {
  a?: number;
  b?: number;
  c?: number;
  d?: number;
  e?: number;
  spriteSize?: number;
  speed?: number;
  rotationSpeed?: number;
}

interface FractalProps extends FractalOrbitConstraints, FractalOrbitParameters {
  visible?: boolean;
  showFps?: boolean;
  scaleFactor?: number;
  cameraBound?: number;
  numPointsPerSubset?: number;
  numSubsets?: number;
  numLevels?: number;
  levelDepth?: number;
  defBrightness?: number;
  defSaturation?: number;
  hueValues?: number[];
  chaosEnabled?: boolean;
}

interface FractalOrbit {
  xMin: number;
  xMax: number;
  yMin: number;
  yMax: number;
  scaleX: number;
  scaleY: number;
}

interface FractalState {
  // mouseX: number;
  // mouseY: number;
  // windowHalfX: number;
  // windowHalfY: number;
}

export class FractalParameters implements FractalProps {
  private isReactive = false;

  visible = true;
  enableDebug = true;
  //im default value: 1500,
  scaleFactor = 1500;
  //im default value: 200
  cameraBound = 400;
  numPointsPerSubset = 2000;
  numSubsets = 7;
  //im default value: 5
  numLevels = 4;
  //im default value: 400,
  levelDepth = 800;
  //im default value: 1
  defBrightness = 0.8;
  defSaturation = 0.8;
  // need hue value for each subset
  // hueValues: [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7];
  hueValues: number[] = [];
  spriteSize = (3 * window.innerWidth) / this.scaleFactor;
  // spriteSizeMin = Math.ceil((3 * window.innerWidth) / this.scaleFactor) * 0.4;
  // spriteSizeMax = Math.ceil((3 * window.innerWidth) / this.scaleFactor) * 1.2;
  // im added chaos mode boolean
  chaosEnabled = true;

  // orbit parameters constraints
  aMin = -30;
  aMax = 30;
  bMin = 0.2;
  bMax = 1.8;
  cMin = 5;
  cMax = 17;
  dMin = 0;
  dMax = 10;
  eMin = 0;
  eMax = 12;

  // orbit parameters
  a = 0;
  b = 0;
  c = 0;
  d = 0;
  e = 0;
  speed = 5;
  rotationSpeed = 0.001;

  // react = () => {
  //   console.log("reacting to audio now");
  //   if (this.isReactive) return;
  //   const ctx = new window.AudioContext();
  //   const audio = new Audio("mars_venus.mp3");
  //   audio.autoplay = true;
  //   audio.loop = true;
  //   // audio.muted = true;
  //   const source = ctx.createMediaElementSource(audio);
  //   const bpmDetector = new BPMDetection(
  //     ctx,
  //     "bpm-detection-node-processor",
  //     undefined,
  //     {
  //       onInitialized: (inst: BPMDetection) => {
  //         bpmDetector.onBPMChanged = (bpm: number) => {
  //           console.log("bpm changed to ", bpm);
  //         };
  //         source.connect(bpmDetector.workletHandle!);
  //         // bpmDetector.workletHandle!.connect(ctx.destination);
  //         source.connect(ctx.destination);
  //       },
  //     }
  //   );
  //   this.isReactive = true;
  // };
}

export class FractalControls<T> {
  container: HTMLElement;
  gui: dat.GUI;
  ctrl: T;
  onChange?: () => void;

  update = () => {
    this.gui.updateDisplay();
  };

  constructor(opts: T, container?: HTMLElement) {
    this.ctrl = opts;
    this.gui = new dat.GUI({ autoPlace: false });

    let orbitParameterMenu = this.gui.addFolder("orbit parameters");
    orbitParameterMenu.close();
    // this.gui.add(this.options, "algorithm", [
    //   "Algorithm 1",
    //   "Algorithm 2",
    //   "Algorithm 3",
    // ]);
    [
      "aMin",
      "aMax",
      "bMin",
      "bMax",
      "cMin",
      "cMax",
      "dMin",
      "dMax",
      "eMin",
      "eMax",
    ].forEach((p) => orbitParameterMenu.add(this.ctrl, p, -30, 30, 1));

    let appearanceMenu = this.gui.addFolder("appearance");
    appearanceMenu.open();
    ["defBrightness", "defSaturation"].forEach((p) =>
      appearanceMenu.add(this.ctrl, p, 0, 1.0, 0.05)
    );
    appearanceMenu.add(this.ctrl, "speed", 0, 50, 1);
    appearanceMenu.add(this.ctrl, "rotationSpeed", -0.1, 0.1, 0.01);

    let fractalParameterMenu = this.gui.addFolder("fractal parameters");
    fractalParameterMenu.close();
    ["a", "b", "c", "d", "e"].forEach((p) =>
      fractalParameterMenu.add(this.ctrl, p)
    );

    // this.gui.add(this.ctrl, "react");
    this.gui
      .add(this.ctrl, "enableDebug")
      .listen()
      .onChange(() => {
        if (this.onChange) this.onChange();
      });
    // this.gui.close();

    this.container = document.createElement("div");
    this.container.style.position = "fixed";
    this.container.style.top = "0";
    this.container.style.backgroundColor = "black";
    this.container.style.right = "0";
    this.container.appendChild(this.gui.domElement);
    (container ?? document.body).appendChild(this.container);
  }
}

interface OrbitPositions {
  colors: Float32Array;
  positions: Float32Array;
  normalized: Float32Array;
}

interface INavProps {
  debug?: string;
}

type FractalPropsAndParams = FractalProps & RouteComponentProps<INavProps>;

export default class Fractal extends React.Component<
  FractalPropsAndParams,
  FractalState
> {
  private stats?: Stats;
  private params = new FractalParameters();
  private controls?: FractalControls<FractalParameters>;

  private container?: any;
  private camera?: any;
  private scene?: any;
  private renderer?: any;
  private composer?: any;

  private textureLoader = new THREE.TextureLoader();

  private orbit: FractalOrbit = {
    xMin: 0,
    xMax: 0,
    yMin: 0,
    yMax: 0,
    scaleX: 0,
    scaleY: 0,
  };
  // private subsets: OrbitSubsetPoint[][] = [];
  // private needsUpdate: boolean[] = [];
  private subsetPositions: OrbitPositions[] = [];
  private particles: {
    points: THREE.Points;
    level: number;
    subset: number;
    material: THREE.PointsMaterial;
  }[] = [];

  // initializeOrbitSubsets = (
  //   numSubsets: number,
  //   numPointsPerSubnet: number
  // ): OrbitSubsetPoint[][] => {
  //   let subsets = [];
  //   for (let i = 0; i < (numSubsets ?? 0); i++) {
  //     let points = [];
  //     for (let j = 0; j < (numPointsPerSubnet ?? 0); j++) {
  //       points.push({
  //         x: 0,
  //         y: 0,
  //         vertex: new THREE.Vector3(0, 0, 0),
  //       });
  //     }
  //     subsets.push(points);
  //   }
  //   return subsets;
  // };

  constructor(props: FractalPropsAndParams) {
    super(props);
    this.state = {
      // mouseX: 0,
      // mouseY: 0,
      // windowHalfX: window.innerWidth / 2,
      // windowHalfY: window.innerHeight / 2,
    };
    // this.needsUpdate = new Array(this.params.numSubsets).fill(false);
    // console.log(this.params);
    this.subsetPositions = new Array(this.params.numSubsets).fill(0).map(() => {
      return {
        colors: new Float32Array(3 * this.params.numPointsPerSubset).fill(0.0),
        positions: new Float32Array(3 * this.params.numPointsPerSubset).fill(
          0.0
        ),
        normalized: new Float32Array(3 * this.params.numPointsPerSubset).fill(
          0.0
        ),
      };
    });
    // console.log(this.subsetPositions);
    // this.subsets = this.initializeOrbitSubsets(
    //   this.params.numSubsets,
    //   this.params.numPointsPerSubset
    // );
  }

  animate = () => {
    requestAnimationFrame(this.animate);
    delta += clock.getDelta();
    if (!limitFPS || delta > maxFPS) {
      this.renderFrame();
      this.stats?.update();
      delta = delta % maxFPS;
    }
  };

  renderFrame = () => {
    /* IM - Mouse Driven Camera Positioning (temporarily removed)                  
    if (camera.position.x >= - CAMERA_BOUND && camera.position.x <= CAMERA_BOUND){
        camera.position.x += ( mouseX - camera.position.x ) * 0.05;
        if (camera.position.x < - CAMERA_BOUND) camera.position.x = -CAMERA_BOUND;
        if (camera.position.x >  CAMERA_BOUND) camera.position.x = CAMERA_BOUND;
    }
    if (camera.position.y >= - CAMERA_BOUND && camera.position.y <= CAMERA_BOUND){
        camera.position.y += ( - mouseY - camera.position.y ) * 0.05;
        if (camera.position.y < - CAMERA_BOUND) camera.position.y = -CAMERA_BOUND;
        if (camera.position.y >  CAMERA_BOUND) camera.position.y = CAMERA_BOUND;
    }
    */

    this.camera.lookAt(this.scene.position);

    const speedup = maxFPS / Math.min(maxFPS, targetFPS);
    // console.log("speedup", speedup);

    // this.params.speed += this.randomNum(-1, 1) * 0.1;
    this.params.rotationSpeed += this.randomNum(-1, 1) * 0.00001;

    // console.log(this.particles.length);
    this.particles.forEach(({ points, material, level, subset }) => {
      // console.log(subset, points.position.z);
      // console.log(points.position.z);
      points.position.z += this.params.speed * speedup;
      points.rotation.z += this.params.rotationSpeed * speedup;
      // let currentColor = { h: 0, s: 0, l: 0 };
      // material.color.getHSL(currentColor);
      // let hue = currentColor.h;

      // update the colors
      let colors = this.subsetPositions[subset].colors;
      for (let i = 0; i < colors.length; i = i + 3) {
        // let [r, g, b] = hslToRGB(colors[i], colors[i], colors[i]);
        let [r, g, b] = HSLToRGB(
          mod(subset * 30, 360),
          this.params.defSaturation,
          this.params.defBrightness
        );
        colors[i] = r;
        colors[i + 1] = g;
        colors[i + 2] = b;
      }
      points.geometry.setAttribute(
        "color",
        new THREE.Float32BufferAttribute(
          colors,
          // this.subsetPositions[subset].colors,
          3
        )
      );
      points.geometry.attributes.color.needsUpdate = true;
      points.geometry.attributes.color.needsUpdate = true;

      if (points.position.z > this.camera.position.z) {
        // hue = this.params.hueValues[subset];
        points.geometry.setAttribute(
          "position",
          new THREE.Float32BufferAttribute(
            this.subsetPositions[subset].normalized,
            3
          )
        );

        points.position.z = -(
          (this.params.numLevels - 1) *
          this.params.levelDepth
        );
      }
      // material.color.setHSL(
      //   hue,
      //   this.params.defSaturation,
      //   this.params.defBrightness
      // );
    });

    this.renderer.render(this.scene, this.camera);
  };

  randomNum = (min: number, max: number): number => {
    return Math.floor(Math.random() * (max - min + 1)) + min;
  };

  updateOrbit = () => {
    this.generateOrbit();
    this.controls?.update();
  };

  generateOrbit = () => {
    let x, y, z, x1;
    // reset the orbit parameters and randomize the parameters
    this.prepareOrbit();

    console.assert(
      this.params.numSubsets === (this.subsetPositions?.length ?? 0)
    );

    for (let s = 0; s < this.params.numSubsets; s++) {
      // Use a different starting point for each orbit subset
      x = s * 0.005 * (0.5 - Math.random());
      y = s * 0.005 * (0.5 - Math.random());

      for (let i = 0; i < this.params.numPointsPerSubset; i++) {
        // Iteration formula (generalization of the Barry Martin's original one)
        z =
          this.params.d +
          Math.sqrt(Math.abs(this.params.b * x - this.params.c));

        // todo: this is veryy bad please refactor
        if (x > 0) {
          x1 = y - z;
        } else if (x === 0) {
          x1 = y;
        } else {
          x1 = y + z;
        }
        y = this.params.a - x;
        x = x1 + this.params.e;

        this.subsetPositions[s].positions[3 * i + 0] = x;
        this.subsetPositions[s].positions[3 * i + 1] = y;

        if (x < this.orbit.xMin) {
          this.orbit.xMin = x;
        } else if (x > this.orbit.xMax) {
          this.orbit.xMax = x;
        }
        if (y < this.orbit.yMin) {
          this.orbit.yMin = y;
        } else if (y > this.orbit.yMax) {
          this.orbit.yMax = y;
        }
      }
    }

    this.orbit.scaleX =
      (2 * this.params.scaleFactor) / (this.orbit.xMax - this.orbit.xMin);
    this.orbit.scaleY =
      (2 * this.params.scaleFactor) / (this.orbit.yMax - this.orbit.yMin);

    // Normalize and update vertex data
    for (let s = 0; s < this.params.numSubsets; s++) {
      for (let i = 0; i < this.params.numPointsPerSubset; i++) {
        this.subsetPositions[s].normalized[3 * i + 0] =
          this.orbit.scaleX *
            (this.subsetPositions[s].positions[3 * i + 0] - this.orbit.xMin) -
          this.params.scaleFactor;
        this.subsetPositions[s].normalized[3 * i + 1] =
          this.orbit.scaleY *
            (this.subsetPositions[s].positions[3 * i + 1] - this.orbit.yMin) -
          this.params.scaleFactor;
      }
    }
  };

  prepareOrbit = () => {
    this.shuffleParams();
    this.params.hueValues = new Array(this.params.numSubsets)
      .fill(0)
      .map((v) => Math.random());
    this.orbit.xMin = 0;
    this.orbit.xMax = 0;
    this.orbit.yMin = 0;
    this.orbit.yMax = 0;
  };

  shuffleParams = () => {
    this.params.a =
      this.params.aMin + Math.random() * (this.params.aMax - this.params.aMin);
    this.params.b =
      this.params.bMin + Math.random() * (this.params.bMax - this.params.bMin);
    this.params.c =
      this.params.cMin + Math.random() * (this.params.cMax - this.params.cMin);
    this.params.d =
      this.params.dMin + Math.random() * (this.params.dMax - this.params.dMin);
    this.params.e =
      this.params.eMin + Math.random() * (this.params.eMax - this.params.eMin);
  };

  init = () => {
    const texture = this.textureLoader.load(spriteTexture, (tex) => {
      // texure can be inspected here
      // console.log("texture size is", tex.image.width, tex.image.height);
    });
    this.container = document.getElementById("Fractal");
    this.camera = new THREE.PerspectiveCamera(
      90,
      window.innerWidth / window.innerHeight,
      1,
      2 * this.params.scaleFactor
    );
    // this.camera.position.z = this.params.scaleFactor / 2;
    this.camera.position.z = this.params.scaleFactor / 2;
    this.scene = new THREE.Scene();
    this.scene.fog = new THREE.FogExp2(0x000000, 0.001);

    this.generateOrbit();

    // Create particle systems
    for (let k = 0; k < this.params.numLevels; k++) {
      for (let s = 0; s < this.params.numSubsets; s++) {
        // const positions = this.subsets[s].map(
        //   (p) => (p.vertex.x, p.vertex.y, p.vertex.z)
        // );
        // console.log(this.subsets[s][0].vertex);
        // const pls = this.subsets[s][0].vertex;
        // console.log(pls.x, pls.y, pls.z);
        // console.log("this is a test");
        // console.log(this.subsetPositions[s].normalized.slice(0, 3));
        // debugger;
        let geometry = new THREE.BufferGeometry();
        // geometry.setFromPoints(this.subsets[s].map((p) => p.vertex));
        geometry.setAttribute(
          "position",
          new THREE.Float32BufferAttribute(
            this.subsetPositions[s].normalized,
            3
          )
        );
        // geometry.setAttribute(
        //   "color",
        //   new THREE.Float32BufferAttribute(this.subsetPositions[s].colors, 3)
        // );

        // geometry.setAttribute( 'size', new THREE.Float32BufferAttribute( sizes, 1 ).setUsage( THREE.DynamicDrawUsage ) );

        let spriteSize = this.params.spriteSize;

        let materials = new THREE.PointsMaterial({
          size: spriteSize,
          vertexColors: true,
          map: texture,
          blending: THREE.AdditiveBlending,
          depthTest: false,
          // transparent: true,
        });
        // materials.color.setHSL(
        //   this.params.hueValues[s],
        //   this.params.defSaturation,
        //   this.params.defBrightness
        // );
        let points = new THREE.Points(geometry, materials);
        points.position.x = 0;
        points.position.y = 0;
        points.position.z =
          -this.params.levelDepth * k -
          (s * this.params.levelDepth) / this.params.numSubsets +
          this.params.scaleFactor / 2;

        this.scene.add(points);
        this.particles.push({
          points,
          level: k,
          subset: s,
          material: materials,
        });
      }
    }

    // Setup renderer and effects
    this.renderer = new THREE.WebGLRenderer({
      antialias: false,
    });
    this.renderer.setClearColor(new THREE.Color(0x000000), 1.0);
    this.renderer.setSize(window.innerWidth, window.innerHeight);

    this.container.appendChild(this.renderer.domElement);

    this.stats = new Stats(this.container);
    this.stats.setVisible(this.params.enableDebug);
    this.controls = new FractalControls(this.params, this.container);
    this.controls.onChange = () => {
      this.stats?.setVisible(this.params.enableDebug);
    };

    // Setup listeners
    // document.addEventListener("mousemove", onDocumentMouseMove, false);
    // document.addEventListener("touchstart", onDocumentTouchStart, false);
    // document.addEventListener("touchmove", onDocumentTouchMove, false);
    // document.addEventListener("keydown", onKeyDown, false);
    // window.addEventListener("resize", onWindowResize, false);

    // Schedule orbit regeneration
    setInterval(this.updateOrbit, 250);
  };

  componentDidMount = () => {
    this.init();
    this.animate();
  };

  render = () => {
    return (
      <div>
        <div id="Fractal"></div>
      </div>
    );
  };
}
