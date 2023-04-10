import {
  ApolloClient,
  ApolloProvider,
  createHttpLink,
  InMemoryCache,
} from "@apollo/client";
import { setContext } from "@apollo/client/link/context";
import { RemixBrowser } from "@remix-run/react";
import { startTransition, StrictMode } from "react";
import { hydrateRoot } from "react-dom/client";

function Client() {
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
      <StrictMode>
        <RemixBrowser />
      </StrictMode>
    </ApolloProvider>
  );
}

function hydrate() {
  startTransition(() => {
    hydrateRoot(document, <Client />);
  });
}

if (typeof requestIdleCallback === "function") {
  requestIdleCallback(hydrate);
} else {
  // Safari doesn't support requestIdleCallback
  // https://caniuse.com/requestidlecallback
  setTimeout(hydrate, 1);
}
