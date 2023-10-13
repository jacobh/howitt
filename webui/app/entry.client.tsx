import {
  ApolloClient,
  ApolloProvider,
  createHttpLink,
  InMemoryCache,
} from "@apollo/client";
import { setContext } from "@apollo/client/link/context";
import { RemixBrowser } from "@remix-run/react";
import React, { StrictMode } from "react";
import { hydrateRoot } from "react-dom/client";
import { CacheProvider } from "@emotion/react";

import { ClientStyleContext } from "~/styles/client.context";
import { createEmotionCache } from "~/styles/createEmotionCache";

interface ClientCacheProviderProps {
  children: React.ReactNode;
}

function ClientStyleCacheProvider({
  children,
}: ClientCacheProviderProps): JSX.Element {
  const [cache, setCache] = React.useState(createEmotionCache());

  const reset = React.useCallback(() => {
    setCache(createEmotionCache());
  }, []);

  return (
    <ClientStyleContext.Provider value={{ reset }}>
      <CacheProvider value={cache}>{children}</CacheProvider>
    </ClientStyleContext.Provider>
  );
}

function Client(): JSX.Element {
  const httpLink = createHttpLink({
    uri: "https://howitt-api.haslehurst.net/graphql",
  });

  const authLink = setContext((_, { headers }) => {
    const apiKey = window.localStorage.getItem("apiKey");

    return {
      headers: {
        ...headers,
        authorization: apiKey ? `Key ${apiKey}` : "",
      },
    };
  });

  const client = new ApolloClient({
    link: authLink.concat(httpLink),
    cache: new InMemoryCache().restore((window as any).__APOLLO_STATE__),
  });

  return (
    <ApolloProvider client={client}>
      <ClientStyleCacheProvider>
        <StrictMode>
          <RemixBrowser />
        </StrictMode>
      </ClientStyleCacheProvider>
    </ApolloProvider>
  );
}

hydrateRoot(document, <Client />);
