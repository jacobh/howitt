import { ApolloClient, ApolloProvider, InMemoryCache } from "@apollo/client";
import { getDataFromTree } from "@apollo/client/react/ssr";
import type { EntryContext } from "@remix-run/node";
import { Response } from "@remix-run/node";
import { RemixServer } from "@remix-run/react";
import { renderToString } from "react-dom/server";
import { ServerStyleSheet } from "styled-components";

export default async function handleRequest(
  request: Request,
  responseStatusCode: number,
  responseHeaders: Headers,
  remixContext: EntryContext
) {
  const client = new ApolloClient({
    ssrMode: true,
    uri: "https://howitt-api.haslehurst.net/graphql",
    cache: new InMemoryCache(),
  });
  const sheet = new ServerStyleSheet();

  const App = (
    <ApolloProvider client={client}>
      <RemixServer context={remixContext} url={request.url} />
    </ApolloProvider>
  );

  await getDataFromTree(App);

  const initialState = client.extract();

  let markup = renderToString(
    sheet.collectStyles(
      <>
        {App}
        <script
          dangerouslySetInnerHTML={{
            __html: `window.__APOLLO_STATE__=${JSON.stringify(
              initialState
            ).replace(/</g, "\\u003c")}`, // The replace call escapes the < character to prevent cross-site scripting attacks that are possible via the presence of </script> in a string literal
          }}
        />
      </>
    )
  );
  const styles = sheet.getStyleTags();

  markup = markup.replace("__STYLES__", styles);

  responseHeaders.set("Content-Type", "text/html");

  return new Response("<!DOCTYPE html>" + markup, {
    status: responseStatusCode,
    headers: responseHeaders,
  });
}
