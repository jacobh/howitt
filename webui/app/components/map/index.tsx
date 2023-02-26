import React, { useEffect, useMemo, useState } from "react";
import OlMap from "ol/Map";
import View from "ol/View";
import TileLayer from "ol/layer/Tile";
import XYZ from "ol/source/XYZ";
import styled from "styled-components";
import { useGeographic } from "ol/proj";
import {
  Checkpoint,
  CheckpointType,
  Route,
  Ride,
} from "../../__generated__/graphql";
import { Feature, MapBrowserEvent } from "ol";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import { Style, Stroke, Circle } from "ol/style";
import { LineString, Point } from "ol/geom";
import Fill from "ol/style/Fill";

const MapContainer = styled.div`
  width: 100%;
  height: 100%;
  position: fixed;
`;

interface MapProps {
  routes?: Pick<Route, "id" | "points">[];
  rides?: Pick<Ride, "id" | "points">[];
  checkpoints?: Pick<Checkpoint, "name" | "point" | "checkpointType">[];
}

export function Map({ routes, rides, checkpoints }: MapProps) {
  const [map, setMap] = useState<OlMap>();
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
    console.log("initial map render");
    // eslint-disable-next-line react-hooks/rules-of-hooks
    useGeographic();

    const view = new View({
      center: [147.19193300372723, -37.416399197237276],
      zoom: 7.6,
    });

    const map = new OlMap({
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
      view,
    });

    setMap(map);

    map.addEventListener("click", (baseEvt) => {
      const evt = baseEvt as MapBrowserEvent<any>;
      console.log(evt.coordinate);
      console.log(map.getFeaturesAtPixel(evt.pixel, { hitTolerance: 5.0 }));
      // console.log(view.getCenter(), view.getZoom());
    });

    return () => map.setTarget(undefined);
  }, []);

  useEffect(() => {
    if (!map) {
      console.log("no map yet");
      return;
    }

    const layers = map.getLayers().getArray();

    for (const route of routes ?? []) {
      console.log(route.id);
      const existingLayer = layers.find(
        (layer) => layer.getProperties().routeId === route.id
      );

      if (existingLayer === undefined) {
        map.addLayer(
          new VectorLayer({
            source: new VectorSource({
              features: [new Feature(new LineString(route.points))],
            }),
            properties: { routeId: route.id },
            style: new Style({
              stroke: new Stroke({ color: "#a54331", width: 2 }),
            }),
          })
        );
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
              checkpoint.checkpointType === CheckpointType.Hut
                ? hutStyle
                : stationStyle,
          })
        );
      }
    }
  }, [routes, checkpoints, map]);

  return <MapContainer id="map" />;
}
