CREATE TABLE osm_features (
    id SERIAL PRIMARY KEY,
    geometry GEOMETRY(GEOMETRY, 4326),
    name TEXT,
    attributes JSONB
);

CREATE INDEX osm_features_geom_idx ON osm_features USING GIST(geometry);
