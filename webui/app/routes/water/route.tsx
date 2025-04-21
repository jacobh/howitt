import { DEFAULT_INITIAL_VIEW } from "../../components/map";
import { useQuery as useGqlQuery } from "@apollo/client/react/hooks/useQuery";
import { gql } from "../../__generated__/gql";
import {
  Container,
  MapContainer,
  SidebarContainer,
  Nav,
} from "~/components/layout";
import { PrimaryMap } from "~/components/map/PrimaryMap";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";
import { useQuery } from "@tanstack/react-query";
import { NearbyFeaturesResponse, WaterFeaturesResponse } from "./schema";
import { buildMarker } from "~/components/map/types";
import { match } from "ts-pattern";
import { PointOfInterestType } from "~/__generated__/graphql";
import { useCallback, useMemo, useState } from "react";
import { isNotNil } from "~/services/isNotNil";
import { MapEvent, MapEventCoords } from "~/components/map/hooks/useMapEvents";
import { DataTable } from "~/components/DataTable";

const ViewerQuery = gql(`
  query viewerQuery {
    viewer {
      ...viewerInfo
    }
  }
`);

export default function Water(): React.ReactElement {
  const { data: viewerData, loading: viewerLoading } = useGqlQuery(ViewerQuery);
  const [clickedPoint, setClickedPoint] = useState<MapEventCoords | undefined>(
    undefined,
  );

  const { data: indexData, isLoading: indexLoading } = useQuery({
    queryKey: ["waterFeatureIndex"],
    queryFn: async () => {
      const resp = await fetch(
        "https://ts-api.howittplains.net/api/water-features",
        // "http://localhost:3001/api/water-features",
      );
      const data = await resp.json();

      return data as WaterFeaturesResponse;
    },
  });

  const { data: nearbyData, isLoading: nearbyLoading } = useQuery({
    queryKey: ["waterFeatureQuery", clickedPoint],
    queryFn: async () => {
      if (!clickedPoint) {
        return { type: "FeatureCollection", features: [] };
      }

      const { lon, lat } = clickedPoint;

      const params = new URLSearchParams({
        origin: [lon, lat].join(","),
        radius: "200",
        // limit: "1",
      });

      const resp = await fetch(
        `https://ts-api.howittplains.net/api/water-features/query?${params.toString()}`,
        // `http://localhost:3001/api/water-features/query?${params.toString()}`,
        // "http://localhost:3001/api/water-features",
      );
      const data = await resp.json();

      return data as NearbyFeaturesResponse;
    },
  });

  const markers = useMemo(() => {
    const nearbyFeatureIds = new Set(
      nearbyData?.features.map(({ properties: { id } }) => id) ?? [],
    );

    const features = indexData?.features ?? [];

    return features
      .map((feature) =>
        match(feature)
          .with({ geometry: { type: "Point" } }, (feature) => {
            const { id, name } = feature.properties;
            const point = feature.geometry.coordinates;

            return buildMarker(
              {
                id: String(id),
                name,
                point,
                pointOfInterestType: PointOfInterestType.WaterSource,
              },
              nearbyFeatureIds.has(id) ? "highlighted" : "default",
            );
          })
          .otherwise(() => undefined),
      )
      .filter(isNotNil);
  }, [indexData, nearbyData]);

  const onMapEvent = useCallback(
    (evt: MapEvent) => {
      console.log(evt);
      setClickedPoint(evt.coords);
    },
    [setClickedPoint],
  );

  return (
    <Container>
      <Nav viewer={viewerData?.viewer} />
      <SidebarContainer titleSegments={[{ name: "Water", linkTo: "/water" }]}>
        {indexLoading || nearbyLoading || viewerLoading ? (
          <LoadingSpinnerSidebarContent />
        ) : (
          <div css={{ marginTop: "16px" }}>
            {clickedPoint === undefined ? (
              <h3>Click on the map to find beta</h3>
            ) : (
              <></>
            )}
            {clickedPoint !== undefined &&
            nearbyData &&
            nearbyData.features.length === 0 ? (
              <h3>
                No sources found for this location, click elsewhere to find some
                beta
              </h3>
            ) : (
              <></>
            )}
            {nearbyData?.features.map((feature) => (
              <>
                <h3>{feature.properties.name}</h3>
                {feature.properties.waterBeta.map((topic) => (
                  // eslint-disable-next-line react/jsx-key
                  <div css={{ margin: "16px 0" }}>
                    <p css={{ margin: "8px 0" }}>{topic.general_notes}</p>
                    {topic.observations.map((observation) => (
                      // eslint-disable-next-line react/jsx-key
                      <DataTable
                        items={[
                          { name: "Date", value: observation.date },
                          { name: "Season", value: observation.season },
                          { name: "Quality", value: observation.quality },
                          {
                            name: "Available",
                            value: observation.available,
                          },
                          {
                            name: "Notes",
                            value: observation.observation_notes,
                          },
                        ]}
                      />
                    ))}
                  </div>
                ))}
              </>
            ))}
          </div>
        )}
      </SidebarContainer>
      <MapContainer>
        <PrimaryMap
          markers={markers ?? []}
          onEvent={onMapEvent}
          initialView={DEFAULT_INITIAL_VIEW}
        />
      </MapContainer>
    </Container>
  );
}
