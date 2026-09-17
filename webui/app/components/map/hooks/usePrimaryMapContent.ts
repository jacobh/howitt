import { useContext } from "react";
import type { MapProps } from "~/components/map";
import { PrimaryMapContext } from "~/components/map/context";
import { useInitialView } from "~/components/map/hooks/useInitialView";
import { useTrackLayers } from "~/components/map/hooks/useTrackLayers";
import { useMarkerLayers } from "~/components/map/hooks/useMarkerLayers";
import { useMapEvents } from "~/components/map/hooks/useMapEvents";

type PrimaryMapContentOptions = Pick<
  MapProps,
  "tracks" | "markers" | "initialView" | "onEvent"
>;

// Configure the layout's persistent map without owning its DOM or lifetime.
export function usePrimaryMapContent({
  tracks = [],
  markers = [],
  initialView,
  onEvent,
}: PrimaryMapContentOptions): void {
  const context = useContext(PrimaryMapContext);
  if (!context) throw new Error("PrimaryMap requires the map route layout");
  const { map } = context;

  useInitialView({ map, tracks, initialView });
  useTrackLayers({ map, tracks });
  useMarkerLayers({ map, markers });
  useMapEvents({ map, onEvent });
}
