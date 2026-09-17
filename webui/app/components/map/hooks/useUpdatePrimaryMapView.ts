import { useContext } from "react";
import { PrimaryMapContext } from "../context";
import { useUpdateMapView } from "./useUpdateMapView";
import View from "ol/View";

export function useUpdatePrimaryMapView(): {
  updateView: (fn: (view: View) => void) => void;
} {
  const context = useContext(PrimaryMapContext);
  if (!context) throw new Error("PrimaryMap requires the map route layout");
  const { map } = context;
  const { updateView } = useUpdateMapView(map);

  return { updateView };
}
