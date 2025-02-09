import { useContext } from "react";
import { PrimaryMapContext } from "../context";
import { useUpdateMapView } from "./useUpdateMapView";
import View from "ol/View";

export function useUpdatePrimaryMapView(): {
  updateView: (fn: (view: View) => void) => void;
} {
  const { map } = useContext(PrimaryMapContext);
  const { updateView } = useUpdateMapView(map);

  return { updateView };
}
