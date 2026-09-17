import { useContext } from "react";
import { PrimaryMapContext } from "~/components/map/context";

export function useMapOverlay(): Pick<PrimaryMapContext, "openMapOverlay"> {
  const context = useContext(PrimaryMapContext);
  if (!context) throw new Error("Map overlay requires the map route layout");
  return { openMapOverlay: context.openMapOverlay };
}
