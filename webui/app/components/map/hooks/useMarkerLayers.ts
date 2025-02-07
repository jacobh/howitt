import { useEffect } from "react";
import { Feature } from "ol";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import { Point } from "ol/geom";
import OlMap from "ol/Map";
import { Style, Circle, Stroke, Fill } from "ol/style";
import { Marker } from "../types";
import { isNotNil } from "~/services/isNotNil";
import { some } from "lodash";

interface UseMarkerLayersProps {
  map: OlMap | undefined;
  markers: Marker[];
}

const MARKER_STYLES = {
  default: new Style({
    image: new Circle({
      fill: new Fill({ color: "rgba(255,255,255,0.4)" }),
      stroke: new Stroke({ color: "#5e8019", width: 1.5 }),
      radius: 8,
    }),
  }),
  muted: new Style({
    image: new Circle({
      fill: new Fill({ color: "rgba(255,255,255,0.4)" }),
      stroke: new Stroke({ color: "#808080", width: 1.5 }),
      radius: 8,
    }),
  }),
  highlighted: new Style({
    image: new Circle({
      fill: new Fill({ color: "rgba(57,171,191,0.3)" }),
      stroke: new Stroke({ color: "#39abbf", width: 2.5 }),
      radius: 16,
    }),
  }),
};

export function useMarkerLayers({ map, markers }: UseMarkerLayersProps) {
  useEffect(() => {
    if (!map) {
      return;
    }

    const layers = map.getLayers().getArray();

    // cleanup any markers that have been dropped
    for (const layer of layers) {
      if (layer instanceof VectorLayer) {
        const vectorLayer = layer as VectorLayer<any>;
        const layerMarkerId = vectorLayer.getProperties().markerId;

        if (isNotNil(layerMarkerId)) {
          const isLayerMarkerInCurrentRender = some(
            markers,
            (marker) => marker.id === layerMarkerId,
          );

          if (!isLayerMarkerInCurrentRender) {
            setInterval(() => {
              map.removeLayer(layer);
            }, 1);
          }
        }
      }
    }

    // Add or update marker layers
    for (const marker of markers) {
      const existingLayer = layers
        .filter((x): x is VectorLayer<any> => x instanceof VectorLayer)
        .find((layer) => layer.getProperties().markerId === marker.id);

      let layer: VectorLayer<any>;

      if (existingLayer === undefined) {
        const point = new Point(marker.point);

        layer = new VectorLayer({
          source: new VectorSource({
            features: [new Feature({ geometry: point, markerId: marker.id })],
          }),
          properties: {
            markerId: marker.id,
            point: marker.point,
            label: marker.label,
          },
        });

        map.addLayer(layer);
      } else {
        layer = existingLayer;
      }

      // Update the layer if the point or label has changed
      if (
        layer.getProperties().point !== marker.point ||
        layer.getProperties().label !== marker.label
      ) {
        const newPoint = new Point(marker.point);

        layer.setSource(
          new VectorSource({
            features: [
              new Feature({ geometry: newPoint, markerId: marker.id }),
            ],
          }),
        );

        layer.setProperties({
          markerId: marker.id,
          point: marker.point,
          label: marker.label,
        });
      }

      layer.setStyle(
        marker.style ? MARKER_STYLES[marker.style] : MARKER_STYLES.default,
      );
    }
  }, [map, markers]);
}
