export interface Track {
  id: string;
  kind: "ride" | "route";
  points: [number, number][];
  style?: "default" | "muted" | "highlighted";
}
