import type { MetaFunction } from "@remix-run/node";
import {
  Links,
  LiveReload,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import { ApolloClient, InMemoryCache, ApolloProvider } from "@apollo/client";

export const meta: MetaFunction = () => ({
  charset: "utf-8",
  title: "Howitt",
  viewport: "width=device-width,initial-scale=1",
});

const client = new ApolloClient({
  uri: "https://howitt-api.haslehurst.net/graphql",
  cache: new InMemoryCache(),
});

export default function App() {
  return (
    <html lang="en">
      <head>
        <Meta />
        <Links />
        {typeof document === "undefined"
          ? "__STYLES__"
          : null}
      </head>
      <body>
      <ApolloProvider client={client}>
          <Outlet />
        </ApolloProvider>
        <ScrollRestoration />
        <Scripts />
        <LiveReload />
      </body>
    </html>
  );
}
