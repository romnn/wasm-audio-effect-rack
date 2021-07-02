import React from "react";
import * as THREE from "three";
import Stats from "stats.js";
import dat from "dat.gui";
import spriteTexture from "./george_face.png";
import typefaceFont from "./fonts/MotoyaLMaru_W3 mono.json";
import BPMDetection from "./nodes/bpm-detection";

const second = 1000;
const minute = 60 * second;
const clock = new THREE.Clock();
let delta = 0;
let maxFPS = 1 / 15;
maxFPS = 1 / 60;
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
  spriteSizeMin?: number;
  spriteSizeMax?: number;
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

interface OrbitSubsetPoint {
  x: number;
  y: number;
  vertex: THREE.Vector3;
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

export class AppStats {
  stats: Stats;
  container: HTMLElement;

  constructor(container?: HTMLElement) {
    this.stats = new Stats();
    this.stats.showPanel(0);
    this.container = container ?? document.body;
    this.container.appendChild(this.stats.dom);
  }

  update(): void {
    this.stats.update();
  }

  start(): void {
    this.stats.begin();
  }

  end(): void {
    this.stats.end();
  }
}

export class FractalParameters implements FractalProps {
  private isReactive = false;

  visible = true;
  showFps = false;
  //im default value: 1500,
  scaleFactor = 1500;
  //im default value: 200
  cameraBound = 400;
  numPointsPerSubset = 20000;
  numSubsets = 7;
  //im default value: 5
  numLevels = 9;
  //im default value: 400,
  levelDepth = 800;
  //im default value: 1
  defBrightness = 0.5;
  defSaturation = 0.8;
  // need hue value for each subset
  // hueValues: [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7];
  hueValues: number[] = [];
  spriteSizeMin = Math.ceil((3 * window.innerWidth) / this.scaleFactor) * 0.4;
  spriteSizeMax = Math.ceil((3 * window.innerWidth) / this.scaleFactor) * 1.2;
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
  speed = 2;
  rotationSpeed = 0.001;

  react = () => {
    console.log("reacting to audio now");
    if (this.isReactive) return;
    const ctx = new window.AudioContext();
    const audio = new Audio("mars_venus.mp3");
    audio.autoplay = true;
    audio.loop = true;
    // audio.muted = true;
    const source = ctx.createMediaElementSource(audio);
    const bpmDetector = new BPMDetection(
      ctx,
      "bpm-detection-node-processor",
      undefined,
      {
        onInitialized: (inst: BPMDetection) => {
          bpmDetector.onBPMChanged = (bpm: number) => {
            console.log("bpm changed to ", bpm);
          };
          source.connect(bpmDetector.workletHandle!);
          // bpmDetector.workletHandle!.connect(ctx.destination);
          source.connect(ctx.destination);
        },
      }
    );
    this.isReactive = true;
  };
}

export class FractalControls<T> {
  container: HTMLElement;
  gui: dat.GUI;
  ctrl: T;

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
    // appearanceMenu.open();
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

    // const wireframeListener = this.gui.add(controls, "wireframe").listen();
    // const cubeColorListener = this.gui.addColor(controls, "color").listen();
    this.gui.add(this.ctrl, "react");
    this.gui.close();

