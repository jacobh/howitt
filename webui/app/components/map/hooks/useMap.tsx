import { useEffect, useState } from "react";
import { defaults as defaultInteractions } from "ol/interaction/defaults";
import OlMap from "ol/Map";
import TileLayer from "ol/layer/Tile";
import XYZ from "ol/source/XYZ";
import { useGeographic } from "ol/proj";

interface UseMapProps {
  mapElementRef: React.RefObject<HTMLElement | null>;
  interactive?: boolean;
}

export function useMap({ mapElementRef, interactive = true }: UseMapProps): {
  map: OlMap | undefined;
} {
  const [map, setMap] = useState<OlMap>();

  useEffect(() => {
    // oxlint-disable-next-line react/rules-of-hooks
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

    return (): void => {
      newMap.dispose();
    };
  }, [mapElementRef]);

  useEffect(() => {
    if (!map) return;

    map.getInteractions().clear();

    if (interactive) {
      for (const interaction of defaultInteractions().getArray()) {
        map.addInteraction(interaction);
      }
    }
  }, [map, interactive]);

  return { map };
}
