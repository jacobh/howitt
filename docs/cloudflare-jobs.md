# Background jobs on Cloudflare

`howitt-worker` is a Rust/Wasm queue consumer, separate from `howitt-web`.
It supports RWGPS webhook fan-out, individual route/trip sync, history discovery,
and media location inference. Image transformations belong to the separate
Cloudflare Images task. The old Apalis runtime and image processor are removed;
the disabled web upload/OAuth/webhook endpoints remain disabled.

## Local verification

Prerequisites: the repo Rust toolchain, `wasm32-unknown-unknown`, worker-build
0.8.5, Bun dependencies, and a loopback PostgreSQL/PostGIS server. In an orb,
run `amp orb services ensure`, then:

```sh
PG_USER=howitt_orb bash scripts/test-jobs-local.sh
bun run check:jobs
cargo check --locked -p howitt-cli
cargo check --locked -p howitt-web --target wasm32-unknown-unknown
```

Outside the orb, set `PG_BIN` and `PG_USER` for your local PostgreSQL installation.
The test script creates, migrates and drops a uniquely named disposable database;
it never reads `DATABASE_URL`. Miniflare executes the built Wasm consumer with
mock RWGPS responses and local Hyperdrive sockets. No live Cloudflare/RWGPS
credentials are required.

## Provision and deploy (explicit approval required)

The jobs configuration reuses the web Worker's `HYPERDRIVE` binding. Confirm its
destination before deploying. These commands create shared resources and deploy
code; local verification does not execute them:

```sh
bunx wrangler queues create howitt-jobs --message-retention-period-secs 1209600
bunx wrangler queues create howitt-jobs-dead --message-retention-period-secs 1209600
bun run deploy:jobs
```

This assumes Workers Paid. `wrangler.jobs.toml` starts with one message per
invocation, at most two concurrent consumers, five retries and a 60-second retry
delay. Retention is finite, including in the dead-letter queue: inspect failures
before they expire. There is no public HTTP endpoint on this Worker.

The old Kubernetes manifest and CI build/deploy entries are removed from source.
This does not delete an existing cluster deployment or Redis service. Retiring any
remaining shared resources is a separate, explicitly approved operation.

## Publish from the CLI

Provide `CLOUDFLARE_ACCOUNT_ID`, `HOWITT_QUEUE_ID` (the ID of `howitt-jobs`) and
`CLOUDFLARE_API_TOKEN` with Queues Write permission for the selected account.
Supply secrets through your environment, not checked-in files. Queue-only commands
do not open a database connection or require Redis.

```sh
cargo run -p howitt-cli -- rwgps enq-route-sync --user-id <uuid> --rwgps-route-id <id>
cargo run -p howitt-cli -- rwgps enq-trip-sync --user-id <uuid> --rwgps-trip-id <id>
cargo run -p howitt-cli -- rwgps enq-history-sync --user-id <uuid>
cargo run -p howitt-cli -- media infer-location --media-id <uuid>
```

Worker producers use `howitt_jobs::enqueue(&env, job)` with a `JOBS` queue binding.
Always await publication before returning success. The wire format is JSON:

```json
{
  "version": "1",
  "job": {
    "Rwgps": {
      "SyncRoute": {
        "user_id": "USER#00000000-0000-0000-0000-000000000001",
        "rwgps_route_id": 123
      }
    }
  }
}
```

The CLI sends this object as `body` with `content_type: "json"` to the
[Queues push API](https://developers.cloudflare.com/api/resources/queues/subresources/messages/methods/push/).
Credentials are resolved from the user's current database connection at execution
time and are never embedded in queue payloads.

## Delivery and recovery

- A message is acknowledged only after its handler and every child publication
  succeed. A partial fan-out can publish duplicates on retry.
- Route/trip writes lock the external ID inside a database transaction, reuse the
  existing identity, and commit the record and points together. Older snapshots
  cannot overwrite newer ones. Equal timestamps can repair missing points.
- Cross-user ownership collisions fail rather than overwrite another user's data.
  Existing route descriptions/tags/name/slug and ride name/distance are preserved.
- Invalid versions, malformed jobs and processing errors retry into
  `howitt-jobs-dead`. Logs include message IDs and completion/failure, not bodies
  or credentials. Inspect the dead-letter payload and upstream/database state to
  diagnose the failure before replaying.
- Replay a route/trip/history/inference job with the corresponding CLI command.
  A webhook can be replayed as its individual route/trip sync. Confirm success,
  then acknowledge/remove the original dead-letter message. For bulk replay, use
  the [HTTP pull API](https://developers.cloudflare.com/queues/configuration/pull-consumers/)
  on the dead-letter queue, republish the decoded JSON to `howitt-jobs`, and only
  acknowledge each pulled lease after publication succeeds. Never bulk-ack first.

The existing history selection fetches up to 1,000 routes and 5,000 trips per user;
it is not a paginated full-account export. Consumer concurrency is an initial
bound, not a per-user RWGPS rate limiter. Monitor upstream throttling and queue
age before raising it. Restoring web producers, adding durable DB-to-queue outbox
delivery, and Cloudflare Images integration are outside this change.
