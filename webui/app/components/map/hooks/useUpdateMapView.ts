import { useCallback } from "react";
import OlMap from "ol/Map";
import View from "ol/View";

export function useUpdateMapView(map: OlMap | undefined): {
  updateView: (fn: (view: View) => void) => void;
} {
  const updateView = useCallback(
    (fn: (view: View) => void): void => {
      if (!map) {
        return;
      }

      const view = map.getView();

      fn(view);
    },
    [map],
  );

  return { updateView };
}
