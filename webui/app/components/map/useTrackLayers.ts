import { useEffect } from "react";
import { Feature } from "ol";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import { LineString } from "ol/geom";
import { match, P } from "ts-pattern";
import OlMap from "ol/Map";
import { Track } from "./index";
import { ROUTE_STYLES, RIDE_STYLES } from "./index";
import { isNotNil } from "~/services/isNotNil";
import { some } from "lodash";
import { Extent } from "ol/extent";
import { ViewOptions } from "ol/View";

interface UseTrackLayersProps {
  map: OlMap | undefined;
  tracks: Track[];
  initialView?:
    | { type: "rides"; rideIds: string[] }
    | { type: "routes"; routeIds: string[] }
    | { type: "view"; view: ViewOptions };
}

export function useTrackLayers({
  map,
  tracks,
  initialView,
}: UseTrackLayersProps) {
  useEffect(() => {
    if (!map) {
      return;
    }

    let initialBounds: Extent | undefined = undefined;
    const layers = map.getLayers().getArray();

    // cleanup any tracks that have been dropped
    for (const layer of layers) {
      if (layer instanceof VectorLayer) {
        const vectorLayer = layer as VectorLayer<any>;
        const layerTrackId = vectorLayer.getProperties().trackId;

        if (isNotNil(layerTrackId)) {
          const isLayerTrackInCurrentRender = some(
            tracks,
            (track) => track.id === layerTrackId,
          );

          if (!isLayerTrackInCurrentRender) {
            setInterval(() => {
              map.removeLayer(layer);
            }, 1);
          }
        }
      }
    }

    // Add or update track layers
    for (const track of tracks) {
      const existingLayer = layers
        .filter((x): x is VectorLayer<any> => x instanceof VectorLayer)
        .find((layer) => layer.getProperties().trackId === track.id);

      let layer: VectorLayer<any>;

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

      if (
        (initialView?.type === "routes" &&
          track.kind === "route" &&
          initialView.routeIds.includes(track.id)) ||
        (initialView?.type === "rides" &&
          track.kind === "ride" &&
          initialView.rideIds.includes(track.id))
      ) {
        const lineString = new LineString(track.points);
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

    if (initialBounds) {
      map.getView().fit(initialBounds, {
        padding: [100, 100, 100, 100],
        duration: 0,
      });
    }
  }, [map, tracks, initialView]);
}
