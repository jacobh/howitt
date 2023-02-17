import { ApolloClient, ApolloProvider, InMemoryCache } from "@apollo/client";
import { RemixBrowser } from "@remix-run/react";
import { startTransition, StrictMode } from "react";
import { hydrateRoot } from "react-dom/client";

function Client() {
  const client = new ApolloClient({
    uri: "https://howitt-api.haslehurst.net/graphql",
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
