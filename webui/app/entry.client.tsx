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
import Cookies from "js-cookie";

import possibleTypes from "./__generated__/fragment-types.json";

interface ClientCacheProviderProps {
  children: React.ReactNode;
}

function ClientStyleCacheProvider({
  children,
}: ClientCacheProviderProps): React.ReactNode {
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

function Client(): React.ReactNode {
  const httpLink = createHttpLink({
    uri:
      (window as any).__ENV__.CLIENT_GRAPHQL_URL ??
      "https://api.howittplains.net/",
  });

  const authLink = setContext((_, { headers }) => {
    const token = Cookies.get("token");

    return {
      headers: {
        ...headers,
        authorization: token ? `Bearer ${token}` : "",
      },
    };
  });

  const client = new ApolloClient({
    link: authLink.concat(httpLink),
    cache: new InMemoryCache({
      possibleTypes: possibleTypes.possibleTypes, // Add possibleTypes configuration
    }).restore((window as any).__APOLLO_STATE__),
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
