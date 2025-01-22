import React, {
  createContext,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";
import OlMap from "ol/Map";
import { getDistance } from "ol/sphere";
import View, { ViewOptions } from "ol/View";
import TileLayer from "ol/layer/Tile";
import XYZ from "ol/source/XYZ";
import { useGeographic } from "ol/proj";
import {
  Route,
  Ride,
  PointOfInterest,
  PointOfInterestType,
} from "../../__generated__/graphql";
import { Feature, MapBrowserEvent } from "ol";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import { Style, Stroke, Circle } from "ol/style";
import { LineString, Point } from "ol/geom";
import Fill from "ol/style/Fill";
import { css } from "@emotion/react";
import BaseEvent from "ol/events/Event";
import { isNotNil } from "~/services/isNotNil";
import { debounce, isEqual, min, some } from "lodash";

export interface DisplayedRoute {
  route: Pick<Route, "id" | "pointsJson">;
  style?: "default" | "muted" | "highlighted";
}

interface MapProps {
  routes?: DisplayedRoute[];
  rides?: Pick<Ride, "id" | "pointsJson">[];
  checkpoints?: Pick<
    PointOfInterest,
    "name" | "point" | "pointOfInterestType"
  >[];
  initialView?:
    | { type: "ride"; rideId: string }
    | { type: "route"; routeId: string }
    | { type: "view"; view: ViewOptions };
  onVisibleRoutesChanged?: (
    routes: { routeId: string; distanceFromCenter: number }[]
  ) => void;

  onRouteClicked?: (routeId: string | undefined) => void;
}

interface MapContext {
  map?: OlMap | undefined;
  setMap: (map: OlMap) => void;
}

export const MapContext = createContext<MapContext>({ setMap: () => {} });

export const DEFAULT_VIEW: ViewOptions = {
  center: [146, -37],
  zoom: 7.5,
  enableRotation: false,
};

const mapCss = css`
  width: 100%;
  height: 100%;
`;

export function Map({
  routes,
  rides,
  checkpoints,
  initialView: initialViewProp,
  onVisibleRoutesChanged,
  onRouteClicked,
}: MapProps): React.ReactElement {
  const { map: existingMap, setMap } = useContext(MapContext);

  const [isFirstMapRender, setIsFirstRender] = useState(true);

  const [initialView, setInitialView] = useState(initialViewProp);

  useEffect(() => {
    if (!isEqual(initialView, initialViewProp)) {
      setInitialView(initialViewProp);
    }
  }, [initialView, initialViewProp, setInitialView]);

  const hutStyle = useMemo<Style>(
    () =>
      new Style({
        image: new Circle({
          fill: new Fill({
            color: "rgba(255,255,255,0.4)",
          }),
          stroke: new Stroke({
            color: "#5e8019",
            width: 1.25,
          }),
          radius: 5,
        }),
      }),
    []
  );

  const stationStyle = useMemo<Style>(
    () =>
      new Style({
        image: new Circle({
          fill: new Fill({
            color: "rgba(255,255,255,0.4)",
          }),
          stroke: new Stroke({
            color: "#4b6eaf",
            width: 1.25,
          }),
          radius: 5,
        }),
      }),
    []
  );

  useEffect(() => {
    let map: OlMap;

    if (existingMap instanceof OlMap) {
      console.log("map already rendered");

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
                feature.getGeometry()?.getClosestPoint(viewState.center)
              )
              .filter(isNotNil)
              .map((closestPoint) =>
                getDistance(closestPoint, viewState.center)
              )
          );

          return isNotNil(distanceFromCenter)
            ? { layer, distanceFromCenter }
            : undefined;
        })
        .filter(isNotNil)
        .flatMap(({ layer }) =>
          isNotNil(layer.getProperties().routeId)
            ? { routeId: layer.getProperties().routeId, distanceFromCenter: 0 }
            : undefined
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

  useEffect(() => {
    if (!existingMap) {
      console.log("no map yet");
      return;
    }

    const map = existingMap;

    const layers = map.getLayers().getArray();

    // cleanup any routes that have been dropped
    for (const layer of layers) {
      if (layer instanceof VectorLayer) {
        const vectorLayer = layer as VectorLayer<any>;

        const layerRouteId = vectorLayer.getProperties().routeId;

        if (isNotNil(layerRouteId)) {
          const isLayerRouteInCurrentRender = some(
            routes,
            ({ route }) => route.id === layerRouteId
          );

          if (!isLayerRouteInCurrentRender) {
            map.removeLayer(layer);
          }
        }
      }
    }

    for (const { route, style } of routes ?? []) {
      // console.log(route.id);
      const existingLayer = layers
        .filter((x): x is VectorLayer<any> => x instanceof VectorLayer)
        .find((layer) => layer.getProperties().routeId === route.id);
      let layer: VectorLayer<any>;

      if (existingLayer === undefined) {
        const lineString = new LineString(JSON.parse(route.pointsJson));

        layer = new VectorLayer({
          source: new VectorSource({
            features: [
              new Feature({ geometry: lineString, routeId: route.id }),
            ],
          }),
          properties: { routeId: route.id, points: route.pointsJson.length },
        });

        map.addLayer(layer);
      } else {
        layer = existingLayer;
      }

      if (layer.getProperties().points !== route.pointsJson.length) {
        const newLineString = new LineString(JSON.parse(route.pointsJson));

        layer.setSource(
          new VectorSource({
            features: [
              new Feature({ geometry: newLineString, routeId: route.id }),
            ],
          })
        );

        layer.setProperties({
          routeId: route.id,
          points: route.pointsJson.length,
        });
      }

      let color;

      switch (style) {
        case "muted":
          color = "#808080";
          break;
        case "highlighted":
          color = "#39abbf";
          break;
        default:
          color = "#a54331";
          break;
      }

      layer.setStyle(
        new Style({
          stroke: new Stroke({ color, width: 4 }),
        })
      );

      if (initialView?.type === "route" && initialView.routeId === route.id) {
        map.getView().fit(new LineString(JSON.parse(route.pointsJson)), {
          padding: [100, 100, 100, 100],
          duration: 0,
        });
      }
    }

    for (const ride of rides ?? []) {
      // console.log(route.id);
      const existingLayer = layers
        .filter((x): x is VectorLayer<any> => x instanceof VectorLayer)
        .find((layer) => layer.getProperties().rideId === ride.id);
      let layer: VectorLayer<any>;

      if (existingLayer === undefined) {
        const lineString = new LineString(JSON.parse(ride.pointsJson));

        layer = new VectorLayer({
          source: new VectorSource({
            features: [new Feature({ geometry: lineString, rideId: ride.id })],
          }),
          properties: { rideId: ride.id, points: ride.pointsJson.length },
        });

        console.log("adding layer");
        map.addLayer(layer);
      } else {
        layer = existingLayer;
      }

      if (layer.getProperties().points !== ride.pointsJson.length) {
        const newLineString = new LineString(JSON.parse(ride.pointsJson));

        layer.setSource(
          new VectorSource({
            features: [
              new Feature({ geometry: newLineString, rideId: ride.id }),
            ],
          })
        );

        layer.setProperties({
          rideId: ride.id,
          points: ride.pointsJson.length,
        });
      }

      const color = "#29892e";

      // let color;

      // switch (style) {
      //   case "muted":
      //     color = "#808080";
      //     break;
      //   case "highlighted":
      //     color = "#39abbf";
      //     break;
      //   default:
      //     color = "#a54331";
      //     break;
      // }

      layer.setStyle(
        new Style({
          stroke: new Stroke({ color, width: 4 }),
        })
      );

      if (initialView?.type === "ride" && initialView.rideId === ride.id) {
        map.getView().fit(new LineString(JSON.parse(ride.pointsJson)), {
          padding: [100, 100, 100, 100],
          duration: 0,
        });
      }
    }

    for (const checkpoint of checkpoints ?? []) {
      console.log(checkpoint.name);
      const existingLayer = layers.find(
        (layer) => layer.getProperties().checkpointName === checkpoint.name
      );

      if (existingLayer === undefined) {
        map.addLayer(
          new VectorLayer({
            source: new VectorSource({
              features: [new Feature(new Point(checkpoint.point))],
            }),
            properties: { checkpointName: checkpoint.name },
            style:
              checkpoint.pointOfInterestType === PointOfInterestType.Hut
                ? hutStyle
                : stationStyle,
          })
        );
      }
    }
  }, [
    routes,
    checkpoints,
    existingMap,
    initialView,
    hutStyle,
    stationStyle,
    rides,
  ]);

  return <div css={mapCss} id="map" />;
}

// function usePrevious<T>(value: T, initialValue: T): T {
//   const ref = useRef(initialValue);
//   useEffect(() => {
//     ref.current = value;
//   });
//   return ref.current;
// }

// function useEffectDebugger(
//   effect: React.EffectCallback,
//   dependencies: React.DependencyList,
//   dependencyNames: string[] = []
// ): void {
//   const previousDeps = usePrevious(dependencies, []);

//   const changedDeps = dependencies.reduce(
//     (
//       accum: Record<
//         string,
//         {
//           before: unknown;
//           after: unknown;
//         }
//       >,
//       dependency,
//       index
//     ) => {
//       if (dependency !== previousDeps[index]) {
//         const keyName = dependencyNames[index] || index;
//         return {
//           ...accum,
//           [keyName]: {
//             before: previousDeps[index],
//             after: dependency,
//           },
//         };
//       }

//       return accum;
//     },
//     {}
//   );

//   if (Object.keys(changedDeps).length) {
//     console.log("[use-effect-debugger] ", changedDeps);
//   }

//   useEffect(effect, dependencies);
// }
