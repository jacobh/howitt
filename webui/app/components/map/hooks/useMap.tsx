import { useEffect, useRef } from "react";
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
  const mapRef = useRef<OlMap>(undefined);

  useEffect(() => {
    if (existingMapInstance) {
      mapRef.current = existingMapInstance;
      existingMapInstance.setTarget(mapElementRef.current ?? undefined);
      return;
    }

    if (mapRef.current) {
      mapRef.current.setTarget(mapElementRef.current ?? undefined);
      return;
    }

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
      ...(interactive ? {} : { interactions: [], controls: [] }),
    });

    mapRef.current = newMap;
    onNewMapInstance?.(newMap);
  }, [existingMapInstance, interactive, onNewMapInstance, mapElementRef]);

  useEffect(() => {
    const map = mapRef.current;
    if (!map) return;

    const clickListener = (event: MapBrowserEvent<any>): void => {
      const feature = map.getFeaturesAtPixel(event.pixel, {
        hitTolerance: 20.0,
      })[0];

      onRouteClicked?.(feature?.getProperties().routeId);
    };

    map.on("click", clickListener);

    return () => map.un("click", clickListener);
  }, [onRouteClicked]);

  useEffect(() => {
    const map = mapRef.current;
    if (!map) return;

    const onViewChange = debounce((event: BaseEvent): void => {
      const view = event.target as View;
      const { extent, viewState } = view.getViewStateAndExtent();

      const visibleRoutes = map
        .getAllLayers()
        .filter((x): x is VectorLayer<any> => x instanceof VectorLayer)
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
  }, [onVisibleRoutesChanged]);
  return { map: mapRef.current };
}
