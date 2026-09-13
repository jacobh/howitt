import { CacheProvider } from "@emotion/react";
import createEmotionServer from "@emotion/server/create-instance";
import { type AppLoadContext, type EntryContext } from "@remix-run/cloudflare";
import { RemixServer } from "@remix-run/react";
import { renderToString } from "react-dom/server";
import { ApolloProvider } from "@apollo/client/react";
import { parseCookie } from "cookie";

import { createEmotionCache } from "~/styles/createEmotionCache";
import { ServerStyleContext } from "~/styles/server.context";
import { getDataFromTree } from "@apollo/client/react/ssr";
import { createApolloClient } from "./services/apollo";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

export default async function handleRequest(
  request: Request,
  responseStatusCode: number,
  responseHeaders: Headers,
  remixContext: EntryContext,
  loadContext: AppLoadContext,
): Promise<Response> {
  const apiBaseUrl = loadContext.apiBaseUrl;
  const cookieData = parseCookie(request.headers.get("Cookie") ?? "");

  const queryClient = new QueryClient();

  const client = createApolloClient({
    ssrMode: true,
    graphqlUrl: apiBaseUrl,
    fetch: loadContext.apiFetch,
    getToken: () => cookieData.token,
  });

  const styleCache = createEmotionCache();
  const { extractCriticalToChunks } = createEmotionServer(styleCache);

  const App = (
    <QueryClientProvider client={queryClient}>
      <ApolloProvider client={client}>
        <CacheProvider value={styleCache}>
          <RemixServer context={remixContext} url={request.url} />
        </CacheProvider>
      </ApolloProvider>
    </QueryClientProvider>
  );

  await getDataFromTree(App);

  const html = renderToString(
    <ServerStyleContext.Provider value={null}>
      {App}
    </ServerStyleContext.Provider>,
  );

  const initialState = client.extract();
  const chunks = extractCriticalToChunks(html);

  const markup = renderToString(
    <ServerStyleContext.Provider value={chunks.styles}>
      {App}
      <script
        dangerouslySetInnerHTML={{
          __html: `window.__APOLLO_STATE__=${JSON.stringify(
            initialState,
          ).replace(/</g, "\\u003c")}`, // The replace call escapes the < character to prevent cross-site scripting attacks that are possible via the presence of </script> in a string literal
        }}
      />
      <script
        dangerouslySetInnerHTML={{
          __html: `window.__ENV__=${JSON.stringify({
            API_BASE_URL: apiBaseUrl,
          }).replace(/</g, "\\u003c")}`,
        }}
      />
    </ServerStyleContext.Provider>,
  );

  responseHeaders.set("Content-Type", "text/html; charset=utf-8");
  // HTML contains viewer-specific Apollo state and versioned asset URLs.
  responseHeaders.set("Cache-Control", "private, no-store");

  return new Response(`<!DOCTYPE html>${markup}`, {
    status: responseStatusCode,
    headers: responseHeaders,
  });
}
