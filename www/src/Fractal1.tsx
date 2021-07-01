import React from "react";
import * as THREE from "three";
import Stats from "stats.js";
import dat from "dat.gui";
import testImage from "./15.png";
import typefaceFont from "./fonts/MotoyaLMaru_W3 mono.json";

const clock = new THREE.Clock();
let delta = 0;
let maxFPS = 1 / 15;
maxFPS = 1 / 60;
const targetFPS = 1 / 60;
const limitFPS = false;

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
}

interface FractalOrbitParameters {
  a?: number;
  b?: number;
  c?: number;
  d?: number;
  e?: number;
  speed?: number;
  rotationSpeed?: number;
}

interface FractalProps extends FractalOrbitConstraints, FractalOrbitParameters {
  visible?: boolean;
  scaleFactor?: number;
  cameraBound?: number;
  numPointsPerSubset?: number;
  numSubsets?: number;
  numLevels?: number;
  levelDepth?: number;
  defBrightness?: number;
  defSaturation?: number;
  hueValues?: number[];
  spriteSize?: number;
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
  orbit: FractalOrbit;
  mouseX: number;
  mouseY: number;
  windowHalfX: number;
  windowHalfY: number;

  // Orbit parameters
  a: number;
  b: number;
  c: number;
  d: number;
  e: number;
  speed: number;
  rotationSpeed: number;
}

//const defaultFractalProps: FractalProps = {
//  visible: true,
//  //im default value: 1500,
//  scaleFactor: 1500,
//  //im default value: 200
//  cameraBound: 400,
//  numPointsPerSubset: 40000,
//  numSubsets: 7,
//  //im default value: 5
//  numLevels: 9,
//  //im default value: 400,
//  levelDepth: 1000,
//  //im default value: 1
//  defBrightness: 1.2,
//  defSaturation: 0.8,
//  // need hue value for each subset
//  // hueValues: [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
//  spriteSize: Math.ceil((3 * window.innerWidth) / 1600),
//  // im added chaos mode boolean
//  chaosEnabled: false,

//  // orbit parameters constraints
//  aMin: -30,
//  aMax: 30,
//  bMin: 0.2,
//  bMax: 1.8,
//  cMin: 5,
//  cMax: 17,
//  dMin: 0,
//  dMax: 10,
//  eMin: 0,
//  eMax: 12,

