import { useCallback, useEffect } from "react";
import OlMap from "ol/Map";
import { MapBrowserEvent } from "ol";
import { Coordinate } from "ol/coordinate";

export interface MapEventCoords {
  lon: number;
  lat: number;
}

export interface MapEvent {
  type: "click";
  coords: MapEventCoords;
}

interface UseMapEventsProps {
  map: OlMap | undefined;
  onEvent?: (event: MapEvent) => void;
}

export function useMapEvents({ map, onEvent }: UseMapEventsProps): void {
  const handleClick = useCallback(
    (event: MapBrowserEvent<MouseEvent>): void => {
      const clickCoordinate: Coordinate = event.map.getCoordinateFromPixel(
        event.pixel,
      );

      const [lon, lat] = clickCoordinate;

      onEvent?.({
        type: "click",
        coords: {
          lon,
          lat,
        },
      });
    },
    [onEvent],
  );

  useEffect(() => {
    if (!map) {
      return;
    }

    map.on("click", handleClick);

    // Cleanup
    return () => {
      map.un("click", handleClick);
    };
  }, [map, handleClick]);
}
