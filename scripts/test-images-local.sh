#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

# Own a disposable database, never DATABASE_URL or production credentials.
PG_BIN="${PG_BIN:-/usr/lib/postgresql/15/bin}"
PG_USER="${PG_USER:-$(id -un)}"
database="howitt_workers_test_$(uuidgen | tr -d '-' | tr '[:upper:]' '[:lower:]')"
"$PG_BIN/createdb" -h 127.0.0.1 -p 5432 -U "$PG_USER" "$database"
cleanup() {
  "$PG_BIN/dropdb" -h 127.0.0.1 -p 5432 -U "$PG_USER" --force "$database"
  echo "Dropped disposable database $database"
}
trap cleanup EXIT
for migration in src/lib/howitt-postgresql/migrations/*.sql; do
  "$PG_BIN/psql" -X -h 127.0.0.1 -p 5432 -U "$PG_USER" -d "$database" -v ON_ERROR_STOP=1 -q -f "$migration"
done
export HOWITT_TEST_DATABASE_URL="postgresql://$PG_USER:local-only@127.0.0.1:5432/$database?sslmode=disable"
bun run build:worker
bun scripts/test-images-local.ts
