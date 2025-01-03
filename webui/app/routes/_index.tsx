import { DEFAULT_VIEW, Map } from "../components/map";
import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";
import { useMemo, useState } from "react";
import {
  Container,
  MapContainer,
  SidebarContainer,
  Nav,
} from "~/components/layout";
import { RouteItem } from "~/components/RouteItem";
import { css } from "@emotion/react";
import { COLORS } from "~/styles/theme";
import { isNotNil } from "~/services/isNotNil";
import { sortBy } from "lodash";
import { useSearchParams } from "@remix-run/react";

const HOME_QUERY = gql(`
  query homeQuery($input: QueryRoutesInput!) {
    queryRoutes(input: $input) {
      id
      name
      distance
      isMetaComplete
      elevationAscentM
      elevationDescentM
      points
    }
  }
`);

const routeItemContainerCss = css`
  padding: 20px 1.5%;
  border-bottom: 1px solid ${COLORS.offWhite};

  &:hover {
    background-color: ${COLORS.offWhite};
  }
`;

const clickedRouteItemContainerCss = css(
  routeItemContainerCss,
  css`
    background-color: ${COLORS.offWhite};
  `
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

export default function Index(): React.ReactElement {
  const [searchParams] = useSearchParams();

  const tags = extractTags(searchParams);

  const filters = isNotNil(tags) ? [{ hasSomeTags: tags }] : [];

  const { data } = useQuery(HOME_QUERY, {
    variables: {
      input: { filters },
    },
  });

  const [clickedRouteId, setClickedRouteId] = useState<string | undefined>(
    undefined
  );

  const [hoveredRouteId, setHoveredRouteId] = useState<string | undefined>(
    undefined
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
        (data?.queryRoutes ?? []).map((route) => [route.id, route])
      ),
    [data]
  );

  const sidebarRoutes = isNotNil(visibleRouteIds)
    ? sortBy(visibleRouteIds, ({ distanceFromCenter }) => distanceFromCenter)
        .filter(({ routeId }) => routeId !== clickedRouteId)
        .map(({ routeId }) => routeIdMap[routeId])
        .filter(isNotNil)
    : Object.values(routeIdMap);

  const mapRoutes = (data?.queryRoutes ?? []).map((route) => ({
    route,
    style:
      hoveredRouteId === route.id || clickedRouteId === route.id
        ? ("highlighted" as const)
        : undefined,
  }));

  return (
    <Container>
      <Nav />
      <SidebarContainer
        title="Routes"
        titleLinkTo={isNotNil(tags) ? "/" : undefined}
        titlePostfix={
          isNotNil(tags) ? tags.map((tag) => `#${tag}`).join(" ") : undefined
        }
      >
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
        <Map
          routes={mapRoutes}
          initialView={{
            type: "view",
            view: DEFAULT_VIEW,
          }}
          onVisibleRoutesChanged={setVisibleRouteIds}
          onRouteClicked={setClickedRouteId}
        />
      </MapContainer>
    </Container>
  );
}
