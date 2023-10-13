import { Map } from "../components/map";
import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";
import { useState } from "react";
import { Link } from "@remix-run/react";
import { Container, MapContainer, SidebarContainer } from "~/components/layout";
import { css } from "@emotion/react";
import { COLORS } from "~/styles/theme";
import { RouteVitals } from "~/components/RouteVitals";

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

const routeItemCss = css(
  { padding: "20px 0", containerType: "inline-size" },
  css`
    border-bottom: 1px solid ${COLORS.offWhite};
  `
);
const routeTitleCss = css({ marginBottom: "6px", textDecoration: "underline" });

export default function Index(): React.ReactElement {
  const [mode] = useState("routes");

  const { data } = useQuery(HOME_QUERY);

  return (
    <Container>
      <SidebarContainer title="Routes">
        {data?.starredRoutes.map((route) => (
          <div key={route.id} css={routeItemCss}>
            <h3 css={routeTitleCss}>
              <Link to={`/routes/${route.id.split("#")[1]}`}>{route.name}</Link>
            </h3>
            <RouteVitals route={route} />
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