//  // orbit parameters
//  // a: 0,
//  // b: 0,
//  // c: 0,
//  // d: 0,
//  // e: 0,
//  // speed: number;
//  // rotationSpeed: number;
//};

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
  visible = true;
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
  spriteSize = Math.ceil((3 * window.innerWidth) / this.scaleFactor);
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

  execute = (): void => {
    alert("Function was called");
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

    // const wireframeListener = this.gui.add(controls, "wireframe").listen();
    // const cubeColorListener = this.gui.addColor(controls, "color").listen();
    // this.gui.add(controls, "execute");
    // this.gui.open();

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

  private textGeometry?: THREE.TextGeometry;
  private subsets: OrbitSubsetPoint[][] = [];
  private particles: {
    points: THREE.Points;
    level: number;
    subset: number;
    material: THREE.PointsMaterial;
  }[] = [];

  initializeOrbitSubsets = (
    numSubsets: number,
    numPointsPerSubnet: number
  ): OrbitSubsetPoint[][] => {
    let subsets = [];
    for (let i = 0; i < (numSubsets ?? 0); i++) {
      let points = [];
      for (let j = 0; j < (numPointsPerSubnet ?? 0); j++) {
        points.push({
          x: 0,
          y: 0,
          vertex: new THREE.Vector3(0, 0, 0),
        });
      }
      subsets.push(points);
    }
    return subsets;
  };

  constructor(props: FractalProps) {
    super(props);
    this.state = {
      orbit: {
        xMin: 0,
        xMax: 0,
        yMin: 0,
        yMax: 0,
        scaleX: 0,
        scaleY: 0,
      },
      mouseX: 0,
      mouseY: 0,
      windowHalfX: window.innerWidth / 2,
      windowHalfY: window.innerHeight / 2,

      // orbit parameters
      a: 0,
      b: 0,
      c: 0,
      d: 0,
      e: 0,
      speed: 1,
      rotationSpeed: 0.005,
    };

    this.subsets = this.initializeOrbitSubsets(
      this.params.numSubsets,
      this.params.numPointsPerSubset
      // this.props.numSubsets ?? defaultFractalProps.numSubsets ?? 0,
      // this.props.numPointsPerSubset ??  defaultFractalProps.numPointsPerSubset ??  0
    );
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

    this.particles.forEach(({ points, material, level, subset }) => {
      // console.log(points.position.z);
      // this.params.speed += 0.000000005;
      points.position.z += this.params.speed * speedup;
      points.rotation.z += this.params.rotationSpeed * speedup;
      if (points.position.z > this.camera.position.z) {
        points.position.z = -(
          (this.params.numLevels - 1) *
          this.params.levelDepth
        );
        // if (node.geometry.attributes.color.needsUpdate) {
        // if (points.geometry.attributes.color.needsUpdate) {
        // if (points.geometry.attributes.needsUpdate) {
        // points.geometry.__dirtyVertices = true;
        // points.material.color.setHSL(
        console.log("updating the colors");
        material.color.setHSL(
          this.params.hueValues[subset],
          this.params.defSaturation,
          this.params.defBrightness
        );
        // points.geometry.attributes.color.needsUpdate = false;
        // node.needsUpdate = false;
        // }
      }
    });

    // console.log(this.scene.objects);
    // for (let i = 0; i < this.scene.objects.length; i++) {
    //   this.scene.objects[i].position.z += this.props.speed;
    //   this.scene.objects[i].rotation.z += this.props.rotationSpeed;
    //   if (this.scene.objects[i].position.z > this.camera.position.z) {
    //     this.scene.objects[i].position.z = -((numLevels - 1) * levelDepth);
    //     if (this.scene.objects[i].needsUpdate == 1) {
    //       this.scene.objects[i].geometry.__dirtyVertices = true;
    //       this.scene.objects[i].myMaterial.color.setHSV(
    //         hueValues[this.scene.objects[i].mySubset],
    //         this.props.defSaturation,
    //         this.props.defBrightness
    //       );
    //       this.scene.objects[i].needsUpdate = 0;
    //     }
    //   }
    // }
    this.renderer.render(this.scene, this.camera);
  };

  updateOrbit = () => {
    this.generateOrbit();
    this.controls?.update();
    this.particles.forEach(({ points, material, level, subset }) => {
      // points.geometry.attributes.color.needsUpdate = true;
      // points.geometry.attributes.color.needsUpdate = true;
    });
  };

  generateOrbit = () => {
    let x, y, z, x1;
    let idx = 0;

    this.prepareOrbit();

    const numPoints = this.params.numPointsPerSubset * this.params.numSubsets;

    let xMin = 0,
      xMax = 0,
      yMin = 0,
      yMax = 0;

    console.assert(this.params.numSubsets == this.subsets?.length ?? 0);

    for (let s = 0; s < this.params.numSubsets; s++) {
      // Use a different starting point for each orbit subset
      x = s * 0.005 * (0.5 - Math.random());
      y = s * 0.005 * (0.5 - Math.random());

      let curSubset: OrbitSubsetPoint[] | undefined = this.subsets[s];
      if (!curSubset) continue;

      for (let i = 0; i < this.params.numPointsPerSubset; i++) {
        // Iteration formula (generalization of the Barry Martin's original one)
        z =
          this.params.d +
          Math.sqrt(Math.abs(this.params.b * x - this.params.c));
        if (x > 0) {
          x1 = y - z;
        } else if (x == 0) {
          x1 = y;
        } else {
          x1 = y + z;
        }
        y = this.params.a - x;
        x = x1 + this.params.e;

        curSubset[i].x = x;
        curSubset[i].y = y;

        if (x < xMin) {
          xMin = x;
        } else if (x > xMax) {
          xMax = x;
        }
        if (y < yMin) {
          yMin = y;
        } else if (y > yMax) {
          yMax = y;
        }

        idx++;
      }
    }

    let scaleX = (2 * this.params.scaleFactor) / (xMax - xMin);
    let scaleY = (2 * this.params.scaleFactor) / (yMax - yMin);

    // this.setState((state, props) => {
    //   return {
    //     orbit: {
    //       xMin: xMin,
    //       xMax: xMax,
    //       yMin: yMin,
    //       yMax: yMax,
    //       scaleX: scaleX,
    //       scaleY: scaleY,
    //     },
    //   };
    // });

    // Normalize and update vertex data
    for (let s = 0; s < this.params.numSubsets; s++) {
      // let curSubset = ;
      for (let i = 0; i < this.params.numPointsPerSubset; i++) {
        this.subsets[s][i].vertex.x =
          scaleX * (this.subsets[s][i].x - xMin) - this.params.scaleFactor;
        this.subsets[s][i].vertex.y =
          scaleY * (this.subsets[s][i].y - yMin) - this.params.scaleFactor;
      }
    }
  };

  prepareOrbit = () => {
    this.shuffleParams();
    this.params.hueValues = new Array(this.params.numSubsets)
      .fill(0)
      .map((v) => Math.random());
    // this.setState((state, props) => {
    //   return {
    //     orbit: {
    //       xMin: 0,
    //       xMax: 0,
    //       yMin: 0,
    //       yMax: 0,
    //       scaleX: state.orbit.scaleX,
    //       scaleY: state.orbit.scaleY,
    //     },
    //   };
    // });
  };

  shuffleParams = () => {
    // this.setState((state, props) => {
    // const aMin = props.aMin ?? defaultFractalProps.aMin ?? 0;
    // const aMax = props.aMax ?? defaultFractalProps.aMax ?? 0;
    // const bMin = props.bMin ?? defaultFractalProps.bMin ?? 0;
    // const bMax = props.bMax ?? defaultFractalProps.bMax ?? 0;
    // const cMin = props.cMin ?? defaultFractalProps.cMin ?? 0;
    // const cMax = props.cMax ?? defaultFractalProps.cMax ?? 0;
    // const dMin = props.dMin ?? defaultFractalProps.dMin ?? 0;
    // const dMax = props.dMax ?? defaultFractalProps.dMax ?? 0;
    // const eMin = props.eMin ?? defaultFractalProps.eMin ?? 0;
    // const eMax = props.eMax ?? defaultFractalProps.eMax ?? 0;
    // return {
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
    // };
    // });
  };

  init = () => {
    // const texture = this.textureLoader.load( 'logo192.png' );
    const texture = this.textureLoader.load(testImage, (tex) => {
      console.log("texture size is", tex.image.width, tex.image.height);
    });
    // const material = new THREE.MeshBasicMaterial( { map: texture } );
    this.container = document.getElementById("Fractal");
    // Camera FOV default 82
    // const numSubsets =
    //   this.props.numSubsets ?? defaultFractalProps.numSubsets ?? 0;
    // const numLevels =
    //   this.props.numLevels ?? defaultFractalProps.numLevels ?? 0;
    // const numPointsPerSubset =
    //   this.props.numPointsPerSubset ??
    //   defaultFractalProps.numPointsPerSubset ??
    //   0;
    // const scaleFactor =
    //   this.props.scaleFactor ?? defaultFractalProps.scaleFactor ?? 1;
    // const spriteSize =
    //   this.props.spriteSize ?? defaultFractalProps.spriteSize ?? 1;
    // const defSaturation =
    //   this.props.defSaturation ?? defaultFractalProps.defSaturation ?? 1;
    // const defBrightness =
    //   this.props.defBrightness ?? defaultFractalProps.defBrightness ?? 1;
    // const levelDepth =
    //   this.props.levelDepth ?? defaultFractalProps.levelDepth ?? 1;

    this.camera = new THREE.PerspectiveCamera(
      90,
      window.innerWidth / window.innerHeight,
      1,
      3 * this.params.scaleFactor
    );
    this.camera.position.z = this.params.scaleFactor / 2;
    this.scene = new THREE.Scene();
    // Fog - Default value 0.0012
    // this.scene.fog = new THREE.FogExp2(0x000000, 0.0015);
    this.scene.fog = new THREE.FogExp2(0x000000, 0.0015);
    this.generateOrbit();
    // const hueValues =
    //   this.props.hueValues ??
    //   this.state.hueValues ??
    //   defaultFractalProps.hueValues ??
    //   [];
    // new Array(numSubsets).fill(0).map((v) => Math.random());

    // Create particle systems
    for (let k = 0; k < this.params.numLevels; k++) {
      for (let s = 0; s < this.params.numSubsets; s++) {
        // const positions = Float32Array.from(
        //   this.subsets[s].map((p) => (p.vertex.x, p.vertex.y, p.vertex.z))
        // );
        // console.log(this.subsets[s][0].vertex);
        // const pls = this.subsets[s][0].vertex;
        // console.log(pls.x, pls.y, pls.z);
        // console.log("this is a test");
        // console.log(positions.slice(0, 3));
        // debugger;
        let geometry = new THREE.BufferGeometry();
        geometry.setFromPoints(this.subsets[s].map((p) => p.vertex));
        // geometry.setAttribute( 'position', new THREE.Float32BufferAttribute( positions, 3 ) );
        // geometry.setAttribute( 'color', new THREE.Float32BufferAttribute( colors, 3 ) );
        // geometry.setAttribute( 'size', new THREE.Float32BufferAttribute( sizes, 1 ).setUsage( THREE.DynamicDrawUsage ) );

        // geometry.setAttribute( 'position', new THREE.Float32BufferAttribute( vertices, 3 ) );
        let materials = new THREE.PointsMaterial({
          size: this.params.spriteSize,
          map: texture,
          blending: THREE.AdditiveBlending,
          depthTest: false,
          transparent: true,
        });
        materials.color = new THREE.Color(0, 255, 0);
        materials.color.setHSL(
          this.params.hueValues[s],
          this.params.defSaturation,
          this.params.defBrightness
        );
        // console.log("hue", this.params.hueValues[s]);
        // console.log("saturation", this.params.defSaturation);
        // console.log("brightness", this.params.defBrightness);
        let points = new THREE.Points(geometry, materials);
        // points.position = new Vector3(0, 0, 0);
        // points.renderOrder = 2;
        points.position.x = 0;
        points.position.y = 0;
        points.position.z =
          -this.params.levelDepth * k -
          (s * this.params.levelDepth) / this.params.numSubsets +
          this.params.scaleFactor / 2;
        // points.geometry.attributes.position.needsUpdate = false;
        // points.geometry.attributes.needsUpdate = false;
        // points.needsUpdate = false;
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
    this.textGeometry = new THREE.TextGeometry("Welcome back Papa", {
      font,
      size: 80,
      height: 5,
      bevelEnabled: false,
      bevelThickness: 10,
      bevelSize: 8,
      bevelOffset: 0,
      bevelSegments: 5,
    });
    let textMaterial = new THREE.MeshBasicMaterial({
      color: new THREE.Color(0xffffff),
      // color: new THREE.Color(255, 0, 0),
      depthTest: false,
    });
    let text = new THREE.Mesh(this.textGeometry, textMaterial);
    text.position.x = -window.innerWidth / 2;
    text.position.y = 0;
    // text.position.z = -10;

    // text.renderOrder = 1;
    // this.scene.add(text);

    // Setup renderer and effects
    this.renderer = new THREE.WebGLRenderer({
      antialias: false,
    });
    this.renderer.setClearColor(new THREE.Color(0x000000), 1.0);
    this.renderer.setSize(window.innerWidth, window.innerHeight);

    this.container.appendChild(this.renderer.domElement);

    this.stats = new AppStats(this.container);
    this.controls = new FractalControls(this.params, this.container);

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
      <div className="Fractal">
        <div id="Fractal"></div>
      </div>
    );
  };
}
