import React, { useRef } from "react";
import { ViewOptions } from "ol/View";
import { Style, Stroke, Circle } from "ol/style";
import Fill from "ol/style/Fill";
import OlMap from "ol/Map";
import { css } from "@emotion/react";
import { useMap } from "./hooks/useMap";
import { useTrackLayers } from "./hooks/useTrackLayers";
import { useInitialView } from "./hooks/useInitialView";
import { Marker, Track } from "./types";
import { useMarkerLayers } from "./hooks/useMarkerLayers";

export { PrimaryMapContext } from "./context";

export interface MapProps {
  mapInstance?: OlMap | undefined;
  onNewMapInstance?: (map: OlMap) => void;
  tracks?: Track[];
  markers?: Marker[];
  initialView?:
    | { type: "tracks"; trackIds: string[] }
    | { type: "view"; view: ViewOptions };
  interactive?: boolean;
}

export const DEFAULT_VIEW: ViewOptions = {
  center: [146, -37],
  zoom: 7.5,
  enableRotation: false,
};

export const DEFAULT_INITIAL_VIEW = {
  type: "view" as const,
  view: DEFAULT_VIEW,
};

const mapCss = css`
  width: 100%;
  height: 100%;
`;

export const ROUTE_STYLES = {
  default: new Style({
    stroke: new Stroke({ color: "#a54331", width: 4 }),
  }),
  muted: new Style({
    stroke: new Stroke({ color: "#808080", width: 4 }),
  }),
  highlighted: new Style({
    stroke: new Stroke({ color: "#39abbf", width: 4 }),
  }),
};

export const CHECKPOINT_STYLES = {
  hut: new Style({
    image: new Circle({
      fill: new Fill({ color: "rgba(255,255,255,0.4)" }),
      stroke: new Stroke({ color: "#5e8019", width: 1.25 }),
      radius: 5,
    }),
  }),
  station: new Style({
    image: new Circle({
      fill: new Fill({ color: "rgba(255,255,255,0.4)" }),
      stroke: new Stroke({ color: "#4b6eaf", width: 1.25 }),
      radius: 5,
    }),
  }),
};

export const RIDE_STYLES = {
  default: new Style({
    stroke: new Stroke({ color: "#29892e", width: 4 }),
  }),
  muted: new Style({
    stroke: new Stroke({ color: "#808080", width: 4 }),
  }),
  highlighted: new Style({
    stroke: new Stroke({ color: "#39abbf", width: 4 }),
  }),
};

export function Map({
  tracks = [],
  markers = [],
  initialView,
  mapInstance,
  onNewMapInstance,
  interactive = true,
}: MapProps): React.ReactElement {
  const mapElementRef = useRef<HTMLDivElement>(null);

  const { map } = useMap({
    mapInstance,
    onNewMapInstance,
    mapElementRef,
    interactive,
  });

  useInitialView({ map, tracks, initialView });
  useTrackLayers({ map, tracks });
  useMarkerLayers({ map, markers });

  return <div css={mapCss} ref={mapElementRef} />;
}
