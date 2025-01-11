import { CacheProvider } from "@emotion/react";
import createEmotionServer from "@emotion/server/create-instance";
import { type AppLoadContext, type EntryContext } from "@remix-run/node";
import { RemixServer } from "@remix-run/react";
import { renderToString } from "react-dom/server";
import {
  ApolloClient,
  ApolloProvider,
  createHttpLink,
  InMemoryCache,
} from "@apollo/client";
import * as cookie from "cookie";

import { createEmotionCache } from "~/styles/createEmotionCache";
import { ServerStyleContext } from "~/styles/server.context";
import { getDataFromTree } from "@apollo/client/react/ssr";
import { setContext } from "@apollo/client/link/context";

export default async function handleRequest(
  request: Request,
  responseStatusCode: number,
  responseHeaders: Headers,
  remixContext: EntryContext,
  loadContext: AppLoadContext
): Promise<Response> {
  const cookieData = cookie.parse(request.headers.get("Cookie") ?? "");

  const httpLink = createHttpLink({
    uri: process.env.GRAPHQL_URL ?? "https://api.howittplains.net/",
  });

  const authLink = setContext(async (_, { headers }) => {
    const { token } = cookieData;

    return {
      headers: {
        ...headers,
        authorization: token ? `Bearer ${token}` : "",
      },
    };
  });

  const client = new ApolloClient({
    ssrMode: true,
    link: authLink.concat(httpLink),
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
      <script
        dangerouslySetInnerHTML={{
          __html: `window.__ENV__=${JSON.stringify({
            GRAPHQL_URL: process.env.GRAPHQL_URL,
          }).replace(/</g, "\\u003c")}`,
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
