import { ApolloProvider } from "@apollo/client";
import { RemixBrowser } from "@remix-run/react";
import React, { StrictMode } from "react";
import { hydrateRoot } from "react-dom/client";
import { CacheProvider } from "@emotion/react";

import { ClientStyleContext } from "~/styles/client.context";
import { createEmotionCache } from "~/styles/createEmotionCache";
import Cookies from "js-cookie";

import { createApolloClient } from "./services/apollo";

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
  const client = createApolloClient({
    graphqlUrl:
      (window as any).__ENV__.CLIENT_GRAPHQL_URL ??
      "https://api.howittplains.net/",
    getToken: () => Cookies.get("token"),
    initialState: (window as any).__APOLLO_STATE__,
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
