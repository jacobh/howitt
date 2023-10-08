import { useQuery } from "@apollo/client";
import Grid2 from "@mui/material/Unstable_Grid2";
import { Link, useParams } from "@remix-run/react";
import styled from "styled-components";
import { formatDistance } from "~/services/formatDistance";
import { gql } from "~/__generated__";
import { Map } from "../../components/map";
import { BikeSpecContent } from "./BikeSpec";
import { ElevationProfile } from "~/components/ElevationProfile";
import { Photo } from "./Photo";
import { useMemo } from "react";
import { isNotNil } from "~/services/isNotNil";
import { NearbyRoutes } from "./NearbyRoutes";

const ROUTE_QUERY = gql(`
query RouteQuery($routeId: RouteId!) {
  route(id: $routeId) {
    id
    name
    externalRef {
      canonicalUrl
    }
    distance
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
      direction
      nearbyRoutes {
        delta {
          distance
          bearing
          elevationGain
        }
        terminus {
          direction
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

const SidebarContainer = styled.div`
  overflow-y: scroll;
  padding: 20px 50px;
`;

function Definition({
  term,
  definition,
}: {
  term: string;
  definition?: string | null;
}): React.ReactElement {
  return definition ? (
    <>
      <dt>{term}</dt>
      <dd>{definition}</dd>
    </>
  ) : (
    <></>
  );
}

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
          t.nearbyRoutes.map((nearby) => ({
            route: nearby.terminus.route,
            style: "muted" as const,
          }))
        ),
      ].filter(isNotNil),
    [data]
  );

  return (
    <Grid2 container spacing={2}>
      <Grid2 xs={4} style={{ height: "100vh", overflowY: "scroll" }}>
        <SidebarContainer>
          <Link to="/">Back</Link>
          {data?.route ? (
            <>
              <h2>{data.route.name}</h2>
              <hr />
              {formatDistance(data.route.distance)}
              {data.route.description ? <p>{data.route.description}</p> : null}
              <br />
              {data.route.externalRef ? (
                <p>
                  <a
                    target="_blank"
                    rel="noreferrer"
                    href={data.route.externalRef?.canonicalUrl}
                  >
                    {data.route.externalRef?.canonicalUrl}
                  </a>
                </p>
              ) : (
                <></>
              )}
              <h3>Info</h3>
              <dl>
                <Definition
                  term="Technical Difficulty"
                  definition={data.route.technicalDifficulty}
                />
                <Definition
                  term="Physical Difficulty"
                  definition={data.route.technicalDifficulty}
                />
                <Definition term="Scouted" definition={data.route.scouted} />
                <Definition
                  term="Direction"
                  definition={data.route.direction}
                />
              </dl>
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
          {data?.route?.elevationPoints && data?.route?.distancePoints ? (
            <ElevationProfile
              elevationPoints={data.route.elevationPoints}
              distancePoints={data.route.distancePoints}
            />
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
                    key={terminus.direction}
                    terminus={terminus}
                    nearbyRoutes={terminus.nearbyRoutes}
                  />
                )
              )}
            </div>
          ) : null}
        </SidebarContainer>
      </Grid2>
      <Grid2 xs={8}>
        <Map
          routes={routes}
          initialView={data?.route ? { routeId: data.route.id } : undefined}
        />
      </Grid2>
    </Grid2>
  );
}
