-- Create the point of interest visits table
CREATE TABLE point_of_interest_visits (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id UUID NOT NULL REFERENCES users(id),
    point_of_interest_id UUID NOT NULL REFERENCES points_of_interest(id),
    visited_at TIMESTAMPTZ NOT NULL,
    status VARCHAR(255) NOT NULL,
    comment TEXT
);

-- Create the join table for visits and media
CREATE TABLE point_of_interest_visit_media (
    visit_id UUID NOT NULL REFERENCES point_of_interest_visits(id),
    media_id UUID NOT NULL REFERENCES media(id),
    PRIMARY KEY (visit_id, media_id)
);

-- Create indexes
CREATE INDEX ON point_of_interest_visits (user_id, visited_at DESC);
CREATE INDEX ON point_of_interest_visits (point_of_interest_id, visited_at DESC);
CREATE INDEX ON point_of_interest_visit_media (visit_id);
CREATE INDEX ON point_of_interest_visit_media (media_id);

-- Update media_relations view to include POI visit IDs
DROP VIEW media_relations;
CREATE VIEW media_relations AS
SELECT 
    m.id,
    COALESCE(array_agg(DISTINCT rm.ride_id) FILTER (WHERE rm.ride_id IS NOT NULL), ARRAY[]::uuid[]) as ride_ids,
    COALESCE(array_agg(DISTINCT rtm.route_id) FILTER (WHERE rtm.route_id IS NOT NULL), ARRAY[]::uuid[]) as route_ids,
    COALESCE(array_agg(DISTINCT tm.trip_id) FILTER (WHERE tm.trip_id IS NOT NULL), ARRAY[]::uuid[]) as trip_ids,
    COALESCE(array_agg(DISTINCT pm.poi_id) FILTER (WHERE pm.poi_id IS NOT NULL), ARRAY[]::uuid[]) as poi_ids,
    COALESCE(array_agg(DISTINCT pvm.visit_id) FILTER (WHERE pvm.visit_id IS NOT NULL), ARRAY[]::uuid[]) as poi_visit_ids
FROM media m
LEFT JOIN ride_media rm ON rm.media_id = m.id
LEFT JOIN route_media rtm ON rtm.media_id = m.id
LEFT JOIN trip_media tm ON tm.media_id = m.id
LEFT JOIN poi_media pm ON pm.media_id = m.id
LEFT JOIN point_of_interest_visit_media pvm ON pvm.media_id = m.id
GROUP BY m.id;
