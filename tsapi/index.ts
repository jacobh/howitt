import { Hono } from "hono";
import { sql } from "bun";
import type { Feature, Geometry, Point } from "geojson";
import { z } from "zod";
import { match, P } from "ts-pattern";

function parseGeometry(geomString: string): Geometry {
  return JSON.parse(geomString) as Geometry;
}

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

function parseIndexRowToFeature(row: any): Feature<
  Geometry,
  {
    id: number;
    name: string;
    observation_count: number;
    attributes: Record<string, unknown>;
  }
> {
  return {
    type: "Feature",
    geometry: parseGeometry(row.geometry),
    properties: {
      id: Number(row.id),
      name: row.name,
      observation_count: Number(row.observation_count),
      attributes: row.attributes,
    },
  };
}

function makeNearbyFeatureQuery({
  origin,
  radius,
}: {
  origin: Point;
  radius: number;
}): Bun.SQLQuery {
  const [lon, lat] = origin.coordinates;

  return sql`SELECT 
      osm_features.id,
      ST_AsGeoJSON(osm_features.geometry) as geometry,
      osm_features.name,
      ST_Distance(
          osm_features.geometry, 
          ST_SetSRID(ST_MakePoint(${lon}, ${lat}), 4326)::geography
      ) as distance_meters,
      osm_features.attributes
  FROM osm_features
  WHERE ST_DWithin(
      osm_features.geometry::geography, 
      ST_SetSRID(ST_MakePoint(${lon}, ${lat}), 4326)::geography, 
      ${radius}
  )
  ORDER BY distance_meters;`;
}

function parseNearbyToFeature(row: any): Feature<
  Geometry,
  {
    id: number;
    name: string;
    distance_meters: number;
    attributes: Record<string, unknown>;
  }
> {
  return {
    type: "Feature",
    geometry: parseGeometry(row.geometry),
    properties: {
      id: Number(row.id),
      name: row.name,
      distance_meters: Number(row.distance_meters),
      attributes: row.attributes,
    },
  };
}

function makeWaterBetaQuery(osmFeatureIds: number[]): Bun.SQLQuery {
  return sql`
      select
        osm_feature_id,
        attributes->'date' as date,
        attributes->'name' as name,
        attributes->'season' as season,
        attributes->'quality' as quality,
        attributes->'topic_id' as topic_id,
        attributes->'available' as available,
        attributes->'related_post_ids' as related_post_ids,
        attributes->'observation_notes' as observation_notes,
        /* attributes->'general_notes_pass1' as general_notes_pass1, */
        attributes->'general_notes_pass2' as general_notes
      from water_beta
      where osm_feature_id in ${sql(osmFeatureIds)}`;
}

interface WaterBetaRow {
  osm_feature_id: number;
  date: string;
  name: string;
  season: string;
  quality: string;
  topic_id: number;
  available: string;
  related_post_ids: number[];
  observation_notes: string;
  general_notes: string;
}

function parseWaterBetaRow(row: any): WaterBetaRow {
  return {
    osm_feature_id: Number(row.osm_feature_id),
    date: row.date,
    name: row.name,
    season: row.season,
    quality: row.quality,
    topic_id: Number(row.topic_id),
    available: row.available,
    related_post_ids: Array.isArray(row.related_post_ids)
      ? row.related_post_ids.map(Number)
      : [],
    observation_notes: row.observation_notes || "",
    general_notes: row.general_notes || "",
  };
}

const app = new Hono();

app.get("/api/water-features", async (c) => {
  const res: unknown[] = await featureIndexQuery.execute();

  const parsed = res.map(parseIndexRowToFeature);

  return c.json({ type: "FeatureCollection", features: parsed });
});

// lon,lat
const PointString = z.string().transform((s): Point => {
  const [lon, lat] = s.split(",").map(Number);

  return match({ lon, lat })
    .with(
      { lon: P.number.between(-180, 180), lat: P.number.between(-90, 90) },
      ({ lon, lat }) => ({
        type: "Point" as const,
        coordinates: [lon, lat],
      })
    )
    .otherwise(() => {
      throw new Error("Invalid point string");
    });
});

const QueryParams = z.object({
  origin: PointString,
  radius: z.coerce.number().optional(),
});

function groupWaterBetaByTopic(
  rows: WaterBetaRow[]
): Record<string, unknown>[] {
  const map: Map<number, WaterBetaRow[]> = new Map(
    rows.map((wb) => [wb.topic_id, []])
  );

  for (const waterBeta of rows) {
    map.get(waterBeta.topic_id)?.push(waterBeta);
  }

  return map
    .entries()
    .map(
      ([topic_id, [firstRow, ...otherRows]]): Record<string, unknown> => ({
        topic_id,
        name: firstRow?.name,
        osm_feature_id: firstRow?.osm_feature_id,
        general_notes: firstRow?.general_notes,
        observations: [firstRow, ...otherRows]
          .filter((x) => x !== undefined)
          .map(
            ({
              date,
              season,
              quality,
              available,
              related_post_ids,
              observation_notes,
            }) => ({
              date,
              season,
              quality,
              available,
              related_post_ids,
              observation_notes,
            })
          ),
      })
    )
    .toArray();
}

app.get("/api/water-features/query", async (c) => {
  const { origin, radius } = QueryParams.parse(c.req.query()); // Parse the query parameters

  const nearbyFeatures = await (async () => {
    const res: unknown[] = await makeNearbyFeatureQuery({
      origin,
      radius: radius ?? 1000,
    }).execute();

    return res.map(parseNearbyToFeature);
  })();

  const waterBetaRows = await (async () => {
    const waterBeta: unknown[] = await makeWaterBetaQuery(
      nearbyFeatures.map(({ properties: { id } }) => id)
    ).execute();

    return waterBeta.map(parseWaterBetaRow);
  })();

  const waterBetaByFeatureId = (() => {
    const map: Map<number, WaterBetaRow[]> = new Map(
      waterBetaRows.map((wb) => [wb.osm_feature_id, []])
    );

    for (const waterBeta of waterBetaRows) {
      map.get(waterBeta.osm_feature_id)?.push(waterBeta);
    }

    return map;
  })();

  return c.json({
    type: "FeatureCollection",
    features: nearbyFeatures.map((feature) => ({
      ...feature,
      properties: {
        ...feature.properties,
        waterBeta: groupWaterBetaByTopic(
          waterBetaByFeatureId.get(feature.properties.id) ?? []
        ),
      },
    })),
  });
});

export default {
  port: 3001,
  fetch: app.fetch,
};
