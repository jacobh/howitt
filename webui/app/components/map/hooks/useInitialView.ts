import { useEffect, useState } from "react";
import View, { ViewOptions } from "ol/View";
import OlMap from "ol/Map";
import { Track } from "../types";
import { LineString } from "ol/geom";
import { Extent } from "ol/extent";

export const DEFAULT_VIEW: ViewOptions = {
  center: [146, -37],
  zoom: 7.5,
  enableRotation: false,
};

interface UseInitialViewProps {
  map: OlMap | undefined;
  tracks: Track[];
  initialView?:
    | { type: "tracks"; trackIds: string[] }
    | { type: "view"; view: ViewOptions };
}

export function useInitialView({
  map,
  tracks,
  initialView,
}: UseInitialViewProps): { isInitialViewSet: boolean } {
  const [isInitialViewSet, setIsInitialViewSet] = useState(false);

  useEffect(() => {
    setIsInitialViewSet(false);
  }, [initialView]);

  useEffect(() => {
    if (!map || isInitialViewSet) {
      return;
    }

    if (initialView?.type === "view") {
      map.setView(new View({ ...initialView.view, enableRotation: false }));
      setIsInitialViewSet(true);
      return;
    }

    let initialBounds: Extent | undefined = undefined;

    // Calculate bounds for tracks
    for (const track of tracks) {
      if (
        initialView?.type === "tracks" &&
        initialView.trackIds.includes(track.id)
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
    } else {
      map.setView(new View(DEFAULT_VIEW));
    }

    setIsInitialViewSet(true);
  }, [map, initialView, isInitialViewSet, tracks]);

  return { isInitialViewSet };
}
