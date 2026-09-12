declare module "*build/server/index.js" {
  const build: import("@remix-run/cloudflare").ServerBuild;
  export = build;
}
