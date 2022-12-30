import React, { useEffect, useRef, useState } from "react";
import OlMap from "ol/Map";
import View from "ol/View";
import TileLayer from "ol/layer/Tile";
import XYZ from "ol/source/XYZ";
import styled from "styled-components";
import { useGeographic } from "ol/proj";
import { Route } from "../../__generated__/graphql";
import { Collection, Feature, MapBrowserEvent, MapEvent, Overlay } from "ol";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import GeoJSON from "ol/format/GeoJSON";
import Style from "ol/style/Style";
import Stroke from "ol/style/Stroke";
import { Geometry, LineString, Point } from "ol/geom";

const MapContainer = styled.div`
  width: 100%;
  height: 100%;
  position: fixed;
`;

interface MapProps {
  routes?: Pick<Route, "id" | "points">[];
}

export function Map({ routes }: MapProps) {
  const [map, setMap] = useState<OlMap>();

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
          source: new XYZ({
            urls: [
              "https://a.tile.thunderforest.com/landscape/{z}/{x}/{y}.png?apikey=f1165310fdfb499d9793b076ed26c08e",
              "https://b.tile.thunderforest.com/landscape/{z}/{x}/{y}.png?apikey=f1165310fdfb499d9793b076ed26c08e",
              "https://c.tile.thunderforest.com/landscape/{z}/{x}/{y}.png?apikey=f1165310fdfb499d9793b076ed26c08e",
            ],
          }),
        }),
      ],
      view,
    });

    setMap(map);

    map.addEventListener("click", (baseEvt) => {
      const evt = baseEvt as MapBrowserEvent<any>
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
            zIndex: 100,
            source: new VectorSource({
              features: [new Feature(new LineString(route.points))],
            }),
            properties: { routeId: route.id },
            style: new Style({
              stroke: new Stroke({ color: "#FF0000", width: 2 }),
            }),
          })
        );
      }
    }
  }, [routes, map]);

  return <MapContainer id="map" />;
}
