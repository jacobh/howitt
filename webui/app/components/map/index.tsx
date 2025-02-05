import React, { useEffect, useRef } from "react";
import { ViewOptions } from "ol/View";
import {
  Route,
  Ride,
  PointOfInterest,
  PointOfInterestType,
} from "../../__generated__/graphql";
import { Feature } from "ol";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import { Style, Stroke, Circle } from "ol/style";
import { Point } from "ol/geom";
import Fill from "ol/style/Fill";
import OlMap from "ol/Map";
import { css } from "@emotion/react";
import { match, P } from "ts-pattern";
import { useMap } from "./hooks/useMap";
import { useTrackLayers } from "./hooks/useTrackLayers";
import { useInitialView } from "./hooks/useInitialView";
import { Track } from "./types";

export { PrimaryMapContext } from "./context";

export interface MapProps {
  mapInstance?: OlMap | undefined;
  onNewMapInstance?: (map: OlMap) => void;
  tracks?: Track[];
  checkpoints?: Pick<
    PointOfInterest,
    "name" | "point" | "pointOfInterestType"
  >[];
  initialView?:
    | { type: "tracks"; trackIds: string[] }
    | { type: "view"; view: ViewOptions };
  onVisibleRoutesChanged?: (
    routes: { routeId: string; distanceFromCenter: number }[],
  ) => void;
  interactive?: boolean;

  onRouteClicked?: (routeId: string | undefined) => void;
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
  checkpoints,
  initialView,
  onVisibleRoutesChanged,
  onRouteClicked,
  mapInstance,
  onNewMapInstance,
  interactive = true,
}: MapProps): React.ReactElement {
  const mapElementRef = useRef<HTMLDivElement>(null);

  const { map } = useMap({
    mapInstance,
    onNewMapInstance,
    onVisibleRoutesChanged,
    onRouteClicked,
    mapElementRef,
    interactive,
  });

  useInitialView({ map, tracks, initialView });
  useTrackLayers({ map, tracks });

  useEffect(() => {
    if (!map) {
      console.log("no map yet");
      return;
    }

    const layers = map.getLayers().getArray();

    for (const checkpoint of checkpoints ?? []) {
      console.log(checkpoint.name);
      const existingLayer = layers.find(
        (layer) => layer.getProperties().checkpointName === checkpoint.name,
      );

      if (existingLayer === undefined) {
        map.addLayer(
          new VectorLayer({
            source: new VectorSource({
              features: [new Feature(new Point(checkpoint.point))],
            }),
            properties: { checkpointName: checkpoint.name },
            style: match(checkpoint.pointOfInterestType)
              .with(PointOfInterestType.Hut, () => CHECKPOINT_STYLES.hut)
              .with(
                PointOfInterestType.RailwayStation,
                () => CHECKPOINT_STYLES.station,
              )
              .otherwise(() => CHECKPOINT_STYLES.station),
          }),
        );
      }
    }
  }, [checkpoints, map, initialView]);

  return <div css={mapCss} ref={mapElementRef} />;
}
