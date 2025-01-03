import { CacheProvider } from "@emotion/react";
import createEmotionServer from "@emotion/server/create-instance";
import type { AppLoadContext, EntryContext } from "@remix-run/node";
import { RemixServer } from "@remix-run/react";
import { renderToString } from "react-dom/server";
import { ApolloClient, ApolloProvider, InMemoryCache } from "@apollo/client";

import { createEmotionCache } from "~/styles/createEmotionCache";
import { ServerStyleContext } from "~/styles/server.context";
import { getDataFromTree } from "@apollo/client/react/ssr";

export default async function handleRequest(
  request: Request,
  responseStatusCode: number,
  responseHeaders: Headers,
  remixContext: EntryContext,
  loadContext: AppLoadContext
): Promise<Response> {
  const graphqlUrl = process.env.GRAPHQL_URL ?? "https://api.howittplains.net/";

  const client = new ApolloClient({
    ssrMode: true,
    uri: graphqlUrl,
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

  await getDataFromTree(App);

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
