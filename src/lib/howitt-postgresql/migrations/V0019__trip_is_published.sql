-- V00XX__add_trip_published.sql
ALTER TABLE trips
ADD COLUMN is_published BOOLEAN NOT NULL DEFAULT FALSE;

-- Set existing trips to published for backward compatibility (optional)
UPDATE trips SET is_published = TRUE;

-- Add an index for efficient querying of published trips
CREATE INDEX ON trips (is_published);
