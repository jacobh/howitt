import assert from "node:assert/strict";
import { parse } from "graphql";
import { test } from "vitest";
import { createApolloClient } from "~/services/apollo";

test("SSR uses the supplied backend fetch and forwards the viewer token", async () => {
  const requests: { url: string; authorization: string | null }[] = [];
  const client = createApolloClient({
    ssrMode: true,
    graphqlUrl: "https://backend.example/",
    getToken: () => "synthetic-test-token",
    fetch: async (input, init) => {
      requests.push({
        url: String(input),
        authorization: new Headers(init?.headers).get("authorization"),
      });
      return Response.json({ data: { viewer: null } });
    },
  });

  const result = await client.query<{ viewer: { id: string } | null }>({
    query: parse("{ viewer { id } }"),
    fetchPolicy: "network-only",
  });

  assert.deepEqual(result.data, { viewer: null });
  assert.equal(requests.length, 1);
  assert.equal(requests[0]?.url, "https://backend.example/");
  assert.equal(requests[0]?.authorization, "Bearer synthetic-test-token");
  client.stop();
});
