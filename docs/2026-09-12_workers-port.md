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

Existing media remains readable. Other GraphQL mutations and username/password authentication remain available. Derived ride/route results are computed directly; the disabled-cache adapter performs neither Redis I/O nor unnecessary cache serialization. No queues or replacement persistent cache have been introduced.

## Timezone data, size and CPU limitations

Embedding the original global tzf dataset produced an 8.6 MiB compressed Worker, above the Free plan's 3 MiB limit. The build now copies the **unchanged**, checksum-verified `tzf-rel 0.0.2025-a` protobuf files from Cargo's locked dependency into public static assets. Each file is below the asset-size limit. The build also publishes ODbL licensing and attribution; the adapted tzf lookup order's MIT notice is in `docs/licenses/tzf-rs-MIT.txt`.

Timezone lookup preserves `tzf-rs 0.4.11`'s preindex/polygon/coordinate-shift order. The preindex is loaded lazily only when a timezone field is requested; full polygons are loaded only for a preindex miss. Parsed immutable data is cached per isolate, not per request. No in-flight I/O or request bindings are shared between requests. Asset/decode failures are errors, not guessed timezones.

Measured dry-run after this change: approximately **1.75 MiB gzip** (6.25 MiB raw), plus separate static assets. Local workerd successfully handles both preindex and polygon-fallback queries. This proves functionality, **not production Free-tier CPU compliance**. A native benchmark measured roughly 24 ms for cold preindex initialization and 56 ms including polygons; Argon2 authentication and uncached route simplification are also CPU-heavy. Local Wrangler request times include database/asset waits and cannot establish billed CPU. Benchmark actual request CPU and memory before promising the sub-$10 hosting target. No paid-plan upgrade is assumed.

## Native infrastructure and cutover

Shared PostgreSQL repositories now use tokio-postgres instead of SQLx macros. Native CLI/job processes retain TLS connections; closed connections are re-established on a subsequent acquisition, with no automatic replay of failed writes. Access is serialized around explicit transactions. Full native CLI/job checks remain blocked locally by missing pre-existing pkg-config/libheif prerequisites.

The obsolete Rust API Dockerfile and its K3s build/deploy matrix entries are removed. Existing Kubernetes API resources, ingress, other services and the running deployment are **untouched**. Production API routing and retirement of that deployment require an explicit cutover after live Hyperdrive/CPU validation. No Cloudflare CI credentials or automatic deployment workflow were introduced.
