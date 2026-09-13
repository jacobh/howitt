#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

# Never use DATABASE_URL: this harness owns and drops a new loopback database.
PG_BIN="${PG_BIN:-/usr/lib/postgresql/15/bin}"
PG_USER="${PG_USER:-$(id -un)}"
database="howitt_workers_test_$(uuidgen | tr -d '-' | tr '[:upper:]' '[:lower:]')"
created=false
cleanup() {
  if [[ "$created" == true ]]; then
    "$PG_BIN/dropdb" -h 127.0.0.1 -p 5432 -U "$PG_USER" --force "$database"
    echo "Dropped disposable database $database"
  fi
}
trap cleanup EXIT
"$PG_BIN/createdb" -h 127.0.0.1 -p 5432 -U "$PG_USER" "$database"
created=true
for migration in src/lib/howitt-postgresql/migrations/*.sql; do
  "$PG_BIN/psql" -X -h 127.0.0.1 -p 5432 -U "$PG_USER" -d "$database" -v ON_ERROR_STOP=1 -q -f "$migration"
done
export HOWITT_TEST_DATABASE_URL="postgresql://$PG_USER:local-only@127.0.0.1:5432/$database?sslmode=disable"
cargo test --locked -p howitt-postgresql --test rwgps_sync -- --ignored --nocapture
bun run build:jobs
bun scripts/test-jobs-local.ts
