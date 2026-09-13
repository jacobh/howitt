import { CacheProvider } from "@emotion/react";
import createEmotionServer from "@emotion/server/create-instance";
import { renderToReadableStream, renderToString } from "react-dom/server";
import {
  ServerRouter,
  type EntryContext,
  type RouterContextProvider,
} from "react-router";
import { ApolloProvider } from "@apollo/client/react";
import { parseCookie } from "cookie";

import { createEmotionCache } from "~/styles/createEmotionCache";
import { ServerStyleContext } from "~/styles/server.context";
import { getDataFromTree } from "@apollo/client/react/ssr";
import { createApolloClient } from "./services/apollo";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cloudflareContext } from "./cloudflare";

export default async function handleRequest(
  request: Request,
  responseStatusCode: number,
  responseHeaders: Headers,
  routerContext: EntryContext,
  loadContext: RouterContextProvider,
): Promise<Response> {
  const { env } = loadContext.get(cloudflareContext);
  const apiBaseUrl = env.API_BASE_URL;
  const cookieData = parseCookie(request.headers.get("Cookie") ?? "");

  const queryClient = new QueryClient();

  const client = createApolloClient({
    ssrMode: true,
    graphqlUrl: apiBaseUrl,
    fetch: env.API.fetch.bind(env.API),
    getToken: () => cookieData.token,
  });

  const styleCache = createEmotionCache();
  const { extractCriticalToChunks } = createEmotionServer(styleCache);

  const renderApp = (context: EntryContext): React.ReactElement => (
    <QueryClientProvider client={queryClient}>
      <ApolloProvider client={client}>
        <CacheProvider value={styleCache}>
          <ServerRouter context={context} url={request.url} />
        </CacheProvider>
      </ApolloProvider>
    </QueryClientProvider>
  );

  const contextWithoutHandoffStream = {
    ...routerContext,
    serverHandoffStream: undefined,
  };

  await getDataFromTree(renderApp(contextWithoutHandoffStream));

  const html = renderToString(
    <ServerStyleContext.Provider value={null}>
      {renderApp(contextWithoutHandoffStream)}
    </ServerStyleContext.Provider>,
  );

  const initialState = client.extract();
  const chunks = extractCriticalToChunks(html);

  const markupStream = await renderToReadableStream(
    <ServerStyleContext.Provider value={chunks.styles}>
      {renderApp(routerContext)}
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
    {
      onError(error): void {
        responseStatusCode = 500;
        console.error(error);
      },
    },
  );
  const markup = await new Response(markupStream).text();

  responseHeaders.set("Content-Type", "text/html; charset=utf-8");
  // HTML contains viewer-specific Apollo state and versioned asset URLs.
  responseHeaders.set("Cache-Control", "private, no-store");

  return new Response(`<!DOCTYPE html>${markup}`, {
    status: responseStatusCode,
    headers: responseHeaders,
  });
}
