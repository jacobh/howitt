# Howitt in Amp orbs

`.agents/setup` installs the Bun version pinned in `.bun-version`,
nightly-2026-06-15 Rust (with rustfmt, Clippy and the Wasm target), worker-build
0.8.5, native build libraries, and dependencies for the root, webui and CDK
packages. Cargo sources are prefetched; application compilation stays on
demand to keep setup short.
The Rust override and login-shell paths are orb-local, not developer-machine
configuration. CDK's Bun lockfile was imported from its existing npm lockfile;
all three JavaScript installs use frozen lockfiles.

Snapshots retain installed tools and dependencies. A warm setup checks installed
packages and lockfiles rather than reinstalling toolchains. Resume only verifies
the prerequisites; it does not install packages or authenticate with cloud services.

## Local services and tests

Run `amp orb services ensure` to start supervised PostgreSQL 15 and Redis.
Both bind only to loopback, with no public portals. PostgreSQL uses its own
cluster at `~/.local/share/howitt-orb/postgres`, separate from Debian's default
cluster. Its `howitt_orb` superuser uses trust authentication for disposable
local tests only. PostGIS is installed; the test migrations enable it.
Durability is disabled for speed. No production data or credentials are copied.

New login shells in this repository set `PG_BIN` and `PG_USER` for the existing
isolated integration harness:

```sh
amp orb services ensure
bash scripts/test-worker-local.sh
```

The harness creates, migrates and drops only its uniquely named test database.
Setup itself does not migrate or seed application databases. Native CLI and job
processes still require their application configuration; setup never supplies or
uses `DATABASE_URL`, AWS, RWGPS or Cloudflare credentials.

Useful credential-free checks:

```sh
bun run test:observability
bun --cwd webui test
cargo test -p howitt-web --lib
bun run build:worker
```

The first application build compiles Rust dependencies and lets worker-build
download its matching Wasm tools. No builds, deployments or infrastructure
changes run automatically during setup. Cloud credentials, if needed for an
explicitly authorized task, must be supplied separately after snapshot activation.

To verify setup changes, run `bash -n` and `shellcheck` on both lifecycle scripts,
then time `.agents/setup` twice and run `.agents/resume` from a fresh login shell.
The scripts must retain their executable Git mode. Future orbs use these files
only after they reach the project's default branch.

## Shipping and CI deployments

Pushes to `main` automatically deploy Cloudflare Workers and CDK to production;
orb setup/resume does not. See [deployment guidance](../docs/cloudflare-deployments.md).
