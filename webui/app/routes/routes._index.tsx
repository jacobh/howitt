import { DEFAULT_INITIAL_VIEW, DEFAULT_VIEW } from "../components/map";
import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { gql } from "../__generated__/gql";
import { useMemo, useState } from "react";
import {
  Container,
  MapContainer,
  SidebarContainer,
  Nav,
} from "~/components/layout";
import { RouteItem } from "~/components/routes/RouteItem";
import { css } from "@emotion/react";
import { tokens } from "~/styles/tokens";
import { isNotNil } from "~/services/isNotNil";
import { sortBy } from "lodash";
import { useSearchParams } from "@remix-run/react";
import { PrimaryMap } from "~/components/map/PrimaryMap";
import { buildRouteTrack } from "~/components/map/types";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";

const HomeQueryNoPoints = gql(`
  query homeQuery($input: QueryRoutesInput!) {
    queryRoutes(input: $input) {
      id
      samplePoints
      ...routeItem
    }
    viewer {
      ...viewerInfo
    }
  }
`);

const HomeQueryWithPoints = gql(`
  query homeQueryPointOnly($input: QueryRoutesInput!) {
    queryRoutes(input: $input) {
      id
      pointsJson
    }
  }
`);

const routeItemContainerCss = css`
  padding: 20px 1.5%;
  border-bottom: 1px solid ${tokens.colors.offWhite};

  &:hover {
    background-color: ${tokens.colors.offWhite};
  }
`;

const clickedRouteItemContainerCss = css(
  routeItemContainerCss,
  css`
    background-color: ${tokens.colors.offWhite};
  `,
);

const routeTitleCss = css`
  font-size: 1.25rem; /* 20px */
  line-height: 1.75rem; /* 28px */
`;

function extractTags(params: URLSearchParams): string[] | undefined {
  const tags = params.get("tags");

  if (isNotNil(tags)) {
    return tags.split(",");
  }

  return undefined;
}

export default function Routes(): React.ReactElement {
  const [searchParams] = useSearchParams();

  const tags = extractTags(searchParams);

  const filters = isNotNil(tags) ? [{ hasSomeTags: tags }] : [];

  const { data, loading } = useQuery(HomeQueryNoPoints, {
    variables: {
      input: { filters },
    },
  });

  const { data: data2 } = useQuery(HomeQueryWithPoints, {
    variables: {
      input: { filters },
    },
    ssr: false,
  });

  const [clickedRouteId, setClickedRouteId] = useState<string | undefined>(
    undefined,
  );

  const [hoveredRouteId, setHoveredRouteId] = useState<string | undefined>(
    undefined,
  );

  const [visibleRouteIds, setVisibleRouteIds] = useState<
    { routeId: string; distanceFromCenter: number }[] | undefined
  >(undefined);

  const routeIdMap: Record<
    string,
    Exclude<typeof data, undefined>["queryRoutes"][number]
  > = useMemo(
    () =>
      Object.fromEntries(
        (data?.queryRoutes ?? []).map((route) => [route.id, route]),
      ),
    [data],
  );

  const sidebarRoutes = useMemo(
    () =>
      isNotNil(visibleRouteIds)
        ? sortBy(
            visibleRouteIds,
            ({ distanceFromCenter }) => distanceFromCenter,
          )
            .filter(({ routeId }) => routeId !== clickedRouteId)
            .map(({ routeId }) => routeIdMap[routeId])
            .filter(isNotNil)
        : Object.values(routeIdMap),
    [clickedRouteId, routeIdMap, visibleRouteIds],
  );

  const mapRoutes = useMemo(
    () =>
      (data2?.queryRoutes ?? data?.queryRoutes ?? []).map((route) =>
        buildRouteTrack(
          {
            id: route.id,
            pointsJson:
              (route as any).pointsJson ??
              JSON.stringify((route as any).samplePoints),
          },
          hoveredRouteId === route.id || clickedRouteId === route.id
            ? "highlighted"
            : undefined,
        ),
      ),
    [clickedRouteId, data?.queryRoutes, data2?.queryRoutes, hoveredRouteId],
  );

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer
        titleSegments={
          isNotNil(tags)
            ? [
                { name: "Routes", linkTo: "/routes" },
                {
                  name: tags.map((tag) => `#${tag}`).join(" "),
                  linkTo: `/routes?tags=${tags.join(",")}`,
                },
              ]
            : [{ name: "Routes", linkTo: "/routes" }]
        }
      >
        {loading ? <LoadingSpinnerSidebarContent /> : <></>}
        {clickedRouteId ? (
          <div
            css={clickedRouteItemContainerCss}
            onMouseEnter={(): void => setHoveredRouteId(clickedRouteId)}
            onMouseLeave={(): void => setHoveredRouteId(undefined)}
          >
            <RouteItem
              route={routeIdMap[clickedRouteId]}
              routeTitleCss={routeTitleCss}
            />
          </div>
        ) : null}
        {sidebarRoutes.map((route) => (
          <div
            key={route.id}
            css={routeItemContainerCss}
            onMouseEnter={(): void => setHoveredRouteId(route.id)}
            onMouseLeave={(): void => setHoveredRouteId(undefined)}
          >
            <RouteItem route={route} routeTitleCss={routeTitleCss} />
          </div>
        ))}
      </SidebarContainer>
      <MapContainer>
        <PrimaryMap
          tracks={mapRoutes}
          initialView={DEFAULT_INITIAL_VIEW}
          onVisibleRoutesChanged={setVisibleRouteIds}
          onRouteClicked={setClickedRouteId}
        />
      </MapContainer>
    </Container>
  );
}
