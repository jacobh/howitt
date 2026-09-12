#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

# This harness never reads DATABASE_URL. All writes are confined to a new,
# uniquely named loopback database owned by this invocation.
PG_BIN="${PG_BIN:-/Applications/Postgres.app/Contents/Versions/latest/bin}"
PG_USER="${PG_USER:-$(id -un)}"
database="howitt_workers_test_$(uuidgen | tr -d '-' | tr '[:upper:]' '[:lower:]')"
worker_pid=""
# Isolate KV too: fixed fixture IDs must not hit a previous test run's cache.
state_dir="$(mktemp -d /tmp/howitt-worker-state.XXXXXX)"
created=false
cleanup() {
  if [[ -n "$worker_pid" ]]; then kill "$worker_pid" 2>/dev/null || true; wait "$worker_pid" 2>/dev/null || true; fi
  if [[ "$created" == true ]]; then
    "$PG_BIN/dropdb" -h 127.0.0.1 -p 5432 -U "$PG_USER" --force "$database"
    echo "Dropped disposable database $database"
  fi
  rm -rf -- "$state_dir"
}
trap cleanup EXIT
"$PG_BIN/createdb" -h 127.0.0.1 -p 5432 -U "$PG_USER" "$database"
created=true
identity="$("$PG_BIN/psql" -X -h 127.0.0.1 -p 5432 -U "$PG_USER" -d "$database" -Atc "select current_database() || '|' || host(inet_server_addr())")"
[[ "$identity" == "$database|127.0.0.1" ]]
for migration in src/lib/howitt-postgresql/migrations/*.sql; do
  "$PG_BIN/psql" -X -h 127.0.0.1 -p 5432 -U "$PG_USER" -d "$database" -v ON_ERROR_STOP=1 -q -f "$migration"
done
export HOWITT_TEST_DATABASE_URL="postgresql://$PG_USER:local-only@127.0.0.1:5432/$database?sslmode=disable"
cargo test -p howitt-postgresql --test worker_compatibility -- --ignored --nocapture

export CLOUDFLARE_HYPERDRIVE_LOCAL_CONNECTION_STRING_HYPERDRIVE="$HOWITT_TEST_DATABASE_URL"
bun run dev:worker --local --persist-to "$state_dir" --ip 127.0.0.1 --port 8789 --log-level info --var JWT_SECRET:local-test-signing-key > /tmp/howitt-local-worker.log 2>&1 &
worker_pid=$!
bun scripts/test-worker-local.ts
