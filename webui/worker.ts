import { createRequestHandler, RouterContextProvider } from "react-router";
import { cloudflareContext, type WebuiEnv } from "./app/cloudflare";

const handleRequest = createRequestHandler(
  () => import("virtual:react-router/server-build"),
  import.meta.env.MODE,
);

export default {
  fetch(
    request: Request,
    env: WebuiEnv,
    ctx: ExecutionContext,
  ): Promise<Response> {
    const context = new RouterContextProvider();
    context.set(cloudflareContext, {
      env,
      ctx,
    });

    return handleRequest(request, context);
  },
} satisfies ExportedHandler<WebuiEnv>;
