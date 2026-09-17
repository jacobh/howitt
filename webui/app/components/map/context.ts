import OlMap from "ol/Map";
import { createContext } from "react";

export interface PrimaryMapContext {
  map?: OlMap | undefined;
  openMapOverlay: () => void;
}

export const PrimaryMapContext = createContext<PrimaryMapContext | undefined>(
  undefined,
);
