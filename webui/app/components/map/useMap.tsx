import { useContext, useEffect, useRef, useState } from "react";
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
import { MapProps } from ".";
import { MapContext } from "./context";

type UseMapProps = Pick<
  MapProps,
  "initialView" | "onVisibleRoutesChanged" | "onRouteClicked"
>;

export const DEFAULT_VIEW: ViewOptions = {
  center: [146, -37],
  zoom: 7.5,
  enableRotation: false,
};

export function useMap({
  initialView,
  onRouteClicked,
  onVisibleRoutesChanged,
}: UseMapProps): { map: OlMap | undefined } {
  const { map: existingMap, setMap } = useContext(MapContext);
  const [isFirstMapRender, setIsFirstRender] = useState(true);
  const mapRef = useRef<OlMap>(undefined);

  useEffect(() => {
    let map: OlMap;
    if (existingMap instanceof OlMap) {
      console.log("map already rendered");

      mapRef.current = existingMap;
      map = existingMap;

      existingMap.setTarget("map");
    } else {
      console.log("initial map render");

      // eslint-disable-next-line react-hooks/rules-of-hooks
      useGeographic();

      const newMap = new OlMap({
        target: "map",
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
      });

      setMap(newMap);
      mapRef.current = newMap;
      map = newMap;
    }

    if (isFirstMapRender && initialView?.type === "view") {
      map.setView(new View({ ...initialView.view, enableRotation: false }));
    }
    if (existingMap === undefined) {
      map.setView(new View(DEFAULT_VIEW));
    }

    setIsFirstRender(false);

    const clickListener = (event: MapBrowserEvent<any>): void => {
      const clickedFeatures = map.getFeaturesAtPixel(event.pixel, {
        hitTolerance: 20.0,
      });
      const feature = clickedFeatures[0];

      if (!isNotNil(feature)) {
        if (isNotNil(onRouteClicked)) {
          onRouteClicked(undefined);
        }
        return;
      }

      const { routeId } = feature.getProperties();

      if (isNotNil(onRouteClicked)) {
        onRouteClicked(routeId);
      }
    };

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

    map.on("click", clickListener);
    map.getView().on("change:center", onViewChange);

    return () => {
      map.setTarget(undefined);
      map.un("click", clickListener);
      map.getView().un("change:center", onViewChange);
    };
  }, [
    existingMap,
    setMap,
    initialView,
    onVisibleRoutesChanged,
    isFirstMapRender,
    setIsFirstRender,
    onRouteClicked,
  ]);

  return { map: mapRef.current };
}
