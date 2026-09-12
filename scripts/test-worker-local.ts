import assert from "node:assert/strict";
const base = "http://127.0.0.1:8789";
const timedFetch = (url: string, init: RequestInit = {}) => fetch(url, { ...init, signal: AbortSignal.timeout(10_000) });
// Wrangler builds the Wasm module before listening; retry readiness only.
let ready = false;
for (let attempt = 0; attempt < 45; attempt++) {
  const response = await timedFetch(`${base}/upload/media`, { method: "POST" }).catch(() => undefined);
  if (response?.status === 503) { ready = true; break; }
  await Bun.sleep(1000);
}
assert(ready, "Local Worker did not start; inspect /tmp/howitt-local-worker.log");
for (const [method, path] of [["POST", "/upload/media"], ["POST", "/webhooks/rwgps"], ["GET", "/auth/rwgps/callback"]]) {
  const response = await timedFetch(base + path, { method, headers: { origin: "https://howittplains.net" } });
  assert.equal(response.status, 503);
  assert.equal(response.headers.get("access-control-allow-origin"), "*");
  assert.equal((await response.json()).code, "BACKGROUND_JOBS_DISABLED");
}
const loginResponse = await timedFetch(`${base}/auth/login`, {
  method: "POST", headers: { "content-type": "application/json" },
  body: JSON.stringify({ username: "worker-test", password: "local-test-password" }),
});
assert.equal(loginResponse.status, 200);
const login = await loginResponse.json();
assert.equal(typeof login.token, "string", JSON.stringify(login));
async function graphql(query: string) {
  const response = await timedFetch(base, {
    method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${login.token}` },
    body: JSON.stringify({ query }),
  });
  assert.equal(response.status, 200);
  return response.json();
}
const viewer = await graphql("{ viewer { id profile { username } } }");
assert.equal(viewer.errors, undefined, JSON.stringify(viewer));
assert.equal(viewer.data.viewer.profile.username, "worker-test");
const route = await graphql('{ routeWithSlug(slug: "test-route") { name } }');
assert.equal(route.errors, undefined, JSON.stringify(route));
assert.equal(route.data.routeWithSlug.name, "Test route");
const timezones = await graphql("{ rides { name tz } trips { media { path tz } } }");
assert.equal(timezones.errors, undefined, JSON.stringify(timezones));
assert.equal(timezones.data.rides[0].tz, "Australia/Melbourne");
const boundaryMedia = timezones.data.trips[0].media.find((item: { path: string }) => item.path === "boundary.jpg");
assert.equal(boundaryMedia.tz, "Australia/Adelaide");
console.log("Wasm timezone assets: preindex and full polygon fallback passed");
const sync = await graphql("mutation { initiateRwgpsHistorySync { id } }");
assert.equal(sync.errors[0].extensions.code, "BACKGROUND_JOBS_DISABLED");
const signup = await timedFetch(`${base}/auth/signup`, {
  method: "POST", headers: { "content-type": "application/json" },
  body: JSON.stringify({ username: "signup-test", email: "signup@example.invalid", password: "local-test-password" }),
});
const signupBody = await signup.json();
assert.equal(signupBody.error, null, JSON.stringify(signupBody));
assert.equal(typeof signupBody.token, "string");
console.log("Local Wasm Worker: disabled routes, JWT login, signup, GraphQL auth/DataLoader, repository query and disabled mutation passed");
