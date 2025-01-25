import OlMap from "ol/Map";
import { createContext } from "react";

export interface MapContext {
  map?: OlMap | undefined;
  setMap: (map: OlMap) => void;
}

export const MapContext = createContext<MapContext>({ setMap: () => {} });
