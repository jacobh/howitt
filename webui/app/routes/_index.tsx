import { Map } from "../components/map";
import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";
import { useState } from "react";
import { Container, MapContainer, SidebarContainer } from "~/components/layout";
import { RouteItem } from "~/components/RouteItem";
import { css } from "@emotion/react";
import { COLORS } from "~/styles/theme";

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

  const { data } = useQuery(HOME_QUERY);

  return (
    <Container>
      <SidebarContainer title="Routes">
        {data?.starredRoutes.map((route) => (
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
        />
      </MapContainer>
    </Container>
  );
}
