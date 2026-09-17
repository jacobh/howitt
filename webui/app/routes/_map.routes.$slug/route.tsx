import { useQuery } from "@apollo/client/react";
import { Link, useParams } from "react-router";
import { gql } from "~/__generated__";
import { BikeSpecContent } from "./BikeSpec";
import { ElevationProfile } from "~/components/ElevationProfile";
import { isNotNil } from "~/services/isNotNil";
import { NearbyRoutes } from "./NearbyRoutes";
import { SidebarContainer } from "~/components/layout";
import { RouteVitals } from "~/components/routes/RouteVitals";
import { makeMqs } from "~/styles/mediaQueries";
import { css } from "@emotion/react";
import { tokens } from "~/styles/tokens";
import { DataTable } from "~/components/DataTable";
import { capitalize } from "es-toolkit";
import { usePrimaryMapContent } from "~/components/map/hooks/usePrimaryMapContent";
import { buildRouteTrack, Marker } from "~/components/map/types";
import { useMemo, useState } from "react";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";

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
    ...routeVitals
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
  const [hoveredPointIndex, setHoveredPointIndex] = useState<
    number | undefined
  >();

  const { data, loading } = useQuery(RouteQuery, {
    variables: { slug: params.slug ?? "" },
  });

  const route = data?.routeWithSlug;

  const nearbyRoutes = (route?.termini ?? []).flatMap((t) =>
    t.nearbyRoutes.filter(
      (nearby) => nearby.closestTerminus.route.id !== route?.id,
    ),
  );

  const tracks = useMemo(
    () =>
      [
        route ? buildRouteTrack(route) : undefined,
        ...(route?.termini ?? []).flatMap((terminus) =>
          terminus.nearbyRoutes
            .filter((nearby) => nearby.closestTerminus.route.id !== route?.id)
            .map((nearby) =>
              buildRouteTrack(nearby.closestTerminus.route, "muted"),
            ),
        ),
      ].filter((track) => isNotNil(track)),
    [route],
  );

  const initialView = useMemo(
    () =>
      route ? { type: "tracks" as const, trackIds: [route.id] } : undefined,
    [route],
  );

  const markers = useMemo((): Marker[] => {
    if (!route || hoveredPointIndex === undefined) {
      return [];
    }

    const point = tracks.find(({ id }) => id === route.id)?.points[
      hoveredPointIndex
    ];
    return point
      ? [{ id: "elevation-profile", point, style: "elevation" }]
      : [];
  }, [route, hoveredPointIndex, tracks]);

  const tableItems = [
    { name: "Technical Difficulty", value: route?.technicalDifficulty },
    { name: "Physical Difficulty", value: route?.physicalDifficulty },
    { name: "Scouted", value: route?.scouted },
    { name: "Direction", value: route?.direction },
  ]
    .map(({ name, value }) => (isNotNil(value) ? { name, value } : undefined))
    .filter((item) => isNotNil(item))
    .map(({ name, value }) => ({ name, value: capitalize(value) }));

  usePrimaryMapContent({ tracks, markers, initialView });

  return (
    <>
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
                  <p css={{ color: tokens.colors.grey700 }}>
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
              {route.description && (
                <section css={contentSectionCss}>
                  <p>{route.description}</p>
                </section>
              )}
              <section css={contentSectionCss}>
                <ElevationProfile
                  data={route}
                  onPointHover={setHoveredPointIndex}
                />
              </section>

              {tableItems.length > 0 && (
                <section css={contentSectionCss}>
                  <DataTable title="Overview" items={tableItems} />
                </section>
              )}

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
          ) : loading ? (
            <LoadingSpinnerSidebarContent />
          ) : (
            <></>
          )}
          {nearbyRoutes.length > 0 && (
            <section css={contentSectionCss}>
              {route?.termini.map((terminus) => (
                <NearbyRoutes key={terminus.bearing} terminus={terminus} />
              ))}
            </section>
          )}
        </div>
      </SidebarContainer>
    </>
  );
}
