import { ApolloProvider } from "@apollo/client/react";
import React, { StrictMode } from "react";
import { hydrateRoot } from "react-dom/client";
import { HydratedRouter } from "react-router/dom";
import { CacheProvider } from "@emotion/react";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { ClientStyleContext } from "~/styles/client.context";
import { createEmotionCache } from "~/styles/createEmotionCache";
import Cookies from "js-cookie";

import { createApolloClient } from "./services/apollo";
import { getApiBaseUrl } from "./env.client";

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
  const queryClient = new QueryClient();

  const client = createApolloClient({
    graphqlUrl: getApiBaseUrl(),
    fetch: window.fetch.bind(window),
    getToken: () => Cookies.get("token"),
    // oxlint-disable-next-line typescript/no-explicit-any
    initialState: (window as any).__APOLLO_STATE__,
  });

  return (
    <QueryClientProvider client={queryClient}>
      <ApolloProvider client={client}>
        <ClientStyleCacheProvider>
          <StrictMode>
            <HydratedRouter />
          </StrictMode>
        </ClientStyleCacheProvider>
      </ApolloProvider>
    </QueryClientProvider>
  );
}

hydrateRoot(document, <Client />);
