-- Add point and captured_at columns to media table
ALTER TABLE media
    ADD COLUMN point JSONB,
    ADD COLUMN captured_at TIMESTAMPTZ;

-- Create index on captured_at for temporal queries
CREATE INDEX media_captured_at_idx ON media (captured_at DESC);
