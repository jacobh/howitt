import React, { useContext } from "react";
import { Map, MapProps } from "./index";
import { PrimaryMapContext } from "./context";

export type PrimaryMapProps = Omit<
  MapProps,
  "mapInstance" | "onNewMapInstance"
>;

export function PrimaryMap(props: PrimaryMapProps): React.ReactElement {
  const { map, setMap } = useContext(PrimaryMapContext);

  return <Map {...props} mapInstance={map} onNewMapInstance={setMap} />;
}
