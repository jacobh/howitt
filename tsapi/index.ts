import { Hono } from "hono";
import { sql } from "bun";
import type { Feature } from "geojson";

const app = new Hono();

const featureIndexQuery = sql`
    WITH feature_observation_counts AS (
        SELECT 
            osm_features.id, 
            COUNT(*) AS count 
        FROM osm_features 
        INNER JOIN water_beta ON osm_features.id = water_beta.osm_feature_id 
        GROUP BY osm_features.id
    )
    SELECT 
        osm_features.id,
        ST_AsGeoJSON(osm_features.geometry) AS geometry,
        osm_features.name,
        count AS observation_count,
        osm_features.attributes
    FROM osm_features 
    INNER JOIN feature_observation_counts ON osm_features.id = feature_observation_counts.id;
`;

function parseRowToFeature(row: any): Feature {
  return {
    type: "Feature",
    geometry: JSON.parse(row.geometry),
    properties: {
      id: Number(row.id),
      name: row.name,
      observation_count: Number(row.observation_count),
      attributes: row.attributes,
    },
  };
}

app.get("/api/water-features", async (c) => {
  const res = await featureIndexQuery.execute();

  const parsed = res.map(parseRowToFeature);

  return c.json({ type: "FeatureCollection", features: parsed });
});

export default {
  port: 3001,
  fetch: app.fetch,
};
