#!/bin/bash
# Unscheduled PostgreSQL backup. Requires pg_dump, zstd, AWS CLI and approval
# before reading a shared database or writing to the production backup bucket.
set -euo pipefail

: "${DATABASE_URL:?Set DATABASE_URL to the database to back up}"
export AWS_REGION="${AWS_REGION:-ap-southeast-4}"

read -r DAY WEEKDAY DATE_TIME <<< "$(date -u '+%d %u %Y-%m-%d_%H-%M-%S')"
BACKUP_FILE=$(mktemp)
trap 'rm -f "$BACKUP_FILE"' EXIT

pg_dump "$DATABASE_URL" | zstd -12 --long > "$BACKUP_FILE"

aws s3 cp "$BACKUP_FILE" \
  "s3://howitt-backups/postgresql/daily/$DATE_TIME.sql.zst"

if [ "$WEEKDAY" = "1" ]; then
  aws s3 cp "$BACKUP_FILE" \
    "s3://howitt-backups/postgresql/weekly/$DATE_TIME.sql.zst"
fi

if [ "$DAY" = "01" ]; then
  aws s3 cp "$BACKUP_FILE" \
    "s3://howitt-backups/postgresql/monthly/$DATE_TIME.sql.zst"
fi
