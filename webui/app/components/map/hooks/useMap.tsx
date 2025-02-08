import { useEffect, useState } from "react";
import { defaults as defaultInteractions } from "ol/interaction/defaults";
import OlMap from "ol/Map";
import { getDistance } from "ol/sphere";
import View, { ViewOptions } from "ol/View";
import TileLayer from "ol/layer/Tile";
import XYZ from "ol/source/XYZ";
import { useGeographic } from "ol/proj";
import { MapBrowserEvent } from "ol";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import BaseEvent from "ol/events/Event";
import { isNotNil } from "~/services/isNotNil";
import { debounce, min } from "lodash";
import { MapProps } from "..";

type UseMapProps = Pick<
  MapProps,
  | "mapInstance"
  | "onNewMapInstance"
  | "onVisibleRoutesChanged"
  | "onRouteClicked"
  | "interactive"
> & { mapElementRef: React.RefObject<HTMLElement | null> };

export const DEFAULT_VIEW: ViewOptions = {
  center: [146, -37],
  zoom: 7.5,
  enableRotation: false,
};

export function useMap({
  mapInstance: existingMapInstance,
  onNewMapInstance,
  onRouteClicked,
  onVisibleRoutesChanged,
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

    if (interactive) {
      for (const interaction of defaultInteractions().getArray()) {
        map.addInteraction(interaction);
      }
    } else {
      for (const interaction of map.getInteractions().getArray()) {
        map.removeInteraction(interaction);
      }
    }
  }, [map, interactive]);

  useEffect(() => {
    if (!map) return;

    const clickListener = (event: MapBrowserEvent<UIEvent>): void => {
      const feature = map.getFeaturesAtPixel(event.pixel, {
        hitTolerance: 20.0,
      })[0];

      onRouteClicked?.(feature?.getProperties().routeId);
    };

    map.on("click", clickListener);

    return () => map.un("click", clickListener);
  }, [onRouteClicked, map]);

  useEffect(() => {
    if (!map) return;

    const onViewChange = debounce((event: BaseEvent): void => {
      const view = event.target as View;
      const { extent, viewState } = view.getViewStateAndExtent();

      const visibleRoutes = map
        .getAllLayers()
        .filter((x): x is VectorLayer => x instanceof VectorLayer)
        .flatMap((layer) => {
          const source = layer.getSource() as VectorSource;
          const features = source.getFeaturesInExtent(extent);
          const distanceFromCenter = min(
            features
              .map((feature) =>
                feature.getGeometry()?.getClosestPoint(viewState.center),
              )
              .filter(isNotNil)
              .map((closestPoint) =>
                getDistance(closestPoint, viewState.center),
              ),
          );

          return isNotNil(distanceFromCenter)
            ? { layer, distanceFromCenter }
            : undefined;
        })
        .filter(isNotNil)
        .flatMap(({ layer }) =>
          isNotNil(layer.getProperties().routeId)
            ? { routeId: layer.getProperties().routeId, distanceFromCenter: 0 }
            : undefined,
        )
        .filter(isNotNil);

      if (onVisibleRoutesChanged) {
        onVisibleRoutesChanged(visibleRoutes);
      }
    }, 250);

    map.getView().on("change:center", onViewChange);

    return () => map.getView().un("change:center", onViewChange);
  }, [onVisibleRoutesChanged, map]);

  return { map };
}
