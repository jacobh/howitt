import OlMap from "ol/Map";
import { createContext } from "react";

export interface PrimaryMapContext {
  map?: OlMap | undefined;
  setMap: (map: OlMap) => void;
}

export const PrimaryMapContext = createContext<PrimaryMapContext>({
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  setMap: () => {},
});
