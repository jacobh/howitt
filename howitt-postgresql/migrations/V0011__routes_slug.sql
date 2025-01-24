-- Add slug column
ALTER TABLE routes ADD COLUMN slug VARCHAR(255);

-- Update existing slugs with improved cleaning
with
    cleaned_slugs as (
        select
            id,
            regexp_replace(
                regexp_replace(lower(name), '[^a-z0-9-]', '-', 'g'), '-+', '-', 'g'
            ) as new_slug,
            row_number() over (
                partition by
                    regexp_replace(
                        regexp_replace(lower(name), '[^a-z0-9-]', '-', 'g'),
                        '-+',
                        '-',
                        'g'
                    )
                order by created_at
            ) as duplicate_number
        from routes
    )
    update routes r
    set slug = case
        when cs.duplicate_number > 1
        then cs.new_slug || '-' || cs.duplicate_number
        else cs.new_slug
    end
from cleaned_slugs cs
where r.id = cs.id
;

-- Make slug mandatory and unique
ALTER TABLE routes ALTER COLUMN slug SET NOT NULL;
ALTER TABLE routes ADD CONSTRAINT routes_slug_unique UNIQUE (slug);
