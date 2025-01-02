import { useQuery } from "@apollo/client";
import { Link, useParams } from "@remix-run/react";
import { gql } from "~/__generated__";
import { DisplayedRoute, Map } from "../../components/map";
import { BikeSpecContent } from "./BikeSpec";
import { ElevationProfile } from "~/components/ElevationProfile";
import { Photo } from "./Photo";
import { isNotNil } from "~/services/isNotNil";
import { NearbyRoutes } from "./NearbyRoutes";
import {
  Container,
  MapContainer,
  Nav,
  SidebarContainer,
} from "~/components/layout";
import { RouteVitals } from "~/components/RouteVitals";
import { makeMqs } from "~/styles/mediaQueries";
import { css } from "@emotion/react";
import { COLORS } from "~/styles/theme";
import { DataTable } from "~/components/DataTable";
import { capitalize } from "lodash";

const ROUTE_QUERY = gql(`
query RouteQuery($routeId: RouteId!) {
  route(id: $routeId) {
    id
    name
    externalRef {
      canonicalUrl
    }
    tags
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
            distance
            elevationAscentM
            elevationDescentM
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

const contentSectionCss = css`
  margin: 24px 0;
`;

const tagLinkCss = css`
  margin-right: 8px;
`;

export default function Route(): React.ReactElement {
  const params = useParams();

  const { data } = useQuery(ROUTE_QUERY, {
    variables: { routeId: ["ROUTE", params.routeId].join("#") },
  });

  const nearbyRoutes = (data?.route?.termini ?? []).flatMap((t) =>
    t.nearbyRoutes.filter(
      (nearby) => nearby.closestTerminus.route.id !== data?.route?.id
    )
  );

  const routes: DisplayedRoute[] = [
    data?.route ? { route: data.route } : undefined,
    ...nearbyRoutes.map((nearby) => ({
      route: nearby.closestTerminus.route,
      style: "muted" as const,
    })),
  ].filter(isNotNil);

  const tableItems = [
    { name: "Technical Difficulty", value: data?.route?.technicalDifficulty },
    { name: "Physical Difficulty", value: data?.route?.physicalDifficulty },
    { name: "Scouted", value: data?.route?.scouted },
    { name: "Direction", value: data?.route?.direction },
  ]
    .map(({ name, value }) => (isNotNil(value) ? { name, value } : undefined))
    .filter(isNotNil)
    .map(({ name, value }) => ({ name, value: capitalize(value) }));

  return (
    <Container>
      <Nav />
      <SidebarContainer
        title="Routes"
        titleLinkTo="/"
        titlePostfix={["/", data?.route?.name ?? ""].join(" ")}
      >
        <div css={routeContentContainerCss}>
          {data?.route ? (
            <>
              <section css={{ marginTop: "2px" }}>
                <RouteVitals route={data.route} />
              </section>
              {isNotNil(data.route.tags) ? (
                <section css={contentSectionCss}>
                  {data.route.tags.map((tag) => (
                    <Link to={`/?tags=${tag}`} key={tag} css={tagLinkCss}>
                      #{tag}
                    </Link>
                  ))}
                </section>
              ) : (
                <></>
              )}
              {data.route.externalRef ? (
                <section css={contentSectionCss}>
                  <p css={{ color: COLORS.darkGrey }}>
                    <a
                      target="_blank"
                      rel="noreferrer"
                      href={data.route.externalRef?.canonicalUrl}
                    >
                      {data.route.externalRef?.canonicalUrl.split("://")[1]}
                    </a>
                  </p>
                </section>
              ) : (
                <></>
              )}
              {data.route.description ? (
                <section css={contentSectionCss}>
                  <p>{data.route.description}</p>
                </section>
              ) : null}
              {data?.route?.elevationPoints && data?.route?.distancePoints ? (
                <section css={contentSectionCss}>
                  <ElevationProfile
                    elevationPoints={data.route.elevationPoints}
                    distancePoints={data.route.distancePoints}
                  />
                </section>
              ) : (
                <></>
              )}

              {tableItems.length > 0 ? (
                <section css={contentSectionCss}>
                  <DataTable title="Overview" items={tableItems} />
                </section>
              ) : null}

              {data.route.minimumBike ? (
                <section css={contentSectionCss}>
                  <BikeSpecContent
                    title="Minimum Bike"
                    bikeSpec={data.route.minimumBike}
                  />
                </section>
              ) : (
                <></>
              )}
              {data.route.idealBike ? (
                <section css={contentSectionCss}>
                  <BikeSpecContent
                    title="Ideal Bike"
                    bikeSpec={data.route.idealBike}
                  />
                </section>
              ) : (
                <></>
              )}
            </>
          ) : (
            <></>
          )}
          {data?.route?.photos.map((photo) => (
            <section css={contentSectionCss} key={photo.id}>
              <Photo photo={photo} />
            </section>
          ))}
          {nearbyRoutes.length > 0 ? (
            <section css={contentSectionCss}>
              <NearbyRoutes nearbyRoutes={nearbyRoutes} />
            </section>
          ) : null}
        </div>
      </SidebarContainer>
      <MapContainer>
        <Map
          routes={routes}
          initialView={
            data?.route ? { type: "route", routeId: data.route.id } : undefined
          }
        />
      </MapContainer>
    </Container>
  );
}
