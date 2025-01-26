-- Create a view that combines trip relationships
CREATE VIEW trip_relations AS
SELECT 
    t.id,
    COALESCE(array_agg(DISTINCT tr.ride_id) FILTER (WHERE tr.ride_id IS NOT NULL), ARRAY[]::uuid[]) as ride_ids,
    COALESCE(array_agg(DISTINCT tm.media_id) FILTER (WHERE tm.media_id IS NOT NULL), ARRAY[]::uuid[]) as media_ids
FROM trips t
LEFT JOIN trip_rides tr ON tr.trip_id = t.id
LEFT JOIN trip_media tm ON tm.trip_id = t.id
GROUP BY t.id;
