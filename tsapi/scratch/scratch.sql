-- CREATE TABLE osm_features (
--     id BIGINT PRIMARY KEY,
--     geometry GEOMETRY(GEOMETRY, 4326),
--     name TEXT,
--     attributes JSONB
-- );

-- CREATE INDEX osm_features_geom_idx ON osm_features USING GIST(geometry);

-- CREATE TABLE water_beta (
--     id UUID PRIMARY KEY,
--     osm_feature_id BIGINT REFERENCES osm_features(id),
--     attributes JSONB
-- );

-- CREATE INDEX water_beta_osm_feature_id_idx ON water_beta(osm_feature_id);


select 
    osm_features.id,
    ST_AsGeoJSON(osm_features.geometry) as geojson,
    osm_features.name,
    osm_features.attributes
from osm_features 
inner join water_beta on osm_features.id = water_beta.osm_feature_id;

-- id	geometry	name	attributes
-- 508105972	0101000020E610000017B5A09C0D54624099171692828642C0	Lake Cobbler	{"type": "node", "tourism": "camp_site"}
-- 3068360649	0101000020E6100000BB5A9313D24E62402BCBC639A0CE42C0	Deep Saddle	{"type": "node", "natural": "saddle"}
-- 10855407006	0101000020E6100000756671B5C44D62401316702AAD9242C0	Buller Huts Trail	{"type": "node", "tourism": "attraction", "website": "https://www.bullerhutstrail.com.au", "opening_hours": "24/7"}

with feature_observation_counts as (
	select osm_features.id, count(*) as count from osm_features inner join water_beta on osm_features.id = water_beta.osm_feature_id group by osm_features.id
)
select 
    osm_features.id,
    ST_AsGeoJSON(osm_features.geometry) as geometry,
    osm_features.name,
    count as observation_count,
    osm_features.attributes
from osm_features 
inner join feature_observation_counts on osm_features.id = feature_observation_counts.id;

-- id	geometry	name	observation_count	attributes
-- 55453570	{"type":"Point","coordinates":[149.0660079,-35.8237005]}	Horse Gully Hut	3	{"type": "node", "tourism": "alpine_hut", "building": "hut"}
-- 179245715	{"type":"Point","coordinates":[148.2686647,-36.4576706]}	Rawson Pass	7	{"ele": "2124", "type": "node", "natural": "saddle", "mountain_pass": "yes"}
-- 251147370	{"type":"Point","coordinates":[147.1824914,-36.9486891]}	Dibbins Hut	16	{"type": "node", "amenity": "shelter", "tourism": "wilderness_hut", "shelter_type": "weather_shelter"}
-- 251154472	{"type":"Point","coordinates":[147.1217582,-36.9765032]}	Diamantina Hut	9	{"type": "node", "amenity": "shelter", "tourism": "wilderness_hut", "shelter_type": "weather_shelter"}