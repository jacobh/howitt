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
import { LineString, Point } from "ol/geom";
import Fill from "ol/style/Fill";
import { css } from "@emotion/react";
import { isNotNil } from "~/services/isNotNil";
import { some } from "lodash";
import { useMap } from "./useMap";
import { Extent } from "ol/extent";
import { match, P } from "ts-pattern";

export { PrimaryMapContext } from "./context";

export interface DisplayedRoute {
  route: Pick<Route, "id" | "pointsJson">;
  style?: "default" | "muted" | "highlighted";
}

export interface MapProps {
  routes?: DisplayedRoute[];
  rides?: Pick<Ride, "id" | "pointsJson">[];
  checkpoints?: Pick<
    PointOfInterest,
    "name" | "point" | "pointOfInterestType"
  >[];
  initialView?:
    | { type: "rides"; rideIds: string[] }
    | { type: "routes"; routeIds: string[] }
    | { type: "view"; view: ViewOptions };
  onVisibleRoutesChanged?: (
    routes: { routeId: string; distanceFromCenter: number }[],
  ) => void;

  onRouteClicked?: (routeId: string | undefined) => void;
}

export const DEFAULT_VIEW: ViewOptions = {
  center: [146, -37],
  zoom: 7.5,
  enableRotation: false,
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
};

export function Map({
  routes,
  rides,
  checkpoints,
  initialView,
  onVisibleRoutesChanged,
  onRouteClicked,
}: MapProps): React.ReactElement {
  const mapElementRef = useRef<HTMLDivElement>(null);

  const { map } = useMap({
    initialView,
    onVisibleRoutesChanged,
    onRouteClicked,
    mapElementRef,
  });

  useEffect(() => {
    if (!map) {
      console.log("no map yet");
      return;
    }

    let initialBounds: Extent | undefined = undefined;

    const layers = map.getLayers().getArray();

    // cleanup any routes/rides that have been dropped
    for (const layer of layers) {
      if (layer instanceof VectorLayer) {
        const vectorLayer = layer as VectorLayer<any>;

        const layerRouteId = vectorLayer.getProperties().routeId;
        const layerRideId = vectorLayer.getProperties().rideId;

        if (isNotNil(layerRouteId)) {
          const isLayerRouteInCurrentRender = some(
            routes,
            ({ route }) => route.id === layerRouteId,
          );

          if (!isLayerRouteInCurrentRender) {
            setInterval(() => {
              map.removeLayer(layer);
            }, 1);
          }
        }

        console.log(
          layerRideId,
          some(rides, (ride) => ride.id === layerRideId),
        );
        if (isNotNil(layerRideId)) {
          const isLayerRideInCurrentRender = some(
            rides,
            (ride) => ride.id === layerRideId,
          );

          if (!isLayerRideInCurrentRender) {
            setInterval(() => {
              map.removeLayer(layer);
            }, 1);
          }
        }
      }
    }

    for (const { route, style } of routes ?? []) {
      // console.log(route.id);
      const existingLayer = layers
        .filter((x): x is VectorLayer<any> => x instanceof VectorLayer)
        .find((layer) => layer.getProperties().routeId === route.id);
      let layer: VectorLayer<any>;

      if (existingLayer === undefined) {
        const lineString = new LineString(JSON.parse(route.pointsJson));

        layer = new VectorLayer({
          source: new VectorSource({
            features: [
              new Feature({ geometry: lineString, routeId: route.id }),
            ],
          }),
          properties: { routeId: route.id, points: route.pointsJson.length },
        });

        map.addLayer(layer);
      } else {
        layer = existingLayer;
      }

      if (layer.getProperties().points !== route.pointsJson.length) {
        const newLineString = new LineString(JSON.parse(route.pointsJson));

        layer.setSource(
          new VectorSource({
            features: [
              new Feature({ geometry: newLineString, routeId: route.id }),
            ],
          }),
        );

        layer.setProperties({
          routeId: route.id,
          points: route.pointsJson.length,
        });
      }

      layer.setStyle(
        match(style)
          .with("muted", () => ROUTE_STYLES.muted)
          .with("highlighted", () => ROUTE_STYLES.highlighted)
          .with(P.union("default", undefined), () => ROUTE_STYLES.default)
          .exhaustive(),
      );

      if (
        initialView?.type === "routes" &&
        initialView.routeIds.includes(route.id)
      ) {
        const lineString = new LineString(JSON.parse(route.pointsJson));
        if (!initialBounds) {
          initialBounds = lineString.getExtent();
        } else {
          initialBounds = [
            Math.min(initialBounds[0], lineString.getExtent()[0]),
            Math.min(initialBounds[1], lineString.getExtent()[1]),
            Math.max(initialBounds[2], lineString.getExtent()[2]),
            Math.max(initialBounds[3], lineString.getExtent()[3]),
          ];
        }
      }
    }

    for (const ride of rides ?? []) {
      // console.log(route.id);
      const existingLayer = layers
        .filter((x): x is VectorLayer<any> => x instanceof VectorLayer)
        .find((layer) => layer.getProperties().rideId === ride.id);
      let layer: VectorLayer<any>;

      if (existingLayer === undefined) {
        const lineString = new LineString(JSON.parse(ride.pointsJson));

        layer = new VectorLayer({
          source: new VectorSource({
            features: [new Feature({ geometry: lineString, rideId: ride.id })],
          }),
          properties: { rideId: ride.id, points: ride.pointsJson.length },
        });

        console.log("adding layer");
        map.addLayer(layer);
      } else {
        layer = existingLayer;
      }

      if (layer.getProperties().points !== ride.pointsJson.length) {
        const newLineString = new LineString(JSON.parse(ride.pointsJson));

        layer.setSource(
          new VectorSource({
            features: [
              new Feature({ geometry: newLineString, rideId: ride.id }),
            ],
          }),
        );

        layer.setProperties({
          rideId: ride.id,
          points: ride.pointsJson.length,
        });
      }

      layer.setStyle(RIDE_STYLES.default);

      if (
        initialView?.type === "rides" &&
        initialView.rideIds.includes(ride.id)
      ) {
        const lineString = new LineString(JSON.parse(ride.pointsJson));
        if (!initialBounds) {
          initialBounds = lineString.getExtent();
        } else {
          initialBounds = [
            Math.min(initialBounds[0], lineString.getExtent()[0]),
            Math.min(initialBounds[1], lineString.getExtent()[1]),
            Math.max(initialBounds[2], lineString.getExtent()[2]),
            Math.max(initialBounds[3], lineString.getExtent()[3]),
          ];
        }
      }
    }

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

    if (initialBounds) {
      map.getView().fit(initialBounds, {
        padding: [100, 100, 100, 100],
        duration: 0,
      });
    }
  }, [routes, checkpoints, map, initialView, rides]);

  return <div css={mapCss} ref={mapElementRef} />;
}
