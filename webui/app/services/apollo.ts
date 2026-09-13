import {
  ApolloClient,
  HttpLink,
  InMemoryCache,
  type NormalizedCacheObject,
} from "@apollo/client";
import { SetContextLink } from "@apollo/client/link/context";
import possibleTypes from "../__generated__/fragment-types.json";

interface CreateApolloClientOptions {
  ssrMode?: boolean;
  initialState?: NormalizedCacheObject;
  graphqlUrl: string;
  fetch: typeof fetch;
  getToken: () => string | undefined;
}

export function createApolloClient({
  ssrMode = false,
  initialState,
  graphqlUrl,
  fetch,
  getToken,
}: CreateApolloClientOptions): ApolloClient {
  const httpLink = new HttpLink({
    uri: graphqlUrl,
    fetch,
  });

  const authLink = new SetContextLink(({ headers }) => {
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
    // ApolloLink.concat composes links; this is not Array.prototype.concat.
    // oxlint-disable-next-line unicorn/prefer-spread
    link: authLink.concat(httpLink),
    cache: new InMemoryCache({
      possibleTypes: possibleTypes.possibleTypes,
    }).restore(initialState ?? {}),
  });
}
