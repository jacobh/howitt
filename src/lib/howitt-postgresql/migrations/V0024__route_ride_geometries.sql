CREATE OR REPLACE VIEW route_geometries AS 
SELECT 
    rp.route_id,
    ST_SetSRID(
        ST_MakeLine(
            ARRAY(
                SELECT ST_MakePoint(
                    (p->0)::numeric, -- X coordinate (longitude)
                    (p->1)::numeric, -- Y coordinate (latitude)
                    (p->2)::numeric  -- Z coordinate (elevation)
                )
                FROM jsonb_array_elements(rp.points) AS p
            )
        ),
        4326  -- Set the SRID to match your osm_highway_features table
    ) AS geometry
FROM 
    route_points rp;

CREATE OR REPLACE VIEW ride_geometries AS 
SELECT 
    rp.ride_id,
    ST_SetSRID(
        ST_MakeLine(
            ARRAY(
                SELECT ST_MakePoint(
                    (p->1)::numeric, -- X coordinate (longitude)
                    (p->2)::numeric, -- Y coordinate (latitude)
                    (p->3)::numeric  -- Z coordinate (elevation)
                )
                FROM jsonb_array_elements(rp.points) AS p
            )
        ),
        4326  -- Set the SRID to match your osm_highway_features table
    ) AS geometry
FROM 
    ride_points rp;
