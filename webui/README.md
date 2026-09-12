# Howitt web UI

React/Remix frontend deployed to **https://howitt-webui.jacob-e2e.workers.dev**.

## Worker configuration

`wrangler.toml` defines the `howitt-webui` Worker, build, static assets, public API URL and `API` service binding to `howitt-web`.

- Browser GraphQL and login requests use `API_BASE_URL`, injected into HTML from Wrangler configuration.
- Server-side Apollo requests use the `API` service binding and forward the viewer's token from the incoming cookie. No API signing secret or database credentials belong in this Worker.
- Personalized HTML uses `Cache-Control: private, no-store`; static assets are served separately by Cloudflare.
- `remote = true` means local development also connects to the **live API**. Login/signup and mutations are real operations; use read-only queries unless intentionally changing production data.
- The water page still uses the existing `ts-api.howittplains.net` service; this migration does not move that API.

## Development and deployment

From `webui/`, with Bun and the existing Wrangler login:

```sh
bun install --frozen-lockfile
bun run dev             # Vite development server, port 3000
bun run start           # Built Worker in local workerd
bun run typecheck
bun run lint
bun run test
bun run check:worker    # Build + Wrangler dry-run
bun run deploy:worker   # Build + publish
```

Stop local development processes before deploying; both builds write to `build/`.
Wrangler runs using its Node shebang, not Bun's runtime. `nodejs_compat` supports the existing SSR dependencies.

The frontend Worker is approximately **948 KiB gzip**. Workers Paid was enabled by the account owner during migration; its CPU allowance is useful for SSR and the Rust API's uncached computation. The account plan is shared, not billed separately per Worker. The combined PlanetScale/Workers baseline is US$10/month before taxes or usage overages, not a hard spending cap.

## Validation and cutover

Deployed 2026-09-12: frontend version `ea78e524-685b-44c2-81de-0225948b747d`, backend compatibility-fix version `1fe98cd6-78fa-4cb1-bfd7-9b106437ca45`.

Type checking, ESLint, the Apollo fetch/auth regression test, and deployment builds pass. Live browser checks cover the route list, individual route details, map rendering and server-rendered route data via the service binding. Live signup or other database writes are not part of smoke testing.

The former Express server, Dockerfile and webui K3s CI matrix entries were removed. Existing Kubernetes workloads and production-domain ingress remain untouched. No custom domain or automatic Cloudflare CI deployment is configured.

Backend integration exposed a Hyperdrive incompatibility with named PostgreSQL prepared statements while result caching is disabled. All repository operations now use typed unnamed statements; local tests cover row codecs, writes, transactions/rollback, native reconnect and the real Wasm API.
