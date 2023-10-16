import { DEFAULT_VIEW, Map } from "../components/map";
import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";
import { useMemo, useState } from "react";
import { Container, MapContainer, SidebarContainer } from "~/components/layout";
import { RouteItem } from "~/components/RouteItem";
import { css } from "@emotion/react";
import { COLORS } from "~/styles/theme";
import { isNotNil } from "~/services/isNotNil";
import { sortBy } from "lodash";

const HOME_QUERY = gql(`
  query homeQuery {
    starredRoutes {
      id
      name
      distance
      elevationAscentM
      elevationDescentM
      points
    }
  }
`);

const routeItemContainerCss = css`
  padding: 20px 0;
  border-bottom: 1px solid ${COLORS.offWhite};
`;

const routeTitleCss = css`
  font-size: 1.25rem; /* 20px */
  line-height: 1.75rem; /* 28px */
`;

export default function Index(): React.ReactElement {
  const [mode] = useState("routes");

  const [visibleRouteIds, setVisibleRouteIds] = useState<
    { routeId: string; distanceFromCenter: number }[] | undefined
  >(undefined);

  const { data } = useQuery(HOME_QUERY);

  const routeIdMap: Record<
    string,
    Exclude<typeof data, undefined>["starredRoutes"][number]
  > = useMemo(
    () =>
      Object.fromEntries(
        (data?.starredRoutes ?? []).map((route) => [route.id, route])
      ),
    [data]
  );

  const sidebarRoutes = isNotNil(visibleRouteIds)
    ? sortBy(visibleRouteIds, ({ distanceFromCenter }) => distanceFromCenter)
        .map(({ routeId }) => routeIdMap[routeId])
        .filter(isNotNil)
    : Object.values(routeIdMap);

  return (
    <Container>
      <SidebarContainer title="Routes">
        {sidebarRoutes.map((route) => (
          <div key={route.id} css={routeItemContainerCss}>
            <RouteItem route={route} routeTitleCss={routeTitleCss} />
          </div>
        ))}
      </SidebarContainer>
      <MapContainer>
        <Map
          routes={
            mode === "routes"
              ? data?.starredRoutes.map((route) => ({ route }))
              : undefined
          }
          initialView={{
            type: "view",
            view: DEFAULT_VIEW,
          }}
          onVisibleRoutesChanged={setVisibleRouteIds}
        />
      </MapContainer>
    </Container>
  );
}
