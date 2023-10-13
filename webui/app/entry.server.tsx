import { CacheProvider } from "@emotion/react";
import createEmotionServer from "@emotion/server/create-instance";
import type { AppLoadContext, EntryContext } from "@remix-run/node";
import { RemixServer } from "@remix-run/react";
import { renderToString } from "react-dom/server";
import { ApolloClient, ApolloProvider, InMemoryCache } from "@apollo/client";
// import { getDataFromTree } from "@apollo/client/react/ssr";
import { Response } from "@remix-run/node";

import { createEmotionCache } from "~/styles/createEmotionCache";
import { ServerStyleContext } from "~/styles/server.context";

export default function handleRequest(
  request: Request,
  responseStatusCode: number,
  responseHeaders: Headers,
  remixContext: EntryContext,
  loadContext: AppLoadContext
): Response {
  const client = new ApolloClient({
    ssrMode: true,
    uri: "https://howitt-api.haslehurst.net/graphql",
    cache: new InMemoryCache(),
  });

  const styleCache = createEmotionCache();
  const { extractCriticalToChunks } = createEmotionServer(styleCache);

  const App = (
    <ApolloProvider client={client}>
      <CacheProvider value={styleCache}>
        <RemixServer context={remixContext} url={request.url} />
      </CacheProvider>
    </ApolloProvider>
  );

  const html = renderToString(
    <ServerStyleContext.Provider value={null}>
      {App}
    </ServerStyleContext.Provider>
  );

  const initialState = client.extract();
  const chunks = extractCriticalToChunks(html);

  const markup = renderToString(
    <ServerStyleContext.Provider value={chunks.styles}>
      {App}
      <script
        dangerouslySetInnerHTML={{
          __html: `window.__APOLLO_STATE__=${JSON.stringify(
            initialState
          ).replace(/</g, "\\u003c")}`, // The replace call escapes the < character to prevent cross-site scripting attacks that are possible via the presence of </script> in a string literal
        }}
      />
    </ServerStyleContext.Provider>
  );

  responseHeaders.set("Content-Type", "text/html");

  return new Response(`<!DOCTYPE html>${markup}`, {
    status: responseStatusCode,
    headers: responseHeaders,
  });
}
