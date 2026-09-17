# PostgreSQL migrations on Cloudflare

`howitt-migrations` is a dedicated Rust/Wasm administrative Worker. It embeds
every `src/lib/howitt-postgresql/migrations/VNNNN__description.sql` file at build
time, orders it by numeric version, and connects through the same production
`HYPERDRIVE` binding as the API and jobs Workers.

Each invocation opens one PostgreSQL transaction, takes a transaction-scoped
advisory lock, validates the complete applied history, and then executes and
records every pending migration before committing. The
`howitt_schema_migrations` table records version, name, SHA-256 checksum,
execution mode (`applied` or `baseline`), and timestamp. A failed statement rolls
back its DDL and history record together. Concurrent invocations serialize on the
database lock; the second invocation rechecks history and becomes a no-op.
Unknown versions, a history gap, changed names, and checksum drift fail closed.
Never edit or rename an applied migration; add a new migration instead.

## Authentication and exposure

The Worker has a `workers.dev` endpoint so an operator can invoke it without
adding an application route. It accepts only authenticated `POST /status`,
`POST /apply`, and `POST /baseline` requests. Every request, including unknown
paths, must supply
`Authorization: Bearer <MIGRATION_ADMIN_TOKEN>`; the token is a dedicated Worker
secret and is compared in constant time. There is no browser CORS policy, cookie
authentication, GET action, scheduled trigger, queue trigger, or API service
binding. Keep `preview_urls = false`.

Use an independently generated random token with at least 256 bits of entropy,
store it in the team password manager, never commit it or pass it in a URL, and
rotate it after suspected exposure. Cloudflare logs must not include request
headers. The Worker returns migration version numbers but no SQL, database error
text, connection details, or row values.

## Local and CI verification

Unit tests validate migration discovery/order, checksum behavior, and exact
Bearer authentication. PostgreSQL integration tests apply the complete bundled
schema and use synthetic migrations to verify ordered application, concurrent
serialization, replay, checksum drift, transactional rollback, and explicit
baselining. They require a disposable PostGIS database and are ignored unless
requested:

```sh
export HOWITT_MIGRATION_TEST_DATABASE_URL='postgresql://.../disposable_database'
cargo test --locked -p howitt-postgresql --lib
cargo test --locked -p howitt-migrations --lib
cargo test --locked -p howitt-postgresql --test migrations -- --ignored --test-threads=1
bun run check:migrations
```

CI supplies an isolated PostgreSQL 15/PostGIS service and runs all four checks. The
Wrangler dry-run builds the Worker and embeds the repository migrations; it does
not contact Hyperdrive or a production database.

## Deploy and configure (approval required)

The migration Worker is intentionally **not** in the automatic production deploy
sequence. The following actions alter shared Cloudflare state and require explicit
approval:

1. Confirm `wrangler.migrations.toml` names the intended account's production
   Hyperdrive ID and that Hyperdrive targets the `howitt` database and intended
   migration-capable PostgreSQL role.
2. Review the bundled SQL and take/verify an appropriate database backup before
   destructive migrations. `V0026__drop_water_beta.sql` drops two tables.
3. Deploy the Worker, provision its dedicated secret, and retain Wrangler's exact
   `workers.dev` URL:

   ```sh
   bun run deploy:migrations
   bunx wrangler secret put MIGRATION_ADMIN_TOKEN --config wrangler.migrations.toml
   ```

   A deployment without the secret fails closed. These commands are not run by
   CI and must not be run as local verification.

## First production adoption

The adoption plan assumes production already has the schema represented by
`V0001` through `V0025` and has no migration audit table; this must be verified,
not inferred from the repository. Do **not** call `/apply` first under that
condition: it would correctly treat all unrecorded migrations as pending. Before
baselining, inspect the live schema against every migration through `V0025`,
confirm `water_beta` and `osm_features` still exist, confirm `V0026` has not run,
and check for `howitt_schema_migrations` or another pre-existing migration ledger.
Stop and reconcile any contradictory history rather than replacing it.

The authenticated status endpoint returns public table names, the bundled latest
version, and version/name metadata from both the current ledger and a legacy
`refinery_schema_history` ledger when present. It does not return application
rows, SQL, checksums, credentials, or database errors:

```sh
curl --fail-with-body --request POST "$MIGRATION_URL/status" \
  --header "Authorization: Bearer $MIGRATION_ADMIN_TOKEN"
```

With the approved token and the exact URL from deployment in shell variables,
record the reviewed existing schema without executing its SQL:

```sh
curl --fail-with-body --request POST "$MIGRATION_URL/baseline" \
  --header "Authorization: Bearer $MIGRATION_ADMIN_TOKEN" \
  --header 'Content-Type: application/json' \
  --data '{"through":25,"confirmation":"BASELINE EXISTING SCHEMA THROUGH V0025"}'
```

Baselining is allowed only when history is empty, only through an embedded
migration version, and is itself serialized and transactional. Review the rows
marked `baseline`, then invoke normal migration application:

```sh
curl --fail-with-body --request POST "$MIGRATION_URL/apply" \
  --header "Authorization: Bearer $MIGRATION_ADMIN_TOKEN"
```

For the current repository, the expected apply result is version 26. A subsequent
call should succeed with an empty `appliedVersions` array. Independently inspect
the audit table and resulting schema before declaring the migration complete.
Remove the token from shell history/environment after use.

## Routine use and incidents

For each future migration, add one immutable, uniquely versioned SQL file, let CI
validate and embed it, obtain deployment approval for this Worker, review backup
and rollback requirements, then invoke `/apply` under a change record. Worker
deployment and database execution are separate approvals.

If the request fails or the client disconnects, do not infer whether PostgreSQL
committed from the HTTP result alone. Inspect `howitt_schema_migrations` and the
schema, then safely retry `/apply`; committed versions replay as no-ops and an
uncommitted transaction leaves no history. A checksum/history conflict requires
investigation, not manual ledger editing. PostgreSQL rolls back statement errors
before commit, but there are deliberately no automatic down migrations after a
successful commit. Recover a committed destructive change from the reviewed
backup or apply a separately reviewed forward repair. Rotate the admin token and
disable the Worker endpoint if invocation credentials may be compromised.
