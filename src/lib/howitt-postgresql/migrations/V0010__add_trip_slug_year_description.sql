ALTER TABLE trips
ADD COLUMN slug VARCHAR(255) NOT NULL,
ADD COLUMN year INTEGER NOT NULL,
ADD COLUMN description TEXT;

-- Create a unique constraint on user_id and slug
CREATE UNIQUE INDEX trips_user_id_slug_unique ON trips (user_id, slug);
