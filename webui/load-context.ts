import "@remix-run/cloudflare";

export interface WebuiEnv {
  API_BASE_URL: string;
  API: Pick<typeof globalThis, "fetch">;
}

declare module "@remix-run/server-runtime" {
  interface AppLoadContext {
    apiBaseUrl: string;
    apiFetch: typeof fetch;
  }
}
