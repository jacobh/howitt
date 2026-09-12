import { createRequestHandler } from "@remix-run/cloudflare";
import * as build from "./build/server/index.js";
import type { WebuiEnv } from "./load-context";

const handleRequest = createRequestHandler(build, "production");

export default {
  fetch(request: Request, env: WebuiEnv): Promise<Response> {
    return handleRequest(request, {
      apiBaseUrl: env.API_BASE_URL,
      apiFetch: env.API.fetch.bind(env.API),
    });
  },
};
