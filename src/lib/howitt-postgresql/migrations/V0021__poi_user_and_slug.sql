-- Add new columns
ALTER TABLE points_of_interest 
ADD COLUMN user_id UUID REFERENCES users(id),
ADD COLUMN slug VARCHAR(255),
ADD COLUMN description TEXT;

-- Backfill user_id with default user
UPDATE points_of_interest 
SET user_id = '01941a60-9cfd-c166-94bb-126a6d8de5fd';

-- Make user_id required
ALTER TABLE points_of_interest 
ALTER COLUMN user_id SET NOT NULL;

-- Update existing slugs with improved cleaning
WITH cleaned_slugs AS (
    SELECT
        id,
        regexp_replace(
            regexp_replace(lower(name), '[^a-z0-9-]', '-', 'g'), 
            '-+', 
            '-', 
            'g'
        ) as new_slug,
        row_number() OVER (
            PARTITION BY regexp_replace(
                regexp_replace(lower(name), '[^a-z0-9-]', '-', 'g'),
                '-+',
                '-',
                'g'
            )
            ORDER BY created_at
        ) as duplicate_number
    FROM points_of_interest
)
UPDATE points_of_interest p
SET slug = CASE
    WHEN cs.duplicate_number > 1 
    THEN cs.new_slug || '-' || cs.duplicate_number
    ELSE cs.new_slug
END
FROM cleaned_slugs cs
WHERE p.id = cs.id;

-- Make slug required and unique
ALTER TABLE points_of_interest 
ALTER COLUMN slug SET NOT NULL;
ALTER TABLE points_of_interest 
ADD CONSTRAINT points_of_interest_slug_unique UNIQUE (slug);

-- Add indexes
CREATE INDEX ON points_of_interest (user_id, created_at DESC);