    // wireframeListener.onChange(
    //   (enabled: boolean) => (app.cube.material.wireframe = enabled)
    // );
    // cubeColorListener.onChange((color: string) =>
    //   app.cube.material.emissive.setHex(parseInt(color.replace("#", "0x"), 16))
    // );
    // speedListener;

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
  positions: Float32Array;
  normalized: Float32Array;
}

export default class Fractal extends React.Component<
  FractalProps,
  FractalState
> {
  private stats?: AppStats;
  private params = new FractalParameters();
  private controls?: FractalControls<FractalParameters>;

  private container?: any;
  private camera?: any;
  private scene?: any;
  private renderer?: any;
  private composer?: any;

  private textureLoader = new THREE.TextureLoader();
  private fontLoader = new THREE.FontLoader();

  private orbit: FractalOrbit = {
    xMin: 0,
    xMax: 0,
    yMin: 0,
    yMax: 0,
    scaleX: 0,
    scaleY: 0,
  };
  // private subsets: OrbitSubsetPoint[][] = [];
  private needsUpdate: boolean[] = [];
  private subsetPositions: OrbitPositions[] = [];
  private text?: THREE.Mesh<THREE.TextGeometry>;
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

  constructor(props: FractalProps) {
    super(props);
    this.state = {
      // mouseX: 0,
      // mouseY: 0,
      // windowHalfX: window.innerWidth / 2,
      // windowHalfY: window.innerHeight / 2,
    };
    this.needsUpdate = new Array(this.params.numSubsets).fill(false);
    this.subsetPositions = new Array(this.params.numSubsets).fill(0).map(() => {
      return {
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

    if (this.text) this.text.rotation.x += 0.1;

    // this.params.speed += this.randomNum(-1, 1) * 0.1;
    this.params.rotationSpeed += this.randomNum(-1, 1) * 0.00001;
    this.particles.forEach(({ points, material, level, subset }) => {
      // console.log(points.position.z);
      points.position.z += this.params.speed * speedup;
      points.rotation.z += this.params.rotationSpeed * speedup;
      let currentColor = { h: 0, s: 0, l: 0 };
      material.color.getHSL(currentColor);
      let hue = currentColor.h;
      if (points.position.z > this.camera.position.z) {
        hue = this.params.hueValues[subset];
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
        // if (node.geometry.attributes.color.needsUpdate) {
        // if (points.geometry.attributes.color.needsUpdate) {
        // if (points.geometry.attributes.needsUpdate) {
        // points.geometry.__dirtyVertices = true;
        // points.material.color.setHSL(
        // console.log("updating the colors");
        // points.geometry.attributes.color.needsUpdate = false;
        // node.needsUpdate = false;
        // }
      }
      material.color.setHSL(
        hue,
        this.params.defSaturation,
        this.params.defBrightness
      );
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
    let idx = 0;

    // reset the orbit parameters and randomize the parameters
    this.prepareOrbit();

    const numPoints = this.params.numPointsPerSubset * this.params.numSubsets;

    // console.log(this.subsetPositions);
    console.assert(
      this.params.numSubsets == (this.subsetPositions?.length ?? 0)
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
        } else if (x == 0) {
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

        idx++;
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
      3 * this.params.scaleFactor
    );
    this.camera.position.z = this.params.scaleFactor / 2;
    this.scene = new THREE.Scene();
    this.scene.fog = new THREE.FogExp2(0x000000, 0.0012);
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
        // geometry.setAttribute( 'color', new THREE.Float32BufferAttribute( colors, 3 ) );
        // geometry.setAttribute( 'size', new THREE.Float32BufferAttribute( sizes, 1 ).setUsage( THREE.DynamicDrawUsage ) );
        // geometry.setAttribute( 'position', new THREE.Float32BufferAttribute( vertices, 3 ) );

        let spriteSize =
          this.params.spriteSizeMin +
          Math.random() *
            (this.params.spriteSizeMax - this.params.spriteSizeMin);

        let materials = new THREE.PointsMaterial({
          size: spriteSize,
          map: texture,
          blending: THREE.AdditiveBlending,
          depthTest: false,
          transparent: true,
        });
        materials.color.setHSL(
          this.params.hueValues[s],
          this.params.defSaturation,
          this.params.defBrightness
        );
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

    // create the text
    let font = this.fontLoader.parse(typefaceFont);
    let textGeometry = new THREE.TextGeometry("Welcome back Papa", {
      font,
      size: 80,
      height: 1,
      bevelEnabled: false,
      bevelThickness: 10,
      bevelSize: 8,
      bevelOffset: 0,
      bevelSegments: 5,
    });
    textGeometry?.computeBoundingBox();
    let textMaterial = new THREE.MeshBasicMaterial({
      color: new THREE.Color(0xffffff),
      depthTest: false,
    });
    this.text = new THREE.Mesh(textGeometry, textMaterial);
    const textBBMax = textGeometry?.boundingBox?.max;
    const textBBMin = textGeometry?.boundingBox?.min;
    let textWidth = (textBBMax?.x ?? 0) - (textBBMin?.x ?? 0);
    let textHeight = (textBBMax?.y ?? 0) - (textBBMin?.y ?? 0);

    this.text.position.x = -0.5 * (textWidth ?? 0);
    this.text.position.y = 0.5 * (textHeight ?? 0);
    this.text.position.z = -10;
    this.text.visible = false;
    this.scene.add(this.text);

    // Setup renderer and effects
    this.renderer = new THREE.WebGLRenderer({
      antialias: false,
    });
    this.renderer.setClearColor(new THREE.Color(0x000000), 1.0);
    this.renderer.setSize(window.innerWidth, window.innerHeight);

    this.container.appendChild(this.renderer.domElement);

    if (this.params.showFps) this.stats = new AppStats(this.container);
    this.controls = new FractalControls(this.params, this.container);

    // Setup listeners
    // document.addEventListener("mousemove", onDocumentMouseMove, false);
    // document.addEventListener("touchstart", onDocumentTouchStart, false);
    // document.addEventListener("touchmove", onDocumentTouchMove, false);
    // document.addEventListener("keydown", onKeyDown, false);
    // window.addEventListener("resize", onWindowResize, false);

    // Schedule orbit regeneration
    setInterval(this.updateOrbit, 250);
    this.showText();
  };

  showText = () => {
    setInterval(() => {
      if (this.text) this.text.visible = true;
      setInterval(() => {
        if (this.text) this.text.visible = false;
        this.showText();
      }, 15 * second);
    }, 15 * second);
  };

  componentDidMount = () => {
    this.init();
    this.animate();
  };

  render = () => {
    return (
      <div className="Fractal">
        <div id="Fractal"></div>
      </div>
    );
  };
}
