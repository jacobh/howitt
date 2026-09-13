import { createContext } from "react-router";

export interface WebuiEnv {
  API_BASE_URL: string;
  API: Pick<typeof globalThis, "fetch">;
}

interface CloudflareContext {
  env: WebuiEnv;
  ctx: ExecutionContext;
}

export const cloudflareContext = createContext<CloudflareContext>();
