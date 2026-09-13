# Cloudflare deployment automation

`.github/workflows/cloudflare.yml` validates every pull request, every push to
`main`, and manual runs. Only push/manual runs on `main` deploy, after validation
and the existing GitHub `Production` environment's protection rules. Manual runs
on other branches validate only. No path filters are used: changes to shared Rust
crates, lockfiles, assets, or build tooling must not bypass the build checks.

Validation has no deployment credentials. It installs Bun from `.bun-version`,
Node 22 (Wrangler's runtime), the Rust nightly used by `.agents/setup`, the Wasm
target, LLVM/Clang and worker-build 0.8.5. Both Bun installs use frozen lockfiles;
the Rust build uses `--locked`. It runs observability tests, frontend typecheck,
lint and tests, then all three package-script Wrangler dry-runs. Wrangler invokes
each configuration's custom build, including API timezone assets and Remix SSR
and static assets. No native image libraries or live database are needed.
The Vite Cloudflare development proxy is enabled only for `serve`; production
builds must not open authenticated remote-binding sessions against the live API.

The deployment job checks out the same event commit and rebuilds using the same
toolchain and lockfiles, reusing the Rust cache. It uses the existing deployment
scripts unchanged (Wrangler runs their custom builds), in this order:

1. `howitt-worker` via `wrangler.jobs.toml`: update the queue consumer first.
2. `howitt-web` via `wrangler.toml`: API and timezone assets.
3. `howitt-webui` via `webui/wrangler.toml`: frontend with its API service binding.

Failure stops later deployments. This is not an atomic three-Worker rollout:
earlier successful updates remain live. Keep changes backwards-compatible across
this interval. Production runs share a concurrency group and are not cancelled
mid-rollout; GitHub may replace an older pending run with a newer pending run.
Avoid rerunning old successful workflows to roll back production unintentionally.
CDK and the native Rust test workflow remain unchanged and run independently;
their success is **not** a prerequisite for this workflow's deployment job.
`GITHUB_TOKEN` has only `contents: read`; no repository write permission is needed.

## One-time setup (requires owner approval)

Before merging/enabling this workflow, review the `Production` environment's
allowed deployment branches (restrict to `main`) and required reviewers. This
environment is also used by existing CDK/test jobs: changing its protections
affects those workflows too. Merging this workflow enables production deployment
on subsequent `main` pushes, including its own merge push once secrets are set.

Create these exact GitHub Actions secrets in **Settings → Environments →
Production → Environment secrets**:

| Secret | Value |
| --- | --- |
| `CLOUDFLARE_ACCOUNT_ID` | The account ID owning all three Workers and their bound resources. |
| `CLOUDFLARE_API_TOKEN` | A scoped Cloudflare deployment API token, not a Global API key or runtime credential. |

Follow Cloudflare's [GitHub Actions authentication guide](https://developers.cloudflare.com/workers/ci-cd/external-cicd/github-actions/):
start from **Edit Cloudflare Workers**, restrict account resources to the Howitt
account and zone resources to `howittplains.net`. Retain its Workers Scripts Edit,
Workers KV Storage Edit and Workers Routes Edit permissions and account/zone read
permissions. Add **Queues Edit** for the configured producer/consumer and
dead-letter queue updates, and **Hyperdrive Read** for the existing configuration.
No AWS, Kubernetes, database, RWGPS, or Images credential belongs in GitHub for
these deployments. Do not reuse the queue-only CLI token documented in
[cloudflare-jobs.md](cloudflare-jobs.md): it cannot deploy Workers.

Confirm these existing resources in that account before the first run; this
workflow does not provision or migrate them:

- Workers Paid, `HYPERDRIVE` pointing to the intended PostgreSQL database, and
  `DERIVED_CACHE` KV namespace matching the IDs in the root Wrangler configs.
- `howitt-jobs` and `howitt-jobs-dead` queues. See
  [queue setup and recovery](cloudflare-jobs.md).
- The active `howittplains.net` zone and API/frontend custom domains. Wrangler
  reconciles the configured domains during deployment; do not independently
  reassign them. See [Cloudflare Custom Domains](https://developers.cloudflare.com/workers/configuration/routing/custom-domains/).
- `JWT_SECRET`, `RWGPS_CLIENT_ID`, and `RWGPS_CLIENT_SECRET` stored as **Worker
  secrets on `howitt-web`**. Preserve their values; they are not build secrets.
  Existing Worker secrets persist through deployments and are not uploaded by
  this workflow. The jobs Worker loads each user's RWGPS access token from the
  database; the frontend needs no runtime secrets.
- Disable any duplicate Workers Builds or other deployment automation before
  adopting this workflow. GitHub concurrency does not serialize external deploys.

Pending Cloudflare Images integration needs no CI toolchain or binding changes.
Keep its `IMAGES_UPLOADS_ENABLED` flag false until separately approved provisioning
and delivery tests. When enabled, `IMAGES_ACCOUNT_ID` and `IMAGES_DELIVERY_HASH`
are Worker vars, and `IMAGES_API_TOKEN` is a dedicated Images Write **Worker
secret**, not this deployment token. Once Images locators exist, disable uploads
rather than roll back to a binary without the Images-aware media resolver.

## Local verification and boundaries

With the toolchain above installed:

```sh
bun install --frozen-lockfile
bun install --frozen-lockfile --cwd webui
bun run test:observability
bun run check:jobs
bun run check:worker
(cd webui && bun run typecheck && bun run lint && bun run test && bun run check:worker)
```

Dry-runs validate builds/bundles, not live token permissions, account ownership,
resource existence, or GitHub environment settings. The first real run needs
separate approval and should be reviewed in Actions and Cloudflare. No deployment,
secret changes, database migrations, backup scheduling, CDK changes, or migration
of the separate `ts-api.howittplains.net` service are part of local verification.
