import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { Link, useParams } from "@remix-run/react";
import { gql } from "~/__generated__";
import { DisplayedRoute } from "../../components/map";
import { BikeSpecContent } from "./BikeSpec";
import { ElevationProfile } from "~/components/ElevationProfile";
import { isNotNil } from "~/services/isNotNil";
import { NearbyRoutes } from "./NearbyRoutes";
import {
  Container,
  MapContainer,
  Nav,
  SidebarContainer,
} from "~/components/layout";
import { RouteVitals } from "~/components/routes/RouteVitals";
import { makeMqs } from "~/styles/mediaQueries";
import { css } from "@emotion/react";
import { tokens } from "~/styles/tokens";
import { DataTable } from "~/components/DataTable";
import { capitalize } from "lodash";
import { PrimaryMap } from "~/components/map/PrimaryMap";

const RouteQuery = gql(`
query RouteQuery($slug: String!) {
  routeWithSlug(slug: $slug) {
    id
    name
    slug
    externalRef {
      canonicalUrl
    }
    tags
    distance
    elevationAscentM
    elevationDescentM
    pointsJson
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
    termini {
      bearing

      nearbyRoutes {
        closestTerminus {
          route {
            id
            pointsJson
          }
        }
      }

      ...nearbyRoutesInfo
    }

    ...elevationPath
  }
  viewer {
    ...viewerInfo
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

  const { data } = useQuery(RouteQuery, {
    variables: { slug: params.slug ?? "" },
  });

  const route = data?.routeWithSlug;

  const nearbyRoutes = (route?.termini ?? []).flatMap((t) =>
    t.nearbyRoutes.filter(
      (nearby) => nearby.closestTerminus.route.id !== route?.id,
    ),
  );

  const routes: DisplayedRoute[] = [
    route ? { route } : undefined,
    ...nearbyRoutes.map((nearby) => ({
      route: nearby.closestTerminus.route,
      style: "muted" as const,
    })),
  ].filter(isNotNil);

  const tableItems = [
    { name: "Technical Difficulty", value: route?.technicalDifficulty },
    { name: "Physical Difficulty", value: route?.physicalDifficulty },
    { name: "Scouted", value: route?.scouted },
    { name: "Direction", value: route?.direction },
  ]
    .map(({ name, value }) => (isNotNil(value) ? { name, value } : undefined))
    .filter(isNotNil)
    .map(({ name, value }) => ({ name, value: capitalize(value) }));

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer
        titleSegments={[
          { name: "Routes", linkTo: "/routes" },
          ...(route
            ? [
                {
                  name: route.name,
                  linkTo: `/routes/${route.slug}`,
                },
              ]
            : []),
        ]}
      >
        <div css={routeContentContainerCss}>
          {route ? (
            <>
              <section css={{ marginTop: "2px" }}>
                <RouteVitals route={route} />
              </section>
              {isNotNil(route.tags) ? (
                <section css={contentSectionCss}>
                  {route.tags.map((tag) => (
                    <Link to={`/?tags=${tag}`} key={tag} css={tagLinkCss}>
                      #{tag}
                    </Link>
                  ))}
                </section>
              ) : (
                <></>
              )}
              {route.externalRef ? (
                <section css={contentSectionCss}>
                  <p css={{ color: tokens.colors.darkGrey }}>
                    <a
                      target="_blank"
                      rel="noreferrer"
                      href={route.externalRef?.canonicalUrl}
                    >
                      {route.externalRef?.canonicalUrl.split("://")[1]}
                    </a>
                  </p>
                </section>
              ) : (
                <></>
              )}
              {route.description ? (
                <section css={contentSectionCss}>
                  <p>{route.description}</p>
                </section>
              ) : null}
              <section css={contentSectionCss}>
                <ElevationProfile data={route} />
              </section>

              {tableItems.length > 0 ? (
                <section css={contentSectionCss}>
                  <DataTable title="Overview" items={tableItems} />
                </section>
              ) : null}

              {route.minimumBike ? (
                <section css={contentSectionCss}>
                  <BikeSpecContent
                    title="Minimum Bike"
                    bikeSpec={route.minimumBike}
                  />
                </section>
              ) : (
                <></>
              )}
              {route.idealBike ? (
                <section css={contentSectionCss}>
                  <BikeSpecContent
                    title="Ideal Bike"
                    bikeSpec={route.idealBike}
                  />
                </section>
              ) : (
                <></>
              )}
            </>
          ) : (
            <></>
          )}
          {nearbyRoutes.length > 0 ? (
            <section css={contentSectionCss}>
              {route?.termini.map((terminus) => (
                <NearbyRoutes key={terminus.bearing} terminus={terminus} />
              ))}
            </section>
          ) : null}
        </div>
      </SidebarContainer>
      <MapContainer>
        <PrimaryMap
          routes={routes}
          initialView={
            route ? { type: "routes", routeIds: [route.id] } : undefined
          }
        />
      </MapContainer>
    </Container>
  );
}
