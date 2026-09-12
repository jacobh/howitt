# Rust API on Cloudflare Workers — first pass

`src/bin/howitt-web` is now a Rust/Wasm library, using workers-rs, Axum and the existing GraphQL schema. The frontend and background worker are not migrated by this change.

## Configuration and secrets

Root `wrangler.toml` owns the Worker build, workers.dev deployment, Hyperdrive binding and static asset binding. It binds `HYPERDRIVE` to `324c2bb0ba3d4ac9b1fce0618ccbae4d`. There are no production custom-domain routes yet.

The only application secret required is `JWT_SECRET`. Reuse the current API's value to keep existing login tokens valid. Provision it interactively with `bun run wrangler secret put JWT_SECRET` (or the dashboard); never commit it. Database credentials remain in Hyperdrive, not Worker environment variables. No `DATABASE_URL`, Redis, S3 or RWGPS OAuth secrets are required on this first-pass Worker.

Before deployment, verify Hyperdrive's origin database/role/TLS and caching configuration. This port preserves parameterized SQL and explicit transactions. Local integration tests exercise the full adapter; a live read-only query through the PlanetScale binding has also passed (see deployment record below). Hyperdrive result caching is disabled for fresh reads after writes. All 65 shared repository statements now use tokio-postgres's typed, unnamed APIs (`query_typed`, `query_typed_one`, `execute_typed`), including transactional writes. Live frontend testing found that dropping named prepared statements closed the Hyperdrive connection when caching was disabled. Explicit parameter types preserve SQL semantics without relying on that unsupported path. A source regression guard prohibits named-statement APIs in repositories, and disposable local integration tests cover parameter types, repeated calls, rollback, and native reconnect.

## Build and local validation

Prerequisites: Bun, a Rust nightly toolchain (the existing domain crate uses nightly features), `wasm32-unknown-unknown`, and worker-build 0.8.5. `ring` also requires a Wasm-capable Clang. Apple Clang lacks that backend; on macOS install Homebrew LLVM. The build script sets **target-specific** compiler variables and does not replace the system compiler.

```sh
rustup target add wasm32-unknown-unknown
cargo install worker-build --version 0.8.5 --locked
# macOS only:
brew install llvm
bun install --frozen-lockfile
bun run build:worker
bun run check:worker  # Wrangler deploy --dry-run; does not publish
cargo test -p howitt-web --lib
cargo test -p howitt bypasses_cache_and_serialization
```

`worker-build` downloads its version-matched wasm-bindgen/wasm-opt tools on first use. Build outputs and local Worker secret files are ignored by Git.

```sh
# Requires a local PostgreSQL+PostGIS server at 127.0.0.1:5432.
# Defaults to Postgres.app tools/current OS user; override PG_BIN/PG_USER if needed.
bash scripts/test-worker-local.sh
```

This harness creates a uniquely named `howitt_workers_test_*` database, verifies the local database identity, applies migrations and synthetic fixtures, tests all repository row codecs plus rollback and native reconnect, starts the real Wasm Worker locally, then drops only its own database. It never uses the project's `DATABASE_URL`. Tests cover login/JWT validation, signup, GraphQL/DataLoader, disabled jobs, and timezone asset fallback. Wrangler is invoked through a Bun package script (honoring Wrangler's Node shebang), not forced to execute under Bun's runtime.

Deployment command: `bun run deploy:worker`.

## Live deployment — 2026-09-12

- URL: https://howitt-web.jacob-e2e.workers.dev
- Initial version: `9837837b-134b-498b-ac61-50c1afba8155`
- GraphiQL fix version: `e2ce552a-a028-49df-9b46-27fa51a87ff0`. The upstream HTML template used unversioned GraphiQL assets that no longer exposed its expected UMD global. Pinned GraphiQL 2.4.7 and React/ReactDOM 17.0.2; a regression test passes and the live browser editor and schema documentation now load.
- A new cryptographically random `JWT_SECRET` was provisioned directly through Wrangler without printing or saving it locally. Existing API tokens must be replaced by logging in again.
- Hyperdrive initially targeted `postgres`, causing repository queries to fail. Its database was corrected to `howitt`, matching the restore, and result caching was disabled.
- Browser-based read-only smoke checks passed: `starredRoutes { id }` returned 62 routes without GraphQL errors, OPTIONS returned HTTP 200 with CORS, and the disabled RWGPS callback returned HTTP 503 / `BACKGROUND_JOBS_DISABLED`.
- Python urllib requests were blocked upstream with Cloudflare 1010; browser requests succeeded.
- No database writes, production-domain cutover, or paid-plan upgrade were performed. Live login/password hashing and cold timezone CPU/memory remain unvalidated.

## Deliberately disabled functionality

Before any body processing, OAuth exchange, upload, database write or enqueue:

- `POST /upload/media`, `POST /webhooks/rwgps`, and `GET /auth/rwgps/callback` return HTTP 503 with `BACKGROUND_JOBS_DISABLED`.
- GraphQL `initiateRwgpsHistorySync` and `viewer.rwgpsAuthRequestUrl` return the same explicit error code.

Existing media remains readable. Other GraphQL mutations and username/password authentication remain available. Background queues are still disabled. Derived-data caching now uses Workers KV, as described below.

## Derived-data cache (Workers KV)

`wrangler.toml` binds `DERIVED_CACHE` to namespace `5de58615f5114086b708a9387f518552`. No KV secrets or API tokens are needed in the Worker. The namespace contains disposable derived values only, never authoritative records or session credentials.

