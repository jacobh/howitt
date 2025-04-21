CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE osm_features (
    id BIGINT PRIMARY KEY,
    geometry GEOMETRY(GEOMETRY, 4326),
    name TEXT,
    attributes JSONB
);

CREATE INDEX osm_features_geom_idx ON osm_features USING GIST(geometry);

CREATE TABLE water_beta (
    id UUID PRIMARY KEY,
    osm_feature_id BIGINT REFERENCES osm_features(id),
    attributes JSONB
);

CREATE INDEX water_beta_osm_feature_id_idx ON water_beta(osm_feature_id);
