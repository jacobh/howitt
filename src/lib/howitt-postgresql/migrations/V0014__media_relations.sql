-- Create a view that combines media relationships
CREATE VIEW media_relations AS
SELECT 
    m.id,
    COALESCE(array_agg(DISTINCT rm.ride_id) FILTER (WHERE rm.ride_id IS NOT NULL), ARRAY[]::uuid[]) as ride_ids,
    COALESCE(array_agg(DISTINCT rtm.route_id) FILTER (WHERE rtm.route_id IS NOT NULL), ARRAY[]::uuid[]) as route_ids,
    COALESCE(array_agg(DISTINCT tm.trip_id) FILTER (WHERE tm.trip_id IS NOT NULL), ARRAY[]::uuid[]) as trip_ids,
    COALESCE(array_agg(DISTINCT pm.poi_id) FILTER (WHERE pm.poi_id IS NOT NULL), ARRAY[]::uuid[]) as poi_ids
FROM media m
LEFT JOIN ride_media rm ON rm.media_id = m.id
LEFT JOIN route_media rtm ON rtm.media_id = m.id
LEFT JOIN trip_media tm ON tm.media_id = m.id
LEFT JOIN poi_media pm ON pm.media_id = m.id
GROUP BY m.id;