- Shared `howitt_client_types::CacheStore` replaces the Redis-named trait; it supports byte reads and writes with a TTL. The native Redis implementation remains available and now uses `SET EX`.
- `CacheFetcher` uses bincode payloads, the `derived-v1:` key prefix, and a fixed **one-hour expiry**. Bump the prefix when changing serialization or derived-data algorithms. Changes to source GPS data can remain stale until expiry; no write-through invalidation is implemented.
- The API caches **full-resolution route GPS data and calculated distance/elevation profiles and totals**, simplified ride geometry, and trip elevation profiles. Route fields retain their original point counts, ordering and values; they do not switch to the simplified-route algorithm. Cues use the cached GPS points but still fetch current POIs and generate cues normally.
- `RouteDataLoader` checks KV for all requested routes, then fetches all misses in one batched PostgreSQL query. A request-scoped GraphQL `HashMapCache` shares the resulting `Arc<RouteData>` across route fields, avoiding repeated KV lookups and copying large point arrays. No request-local state or I/O handles survive the request.
- Cache hits skip computation. Read failures, invalid cached bytes, serialization failures and write failures return freshly computed data instead of failing the request. Source/database/computation errors still propagate. In particular, KV's one-write-per-second-per-key limit does not cause API failures on concurrent misses. Concurrent misses may still duplicate computation.
- KV is eventually consistent; a write (or even a cached miss) may not immediately become visible elsewhere. It is not suitable here for authorization or other correctness-critical state. Authorization remains outside the cache.
- KV bindings are request-scoped. Binary get/put is implemented directly using workers-rs 0.8.5, with its Send wrappers for Axum/async-graphql compatibility.
- Workers Paid is enabled. Its included KV allowance is 10 million reads, 1 million writes and 1 GB storage per month; excess operations/storage are billable. This is not a hard spending cap.

Initial KV deployment: API version `4dce5940-4754-49c7-b955-dbc7b936daa8`. Live read-only GraphQL smoke queries for a four-ride trip returned identical results on repeat. Remote KV inspection confirmed five versioned entries (four ride geometries and one trip elevation profile), each expiring approximately one hour after creation. No production database writes were performed.

Route caching deployment: API version `873edc62-8963-448d-987c-2b611a9391e4`. A live Alpine Way query covering all geometry/profile fields and totals produced the same SHA-256 before deployment, on a cold cache, and on a warm cache (`c2cdd54b23ca1e8d92904824fda1366aecec61ca4f4465c0d4274ec28d853264`, 519 original points). The frontend route-list browser check passed, and remote KV inspection confirmed 62 versioned route-data entries with one-hour expiry.

Validation: eight shared cache tests cover hits, expiry/version arguments, misses, corrupt bytes, failed reads/writes/serialization, source errors and disabled-cache bypass. Four route-data tests verify empty/single/multi-point semantics, one database batch for cold misses, mixed-hit batches, and serialized cache reuse across independent loaders. `cargo check -p howitt_clients` checks the native Redis implementation; API tests and the Wasm dry-run pass. `scripts/test-worker-local.sh` now creates isolated local KV state as well as a disposable PostgreSQL database. It proves real Wasm KV hits across HTTP requests by warming derived values, deleting the disposable source GPS rows, and asserting identical subsequent results. The harness removes both owned resources on exit. Never run its deletion check against production.

## Timezone data, size and CPU limitations

Embedding the original global tzf dataset produced an 8.6 MiB compressed Worker, above the Free plan's 3 MiB limit. The build now copies the **unchanged**, checksum-verified `tzf-rel 0.0.2025-a` protobuf files from Cargo's locked dependency into public static assets. Each file is below the asset-size limit. The build also publishes ODbL licensing and attribution; the adapted tzf lookup order's MIT notice is in `docs/licenses/tzf-rs-MIT.txt`.

Timezone lookup preserves `tzf-rs 0.4.11`'s preindex/polygon/coordinate-shift order. The preindex is loaded lazily only when a timezone field is requested; full polygons are loaded only for a preindex miss. Parsed immutable data is cached per isolate, not per request. No in-flight I/O or request bindings are shared between requests. Asset/decode failures are errors, not guessed timezones.

Measured dry-run after this change: approximately **1.75 MiB gzip** (6.25 MiB raw), plus separate static assets. Local workerd successfully handles both preindex and polygon-fallback queries. This proves functionality, **not production Free-tier CPU compliance**. A native benchmark measured roughly 24 ms for cold preindex initialization and 56 ms including polygons; Argon2 authentication and uncached route simplification are also CPU-heavy. Local Wrangler request times include database/asset waits and cannot establish billed CPU. Benchmark actual request CPU and memory before promising the sub-$10 hosting target. The account owner subsequently enabled Workers Paid during frontend deployment; the original Free-tier CPU caveat above is retained as historical context.

## Native infrastructure and cutover

Shared PostgreSQL repositories now use tokio-postgres instead of SQLx macros. Native CLI/job processes retain TLS connections; closed connections are re-established on a subsequent acquisition, with no automatic replay of failed writes. Access is serialized around explicit transactions. Full native CLI/job checks remain blocked locally by missing pre-existing pkg-config/libheif prerequisites.

The obsolete Rust API Dockerfile and its K3s build/deploy matrix entries are removed. Existing Kubernetes API resources, ingress, other services and the running deployment are **untouched**. Production API routing and retirement of that deployment require an explicit cutover after live Hyperdrive/CPU validation. No Cloudflare CI credentials or automatic deployment workflow were introduced.
