// const path = require('path');
// const paths = require('paths');
// const workspaces = require('../workspaces');
// const workspacesConfig = workspaces.init(paths);
// const {alias} = require('react-app-rewire-alias')

const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');
const fs = require('fs');
const path = require('path');
const {override, babelInclude} = require("customize-cra");

const appDirectory = fs.realpathSync(process.cwd());
const resolveApp = relativePath => path.resolve(appDirectory, relativePath);

// module.exports = {
//   paths : (paths, env) => {
//     paths.discoWorkspaces = resolveApp('../');
//     // paths.appIndexJs = path.resolve(__dirname, 'client/index.jsx');
//     // paths.appSrc = path.resolve(__dirname, 'client');
//     return paths;
//   },
//   resolve : {
//     modules : (modules, env) => {
//       // paths.discoWorkspaces = resolveApp('../');
//       // paths.appIndexJs = path.resolve(__dirname, 'client/index.jsx');
//       // paths.appSrc = path.resolve(__dirname, 'client');
//       // modules.push(paths.discoWorkspaces);
//       return [];
//       // modules.push
//       // return modules;
//     },
//   },
//   webpack :
//       override(babelInclude([ fs.realpathSync('src'), fs.realpathSync('..')
//       ]))
// }
module.exports =
    (config, env) => {
      // console.log(config.resolve);
      // config.resolve.modules.push([
      // path.resolve(__dirname, 'src'));
      // config.resolve.modules.push(resolveApp('../tsconfig.json'));
      //     resolveApp('../visuals'), resolveApp('../'),
      //     resolveApp('../viewer')
      //     // paths.appSrc,
      //     // paths.packagesSrc,
      //     // paths.appsSrc
      // );
      // console.log(config.resolve);
      config.module.rules.forEach(rule => {
        (rule.oneOf || []).forEach(oneOf => {
          if (oneOf.loader && oneOf.loader.indexOf('babel-loader') >= 0) {
            // oneOf.include.push(resolveApp('../**/*.ts'));
            // console.log(oneOf);
            oneOf.include = resolveApp('../');
            // oneOf.include = resolveApp('../**/*.ts');
            // console.log(oneOf);
          }
        });
      });

      config.plugins.splice(0, 0, new ForkTsCheckerWebpackPlugin({
                              async : false,
                              useTypescriptIncrementalApi : true,
                              memoryLimit : 4096
                            }));
      // config.plugins.forEach((p, i) => {
      //   if (p.constructor.name === 'ESLintWebpackPlugin') {
      //     // p
      //     // config.plugins.splice(i, 1, new LiveReloadPlugin({
      //     //     port: 35729
      //     // }));
      //   }
      // });
      // // console.log(process.cwd());
      // config.plugins.forEach(plugin => {
      //   if (typeof plugin.constructor === "function") {
      //     try {
      //       const t = new plugin.constructor();
      //       if (t.constructor.name == "ForkTsCheckerWebpackPlugin") {
      //         // console.log(t);
      //       }
      //     } catch (e) {
      //       // ignore
      //     }
      //   }
      //   // (rule.oneOf || []).forEach(oneOf => {
      //   //   if (oneOf.loader && oneOf.loader.indexOf('babel-loader') >= 0) {
      //   //     // oneOf.include.push(resolveApp('../**/*.ts'));
      //   //     console.log(oneOf);
      //   //     oneOf.include = resolveApp('../');
      //   //     // oneOf.include = resolveApp('../**/*.ts');
      //   //     console.log(oneOf);
      //   //   }
      //   // });
      // });

      // return config;
      // return Object.assign(config, override(babelInclude([
      //                        fs.realpathSync('src'), fs.realpathSync('..')
      //                      ]))(config, env))
      return config;
    }

// module.exports = function override(config, env) {
// alias({
//   '@' : 'src',
// })(config)

// const wasmExtensionRegExp = /\.wasm$/;

// config.resolve.extensions.push('.wasm');

// config.module.rules.forEach(rule => {
//   (rule.oneOf || []).forEach(oneOf => {
//     if (oneOf.loader && oneOf.loader.indexOf('file-loader') >= 0) {
//       // make file-loader ignore WASM files
//       oneOf.exclude.push(wasmExtensionRegExp);
//     }
//   });
// });

// // add a dedicated loader for WASM
// config.module.rules.push({
//   test : wasmExtensionRegExp,
//   include : path.resolve(__dirname, 'src'),
//   use : [ {loader : require.resolve('wasm-loader'), options : {}} ]
// });

// make sure web workers are handled before going through typescript
// config.module.rules.unshift({
//   test: /\.worker\.ts$/,
//   // test: /\.ts$/,
//   use: {
//     loader: 'worker-loader',
//     options: {
//       filename: 'static/js/[name].[contenthash:8].js',
//     },
//   },
// });

// return config;
// };
