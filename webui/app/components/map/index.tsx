import React, { createContext, useContext, useEffect, useMemo } from "react";
import OlMap from "ol/Map";
import View, { ViewOptions } from "ol/View";
import TileLayer from "ol/layer/Tile";
import XYZ from "ol/source/XYZ";
import { useGeographic } from "ol/proj";
import {
  Route,
  Ride,
  PointOfInterest,
  PointOfInterestType,
} from "../../__generated__/graphql";
import { Feature, MapBrowserEvent } from "ol";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import { Style, Stroke, Circle } from "ol/style";
import { LineString, Point } from "ol/geom";
import Fill from "ol/style/Fill";
import { css } from "@emotion/react";

export interface DisplayedRoute {
  route: Pick<Route, "id" | "points">;
  style?: "default" | "muted" | "highlighted";
}

interface MapProps {
  routes?: DisplayedRoute[];
  rides?: Pick<Ride, "id" | "points">[];
  checkpoints?: Pick<
    PointOfInterest,
    "name" | "point" | "pointOfInterestType"
  >[];
  initialView?:
    | { type: "route"; routeId: string }
    | { type: "view"; view: ViewOptions };
}

interface MapContext {
  map?: OlMap | undefined;
  setMap: (map: OlMap) => void;
}

export const MapContext = createContext<MapContext>({ setMap: () => {} });

export const DEFAULT_VIEW: ViewOptions = {
  center: [146, -37],
  zoom: 7.5,
};

const mapCss = css`
  width: 100%;
  height: 100%;
`;

export function Map({
  routes,
  rides,
  checkpoints,
  initialView,
}: MapProps): React.ReactElement {
  const { map: existingMap, setMap } = useContext(MapContext);

  const hutStyle = useMemo<Style>(
    () =>
      new Style({
        image: new Circle({
          fill: new Fill({
            color: "rgba(255,255,255,0.4)",
          }),
          stroke: new Stroke({
            color: "#5e8019",
            width: 1.25,
          }),
          radius: 5,
        }),
      }),
    []
  );

  const stationStyle = useMemo<Style>(
    () =>
      new Style({
        image: new Circle({
          fill: new Fill({
            color: "rgba(255,255,255,0.4)",
          }),
          stroke: new Stroke({
            color: "#4b6eaf",
            width: 1.25,
          }),
          radius: 5,
        }),
      }),
    []
  );

  useEffect(() => {
    let map: OlMap;

    if (existingMap instanceof OlMap) {
      console.log("map already rendered");

      map = existingMap;

      existingMap.setTarget("map");
    } else {
      console.log("initial map render");

      // eslint-disable-next-line react-hooks/rules-of-hooks
      useGeographic();

      const newMap = new OlMap({
        target: "map",
        layers: [
          new TileLayer({
            preload: Infinity,
            source: new XYZ({
              urls: [
                "https://d2o31mmlexa59r.cloudfront.net/landscape/{z}/{x}/{y}.png?apikey=f1165310fdfb499d9793b076ed26c08e",
              ],
            }),
          }),
        ],
      });

      setMap(newMap);

      map = newMap;
    }

    if (initialView?.type === "view") {
      map.setView(new View(initialView.view));
    }
    if (existingMap === undefined) {
      map.setView(new View(DEFAULT_VIEW));
    }

    const clickListener = (event: MapBrowserEvent<any>): void => {
      console.log(event.coordinate);
      console.log(map.getFeaturesAtPixel(event.pixel, { hitTolerance: 5.0 }));
      // console.log(view.getCenter(), view.getZoom());
    };

    map.on("click", clickListener);

    return () => {
      map.setTarget(undefined);
      map.un("click", clickListener);
    };
  }, [existingMap, setMap, initialView]);

  useEffect(() => {
    if (!existingMap) {
      console.log("no map yet");
      return;
    }

    const map = existingMap;

    const layers = map.getLayers().getArray();

    for (const { route, style } of routes ?? []) {
      console.log(route.id);
      const existingLayer = layers
        .filter((x): x is VectorLayer<any> => x instanceof VectorLayer)
        .find((layer) => layer.getProperties().routeId === route.id);
      let layer: VectorLayer<any>;

      if (existingLayer === undefined) {
        const lineString = new LineString(route.points);

        layer = new VectorLayer({
          source: new VectorSource({
            features: [new Feature(lineString)],
          }),
          properties: { routeId: route.id },
        });

        map.addLayer(layer);
      } else {
        layer = existingLayer;
      }

      const color = style === "muted" ? "#808080" : "#a54331";

      layer.setStyle(
        new Style({
          stroke: new Stroke({ color, width: 4 }),
        })
      );

      if (initialView?.type === "route" && initialView.routeId === route.id) {
        map.getView().fit(new LineString(route.points), {
          padding: [100, 100, 100, 100],
          duration: 0,
        });
      }
    }

    for (const checkpoint of checkpoints ?? []) {
      console.log(checkpoint.name);
      const existingLayer = layers.find(
        (layer) => layer.getProperties().checkpointName === checkpoint.name
      );

      if (existingLayer === undefined) {
        map.addLayer(
          new VectorLayer({
            source: new VectorSource({
              features: [new Feature(new Point(checkpoint.point))],
            }),
            properties: { checkpointName: checkpoint.name },
            style:
              checkpoint.pointOfInterestType === PointOfInterestType.Hut
                ? hutStyle
                : stationStyle,
          })
        );
      }
    }
  }, [routes, checkpoints, existingMap, initialView, hutStyle, stationStyle]);

  return <div css={mapCss} id="map" />;
}
