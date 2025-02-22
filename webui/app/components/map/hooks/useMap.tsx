import { useEffect, useState } from "react";
import { defaults as defaultInteractions } from "ol/interaction/defaults";
import OlMap from "ol/Map";
import { ViewOptions } from "ol/View";
import TileLayer from "ol/layer/Tile";
import XYZ from "ol/source/XYZ";
import { useGeographic } from "ol/proj";
import { MapProps } from "..";

type UseMapProps = Pick<
  MapProps,
  "mapInstance" | "onNewMapInstance" | "interactive"
> & { mapElementRef: React.RefObject<HTMLElement | null> };

export const DEFAULT_VIEW: ViewOptions = {
  center: [146, -37],
  zoom: 7.5,
  enableRotation: false,
};

export function useMap({
  mapInstance: existingMapInstance,
  onNewMapInstance,
  mapElementRef,
  interactive = true,
}: UseMapProps): { map: OlMap | undefined } {
  const [map, setMap] = useState<OlMap | undefined>(undefined);

  useEffect(() => {
    if (existingMapInstance) {
      setMap(existingMapInstance);
      existingMapInstance.setTarget(mapElementRef.current ?? undefined);
      return;
    }

    if (map) {
      map.setTarget(mapElementRef.current ?? undefined);
      return;
    }

    // eslint-disable-next-line react-hooks/rules-of-hooks
    useGeographic();

    const newMap = new OlMap({
      target: mapElementRef.current ?? undefined,
      layers: [
        new TileLayer({
          preload: Infinity,
          source: new XYZ({
            urls: [
              "https://d2o31mmlexa59r.cloudfront.net/landscape/{z}/{x}/{y}.png?apikey=f1165310fdfb499d9793b076ed26c08e",
            ],
          }),
        }),
      ],
      controls: [],
      interactions: [],
    });

    setMap(newMap);
    onNewMapInstance?.(newMap);
  }, [existingMapInstance, mapElementRef, map, onNewMapInstance]);

  useEffect(() => {
    if (!map) return;

    // always reset the map to zero
    for (const interaction of map.getInteractions().getArray()) {
      map.removeInteraction(interaction);
    }

    // then if interactive re-add controls
    if (interactive) {
      for (const interaction of defaultInteractions().getArray()) {
        map.addInteraction(interaction);
      }
    }
  }, [map, interactive]);

  return { map };
}
