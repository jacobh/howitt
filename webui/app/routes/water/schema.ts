import type { Feature, FeatureCollection, Geometry, Point } from "geojson";

// Types for /api/water-features
export interface WaterFeatureProperties {
  id: number;
  name: string;
  observation_count: number;
  attributes: Record<string, unknown>;
}

export type WaterFeature = Feature<Geometry, WaterFeatureProperties>;

export type WaterFeaturesResponse = FeatureCollection<
  WaterFeature["geometry"],
  WaterFeature["properties"]
>;

// Types for /api/water-features/query
export interface WaterObservation {
  date: string;
  season: string;
  quality: string;
  available: string;
  related_post_ids: number[];
  observation_notes: string;
}

export interface WaterBetaTopic {
  topic_id: number;
  name: string;
  osm_feature_id: number;
  general_notes: string;
  observations: WaterObservation[];
}

export interface NearbyFeatureProperties {
  id: number;
  name: string;
  distance_meters: number;
  attributes: Record<string, unknown>;
  waterBeta: WaterBetaTopic[];
}

export type NearbyFeature = Feature<Geometry, NearbyFeatureProperties>;

export type NearbyFeaturesResponse = FeatureCollection<
  NearbyFeature["geometry"],
  NearbyFeature["properties"]
>;

// Query parameters for /api/water-features/query
export interface WaterFeaturesQueryParams {
  origin: string; // formatted as "lon,lat"
  radius?: number; // optional, defaults to 1000 meters
}

// Parsed query parameters
export interface ParsedWaterFeaturesQueryParams {
  origin: Point;
  radius: number;
}

// Types for /api/now
export interface NowResponse {
  now: string; // ISO timestamp
}
