const path = require('path');
// const {alias} = require('react-app-rewire-alias')

module.exports = function override(config, env) {
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

  return config;
};
