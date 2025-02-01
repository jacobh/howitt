import { createHttpLink } from "@apollo/client/link/http/createHttpLink";
import { ApolloClient } from "@apollo/client/core/ApolloClient";
import { NormalizedCacheObject } from "@apollo/client/cache/inmemory/types";
import { InMemoryCache } from "@apollo/client/cache/inmemory/inMemoryCache";
import { setContext } from "@apollo/client/link/context";
import possibleTypes from "../__generated__/fragment-types.json";

interface CreateApolloClientOptions {
  ssrMode?: boolean;
  initialState?: NormalizedCacheObject;
  graphqlUrl: string;
  getToken: () => string | undefined;
}

export function createApolloClient({
  ssrMode = false,
  initialState,
  graphqlUrl,
  getToken,
}: CreateApolloClientOptions): ApolloClient<NormalizedCacheObject> {
  const httpLink = createHttpLink({
    uri: graphqlUrl,
  });

  const authLink = setContext((_, { headers }) => {
    const token = getToken();

    return {
      headers: {
        ...headers,
        authorization: token ? `Bearer ${token}` : "",
      },
    };
  });

  return new ApolloClient({
    ssrMode,
    link: authLink.concat(httpLink),
    cache: new InMemoryCache({
      possibleTypes: possibleTypes.possibleTypes,
    }).restore(initialState ?? {}),
  });
}
