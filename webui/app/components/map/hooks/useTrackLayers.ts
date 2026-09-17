import { useEffect } from "react";
import { Feature } from "ol";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import { LineString } from "ol/geom";
import { match, P } from "ts-pattern";
import OlMap from "ol/Map";
import { Track } from "../types";
import { ROUTE_STYLES, RIDE_STYLES } from "../index";
import { isNotNil } from "~/services/isNotNil";

interface UseTrackLayersProps {
  map: OlMap | undefined;
  tracks: Track[];
}

export function useTrackLayers({ map, tracks }: UseTrackLayersProps): void {
  useEffect(() => {
    return (): void => {
      // Removing layers mutates the collection, so iterate over a snapshot.
      const layers = [...(map?.getLayers().getArray() ?? [])];
      for (const layer of layers) {
        if (isNotNil(layer.get("trackId"))) map?.removeLayer(layer);
      }
    };
  }, [map]);

  useEffect(() => {
    if (!map) {
      return;
    }

    const layers = map.getLayers().getArray();
    const previousLayers = [...layers];

    // cleanup any tracks that have been dropped
    for (const layer of previousLayers) {
      if (layer instanceof VectorLayer) {
        const vectorLayer = layer as VectorLayer;
        const layerTrackId = vectorLayer.getProperties().trackId;

        if (isNotNil(layerTrackId)) {
          const isLayerTrackInCurrentRender = tracks.some(
            (track) => track.id === layerTrackId,
          );

          if (!isLayerTrackInCurrentRender) {
            map.removeLayer(layer);
          }
        }
      }
    }

    // Add or update track layers
    for (const track of tracks) {
      const existingLayer = layers
        .filter((x): x is VectorLayer => x instanceof VectorLayer)
        .find((layer) => layer.getProperties().trackId === track.id);

      let layer: VectorLayer;

      if (existingLayer === undefined) {
        const lineString = new LineString(track.points);

        layer = new VectorLayer({
          source: new VectorSource({
            features: [
              new Feature({ geometry: lineString, trackId: track.id }),
            ],
          }),
          properties: { trackId: track.id, points: track.points.length },
        });

        map.addLayer(layer);
      } else {
        layer = existingLayer;
      }

      if (layer.getProperties().points !== track.points.length) {
        const newLineString = new LineString(track.points);

        layer.setSource(
          new VectorSource({
            features: [
              new Feature({ geometry: newLineString, trackId: track.id }),
            ],
          }),
        );

        layer.setProperties({
          trackId: track.id,
          points: track.points.length,
        });
      }

      const styles = track.kind === "ride" ? RIDE_STYLES : ROUTE_STYLES;
      layer.setStyle(
        match(track.style)
          .with("muted", () => styles.muted)
          .with("highlighted", () => styles.highlighted)
          .with(P.union("default", undefined), () => styles.default)
          .exhaustive(),
      );
    }
  }, [map, tracks]);
}
