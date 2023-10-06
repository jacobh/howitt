/** @type {import('@remix-run/dev').AppConfig} */
module.exports = {
  ignoredRouteFiles: ["**/.*"],
  // appDirectory: "app",
  // assetsBuildDirectory: "public/build",
  // serverBuildPath: "build/index.js",
  // publicPath: "/build/",
  serverModuleFormat: 'cjs',
  serverDependenciesToBundle: [
    /^@apollo\/client(\/.*)?$/,
    /^ol(\/.*)?/,
    "ts-invariant", 
    "zen-observable-ts",
    "@wry/equality",
    "@wry/trie",
    "optimism",
    "@wry/context",
  ],
  future: {
    v2_dev: true,
    v2_errorBoundary: true,
    v2_normalizeFormMethod: true,
    v2_meta: true,
    v2_headers: true,
  },
};
