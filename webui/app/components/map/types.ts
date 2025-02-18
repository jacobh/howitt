import { PointOfInterestType } from "~/__generated__/graphql";

export interface Track {
  id: string;
  kind: "ride" | "route";
  points: [number, number][];
  style?: "default" | "muted" | "highlighted";
}

export interface Marker {
  id: string;
  point: [number, number];
  label?: string;
  style?: "default" | "muted" | "highlighted";
}

type TrackLike = { id: string; pointsJson: string };

type PointOfInterestLike = {
  id: string;
  name: string;
  point: number[];
  pointOfInterestType: PointOfInterestType;
};
export function buildTrack(
  { id, pointsJson }: TrackLike,
  opts: Pick<Track, "kind" | "style">,
): Track {
  return {
    id,
    kind: opts.kind,
    points: JSON.parse(pointsJson),
    style: opts.style,
  };
}

export function buildRouteTrack(
  route: TrackLike,
  style?: Track["style"],
): Track {
  return buildTrack(route, { kind: "route", style });
}

export function buildRideTrack(ride: TrackLike, style?: Track["style"]): Track {
  return buildTrack(ride, { kind: "ride", style });
}

export function buildMarker(
  { id, point, name }: PointOfInterestLike,
  style?: Marker["style"],
): Marker {
  return {
    id,
    point: [point[0], point[1]],
    label: name,
    style,
  };
}
