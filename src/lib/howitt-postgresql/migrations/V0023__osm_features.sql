-- Enable PostGIS extension if not already enabled
CREATE EXTENSION IF NOT EXISTS postgis;

-- Create a table for storing GeoJSON features
CREATE TABLE osm_highway_features (
    id UUID PRIMARY KEY,
    feature_type VARCHAR(50) NOT NULL,
    properties JSONB NOT NULL,
    geometry GEOMETRY(GEOMETRY, 4326) NOT NULL,  
    geometry_type VARCHAR(50) NOT NULL, -- e.g. MultiLineString, MultiPolygon, Point
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Add indexes for common queries
CREATE INDEX idx_osm_highway_features_geometry ON osm_highway_features USING GIST(geometry);

-- SELECT 
--     id,
--     feature_type,
--     properties,
--     geometry_type,
--     ST_AsGeoJSON(geometry) AS geometry_json,
--     ST_DistanceSphere(
--         geometry,
--         ST_SetSRID(ST_MakePoint(146.7658526343377, -37.33661893790693), 4326)
--     ) AS distance_meters
-- FROM 
--     osm_highway_features
-- ORDER BY 
--     geometry <-> ST_SetSRID(ST_MakePoint(146.7658526343377, -37.33661893790693), 4326)
-- LIMIT 5;

-- INSERT INTO "public"."Query Results" (
--     "id",
--     "feature_type",
--     "properties",
--     "geometry_type",
--     "geometry_json",
--     "distance_meters"
-- )
-- VALUES
-- (
--     E'4b054c95-7ae5-4a25-9cd5-835e150d0c39',
--     E'Feature',
--     E'{"name": "Howitt Road", "source": "surveyed", "highway": "tertiary", "surface": "unpaved", "4wd_only": "yes"}',
--     E'MultiLineString',
--     E'{"type":"MultiLineString","coordinates":[[[146.7839144,-37.419804],[146.783871,-37.419489],...]]]}',
--     2.98692492
-- ),
-- (
--     E'b48b77a7-47ea-4e33-ad72-8f6a55c6e228',
--     E'Feature',
--     E'{"highway": "track"}',
--     E'MultiLineString',
--     E'{"type":"MultiLineString","coordinates":[[[146.7640275,-37.3485592],[146.7628117,-37.3486283],...]]]}',
--     1335.28961677
-- ),
-- (
--     E'5f72a275-ccbe-46e0-9549-b4c79f8daf05',
--     E'Feature',
--     E'{"source": "Bing", "highway": "service"}',
--     E'MultiLineString',
--     E'{"type":"MultiLineString","coordinates":[[[146.7634307,-37.3543673],[146.7637636,-37.3535733],...]]]}',
--     1672.80958716
-- ),
-- (
--     E'9b992ce1-7440-4670-9e94-0efcbdef03f4',
--     E'Feature',
--     E'{"highway": "track", "surface": "unpaved"}',
--     E'MultiLineString',
--     E'{"type":"MultiLineString","coordinates":[[[146.7634307,-37.3543673],[146.7632969,-37.3546671],...]]]}',
--     1985.10965469
-- ),
-- (
--     E'be5c355e-faa4-4fac-afe4-8404495fcd4f',
--     E'Feature',
--     E'{"highway": "track"}',
--     E'MultiLineString',
--     E'{"type":"MultiLineString","coordinates":[[[146.7618034,-37.3579189],[146.7625678,-37.3575253],[146.7630242,-37.3574146]]]}',
--     2325.85310957
-- );

-- -- scratch query
-- WITH ride_segment AS (
--     SELECT ST_GeomFromGeoJSON(
--         '{
--         "coordinates": [
--           [
--             145.01749841634842,
--             -37.78962722016847
--           ],
--           [
--             145.0198619754048,
--             -37.79207038155238
--           ],
--           [
--             145.01995693983048,
--             -37.79265406002954
--           ],
--           [
--             145.0194610144933,
--             -37.793112661313
--           ],
--           [
--             145.0178650845491,
--             -37.79292296690176
--           ],
--           [
--             145.0166490123126,
--             -37.79288336001298
--           ],
--           [
--             145.01451231272216,
--             -37.79232053003594
--           ],
--           [
--             145.01429072906137,
--             -37.79368799344229
--           ],
--           [
--             145.01474444798606,
--             -37.79421329277897
--           ]
--         ],
--         "type": "LineString"
--       }'
--     ) AS geom
-- )
-- SELECT 
--     o.id,
--     o.feature_type,
--     o.properties,
--     o.geometry_type,
--     ST_AsGeoJSON(o.geometry) AS geometry_json,
--     -- Hausdorff distance for shape similarity (lower = more similar)
--     ST_HausdorffDistance(o.geometry, r.geom) AS hausdorff_distance
-- FROM 
--     osm_highway_features o,
--     ride_segment r
-- WHERE 
--     -- Filter to features within reasonable distance for performance
--     ST_DWithin(o.geometry, r.geom, 0.002) -- ~200m in decimal degrees
-- ORDER BY 
--     -- Sort by shape similarity
--     ST_HausdorffDistance(o.geometry, r.geom) ASC
-- LIMIT 5;