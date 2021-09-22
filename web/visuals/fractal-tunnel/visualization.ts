import {DatGuiParameterControls} from "@disco/controls"
import {HSLToRGB, map, sum, threeColor} from "@disco/core/utils/functions";
import * as THREE from "three";
import {OrbitControls} from "three/examples/jsm/controls/OrbitControls";

import Stats from "../stats";
import {
  BaseParameterizedVisualization,
  ParameterizedVisualization,
} from "../visualization"

import spriteTexture from "./fractalLR.png";
import {
  defaultConfig,
  defaultParams,
  FTParams,
  FTStartConfig
} from "./parameterizer";

export class FTControls extends DatGuiParameterControls<FTParams> {
  protected setup() {
    let parameterMenu = this.gui.addFolder("parameters");
    parameterMenu.close();
  }
}

interface FractalOrbit {
  xMin: number;
  xMax: number;
  yMin: number;
  yMax: number;
  scaleX: number;
  scaleY: number;
}

interface OrbitPositions {
  colors: Float32Array;
  positions: Float32Array;
  normalized: Float32Array;
}

export default class FTVisualization extends
    BaseParameterizedVisualization<FTStartConfig, any, FTParams, FTControls>
        implements ParameterizedVisualization<FTStartConfig, any, FTParams> {

  public readonly name = "Fractal Tunnel";
  protected parameters: FTParams = defaultParams;
  protected config: FTStartConfig = defaultConfig;
  protected configured = false;

  public get isDebug() { return super.isDebug };

  protected camera?: any;
  protected scene?: any;
  protected renderer?: any;
  protected composer?: any;
  protected texture!: THREE.Texture;
  protected textureLoader = new THREE.TextureLoader();

  protected orbit: FractalOrbit = {
    xMin : 0,
    xMax: 0,
    yMin: 0,
    yMax: 0,
    scaleX: 0,
    scaleY: 0,
  };

  protected subsetPositions: OrbitPositions[] = [];
  protected particles: {
    points: THREE.Points; level : number; subset : number;
    material : THREE.PointsMaterial;
  }[] = [];

  public frame = 0;

  renderFrame = (frame: number) => {
    if (!this.configured)
      return;
    this.camera.lookAt(this.scene.position);
    // this.params.speed += this.randomNum(-1, 1) * 0.1;
    // this.params.rotationSpeed += this.randomNum(-1, 1) * 0.00001;

    this.particles.forEach(({points, material, level, subset}) => {
      points.position.z += this.parameters.getSpeed();
      points.rotation.z += this.parameters.getRotationSpeed();
      // update the colors
      let [r, g, b] =
          HSLToRGB(this.parameters.getLevelHueList()[subset],
                   this.parameters.getLevelSaturationList()[subset],
                   this.parameters.getLevelBrightnessList()[subset]);

      let colors = this.subsetPositions[subset].colors;
      for (let i = 0; i < colors.length; i = i + 3) {
        colors[i] = r;
        colors[i + 1] = g;
        colors[i + 2] = b;
      }
      points.geometry.setAttribute("color",
                                   new THREE.Float32BufferAttribute(colors, 3));
      points.geometry.attributes.color.needsUpdate = true;
      points.geometry.attributes.color.needsUpdate = true;

      if (points.position.z > this.camera.position.z) {
        points.geometry.setAttribute(
            "position", new THREE.Float32BufferAttribute(
                            this.subsetPositions[subset].normalized, 3));

        points.position.z =
            -((this.config.getNumLevels() - 1) * this.config.getLevelDepth());
      }
    });

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
        this.renderer.setClearColor(new THREE.Color(0x000000), 1.0);
        this.renderer.setSize(window.innerWidth, window.innerHeight);
        this.container.appendChild(this.renderer.domElement);

        this.texture = this.textureLoader.load(
            spriteTexture, (tex) => {
                               // texure can be inspected here
                               // console.log("texture size is",
                               // tex.image.width, tex.image.height);
                           });

        this.scene = new THREE.Scene();
        const [near, far] = [ 1, 2 ];
        this.camera = new THREE.PerspectiveCamera(
            90,
            window.innerWidth / window.innerHeight,
            near,
            far,
        );
        this.camera.position.z = 2;
        this.scene = new THREE.Scene();
        this.scene.fog = new THREE.FogExp2(0x000000, 0.0008);

        // create stats
        this.stats = new Stats(this.container);
        this.stats?.setVisible(super.isDebug);

        this.configure(this.config);

        setInterval(this.updateOrbit, 250);
      }

  public configure = (config: FTStartConfig): void => {
    this.config = config;
    console.log(this.config.toObject());
    const [near, far] = [ 1, 2 * this.config.getScaleFactor() ];
    this.camera = new THREE.PerspectiveCamera(
        90,
        window.innerWidth / window.innerHeight,
        near,
        far,
    );
    this.camera.position.z = this.config.getScaleFactor() / 2;
    this.subsetPositions =
        new Array(this.config.getNumSubsets()).fill(0).map(() => {
          return {
            colors : new Float32Array(3 * this.config.getNumPointsPerSubset())
                         .fill(0.0),
            positions :
                new Float32Array(3 * this.config.getNumPointsPerSubset())
                    .fill(0.0),
            normalized :
                new Float32Array(3 * this.config.getNumPointsPerSubset())
                    .fill(0.0),
          };
        });
    this.generateOrbit();

    // Create particle systems
    for (let k = 0; k < this.config.getNumLevels(); k++) {
      for (let s = 0; s < this.config.getNumSubsets(); s++) {
        let geometry = new THREE.BufferGeometry();
        geometry.setAttribute("position",
                              new THREE.Float32BufferAttribute(
                                  this.subsetPositions[s].normalized, 3));
        geometry.setAttribute("color", new THREE.Float32BufferAttribute(
                                           this.subsetPositions[s].colors, 3));

        // geometry.setAttribute( 'size', new THREE.Float32BufferAttribute(
        // sizes, 1 ).setUsage( THREE.DynamicDrawUsage ) );

        let spriteSize = this.config.getSpriteSize();

        let materials = new THREE.PointsMaterial({
          size : spriteSize,
          vertexColors : true,
          map : this.texture,
          blending : THREE.AdditiveBlending,
          depthTest : false,
        });
        let points = new THREE.Points(geometry, materials);
        points.position.x = 0;
        points.position.y = 0;
        points.position.z =
            -this.config.getLevelDepth() * k -
            (s * this.config.getLevelDepth()) / this.config.getNumSubsets() +
            this.config.getScaleFactor() / 2;
        this.scene.add(points);
        this.particles.push({
          points,
          level : k,
          subset : s,
          material : materials,
        });
      }
    }
    this.configured = true;
  };

  updateOrbit = () => { this.generateOrbit(); };

  generateOrbit = () => {
    let x, y, z, x1;
    let idx = 0;

    // reset the orbit parameters and randomize the parameters
    this.prepareOrbit();

    const numPoints =
        this.config.getNumPointsPerSubset() * this.config.getNumSubsets();

    console.assert(
      this.config.getNumSubsets() == (this.subsetPositions?.length ?? 0)
    );

    for (let s = 0; s < this.config.getNumSubsets(); s++) {
      // Use a different starting point for each orbit subset
      x = s * 0.005 * (0.5 - Math.random());
      y = s * 0.005 * (0.5 - Math.random());

      for (let i = 0; i < this.config.getNumPointsPerSubset(); i++) {
        // iteration formula (generalization of the Barry Martin's original one)
        z = this.parameters.getD() +
            Math.sqrt(
                Math.abs(this.parameters.getB() * x - this.parameters.getC()));

        // todo: this is veryy bad please refactor
        if (x > 0) {
          x1 = y - z;
        } else if (x == 0) {
          x1 = y;
        } else {
          x1 = y + z;
        }
        y = this.parameters.getA() - x;
        x = x1 + this.parameters.getE();

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

    this.orbit.scaleX = (2 * this.config.getScaleFactor()) /
                        (this.orbit.xMax - this.orbit.xMin);
    this.orbit.scaleY = (2 * this.config.getScaleFactor()) /
                        (this.orbit.yMax - this.orbit.yMin);

    // Normalize and update vertex data
    for (let s = 0; s < this.config.getNumSubsets(); s++) {
      for (let i = 0; i < this.config.getNumPointsPerSubset(); i++) {
        this.subsetPositions[s].normalized[3 * i + 0] =
            this.orbit.scaleX * (this.subsetPositions[s].positions[3 * i + 0] -
                                 this.orbit.xMin) -
            this.config.getScaleFactor();
        this.subsetPositions[s].normalized[3 * i + 1] =
            this.orbit.scaleY * (this.subsetPositions[s].positions[3 * i + 1] -
                                 this.orbit.yMin) -
            this.config.getScaleFactor();
      }
    }
  };

  prepareOrbit = () => {
    this.shuffleParams();
    this.orbit.xMin = 0;
    this.orbit.xMax = 0;
    this.orbit.yMin = 0;
    this.orbit.yMax = 0;
  };

  shuffleParams = () => {
    let constraints = this.parameters.getOrbitConstraints();
    if (constraints) {
      let a = constraints.getAMin() +
              Math.random() * (constraints.getAMax() - constraints.getAMin());
      let b = constraints.getBMin() +
              Math.random() * (constraints.getBMax() - constraints.getBMin());
      let c = constraints.getCMin() +
              Math.random() * (constraints.getCMax() - constraints.getCMin());
      let d = constraints.getDMin() +
              Math.random() * (constraints.getDMax() - constraints.getDMin());
      let e = constraints.getEMin() +
              Math.random() * (constraints.getEMax() - constraints.getEMin());

      this.parameters.setA(a);
      this.parameters.setB(b);
      this.parameters.setC(c);
      this.parameters.setD(d);
      this.parameters.setE(e);
    }
  };
}
