export interface Track {
  id: string;
  kind: "ride" | "route";
  points: [number, number][];
  style?: "default" | "muted" | "highlighted";
}

type TrackLike = { id: string; pointsJson: string };

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
