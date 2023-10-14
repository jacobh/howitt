import { useQuery } from "@apollo/client";
import { useParams } from "@remix-run/react";
import { gql } from "~/__generated__";
import { Map } from "../../components/map";
import { BikeSpecContent } from "./BikeSpec";
import { ElevationProfile } from "~/components/ElevationProfile";
import { Photo } from "./Photo";
import { useMemo } from "react";
import { isNotNil } from "~/services/isNotNil";
import { NearbyRoutes } from "./NearbyRoutes";
import { Container, MapContainer, SidebarContainer } from "~/components/layout";
import { RouteVitals } from "~/components/RouteVitals";
import { makeMqs } from "~/styles/mediaQueries";
import { css } from "@emotion/react";
import { COLORS } from "~/styles/theme";
import { DataTable } from "~/components/DataTable";

const ROUTE_QUERY = gql(`
query RouteQuery($routeId: RouteId!) {
  route(id: $routeId) {
    id
    name
    externalRef {
      canonicalUrl
    }
    distance
    elevationAscentM
    elevationDescentM
    points
    elevationPoints
    distancePoints
    description
    technicalDifficulty
    physicalDifficulty
    scouted
    direction
    minimumBike {
      tyreWidth
      frontSuspension
      rearSuspension
    }
    idealBike {
      tyreWidth
      frontSuspension
      rearSuspension
    }
    photos {
      id
      url
      caption
    }
    termini {
      bearing
      nearbyRoutes {
        delta {
          distance
          bearing
          elevationGain
        }
        closestTerminus {
          bearing
          route {
            id
            name
            points
          }
        }
      }
    }
  }
  viewer {
    role
  }
}
`);

const routeContentContainerCss = makeMqs([
  css`
    padding: 10px 0;
  `,
  css``,
  css`
    padding: 12px 0;
  `,
  css`
    padding: 14px 0;
  `,
  css`
    padding: 16px 0;
  `,
]);

export default function Route(): React.ReactElement {
  const params = useParams();

  const { data } = useQuery(ROUTE_QUERY, {
    variables: { routeId: ["ROUTE", params.routeId].join("#") },
  });

  const routes = useMemo(
    () =>
      [
        data?.route ? { route: data?.route } : undefined,
        ...(data?.route?.termini ?? []).flatMap((t) =>
          t.nearbyRoutes
            .filter(
              (nearby) => nearby.closestTerminus.route.id !== data?.route?.id
            )
            .map((nearby) => ({
              route: nearby.closestTerminus.route,
              style: "muted" as const,
            }))
        ),
      ].filter(isNotNil),
    [data]
  );

  const tableItems = [
    { name: "Technical Difficulty", value: data?.route?.technicalDifficulty },
    { name: "Physical Difficulty", value: data?.route?.physicalDifficulty },
    { name: "Scouted", value: data?.route?.scouted },
    { name: "Direction", value: data?.route?.direction },
  ]
    .map(({ name, value }) => (isNotNil(value) ? { name, value } : undefined))
    .filter(isNotNil);

  return (
    <Container>
      <SidebarContainer title={data?.route?.name ?? ""} showBack>
        <div css={routeContentContainerCss}>
          {data?.route ? (
            <>
              <RouteVitals route={data.route} />
              {data.route.externalRef ? (
                <p css={{ margin: "20px 0", color: COLORS.darkGrey }}>
                  <a
                    target="_blank"
                    rel="noreferrer"
                    href={data.route.externalRef?.canonicalUrl}
                  >
                    {data.route.externalRef?.canonicalUrl.split("://")[1]}
                  </a>
                </p>
              ) : (
                <></>
              )}
              {data.route.description ? (
                <p css={{ margin: "20px 0" }}>{data.route.description}</p>
              ) : null}
              {data?.route?.elevationPoints && data?.route?.distancePoints ? (
                <div css={{ margin: "20px 0" }}>
                  <ElevationProfile
                    elevationPoints={data.route.elevationPoints}
                    distancePoints={data.route.distancePoints}
                  />
                </div>
              ) : (
                <></>
              )}

              <DataTable title="Overview" items={tableItems} />
              {data.route.minimumBike ? (
                <BikeSpecContent
                  title="Minimum Bike"
                  bikeSpec={data.route.minimumBike}
                />
              ) : (
                <></>
              )}
              {data.route.idealBike ? (
                <BikeSpecContent
                  title="Ideal Bike"
                  bikeSpec={data.route.idealBike}
                />
              ) : (
                <></>
              )}
            </>
          ) : (
            <></>
          )}
          {data?.route?.photos.map((photo) => (
            <Photo key={photo.id} photo={photo} />
          ))}
          {data?.route ? (
            <div>
              {(data?.route?.termini ? data.route.termini : []).map(
                (terminus) => (
                  <NearbyRoutes
                    key={terminus.bearing}
                    terminus={terminus}
                    nearbyRoutes={terminus.nearbyRoutes}
                  />
                )
              )}
            </div>
          ) : null}
        </div>
      </SidebarContainer>
      <MapContainer>
        <Map
          routes={routes}
          initialView={data?.route ? { routeId: data.route.id } : undefined}
        />
      </MapContainer>
    </Container>
  );
}
