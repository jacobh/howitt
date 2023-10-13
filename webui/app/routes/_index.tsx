import { Map } from "../components/map";
import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";
import { useState } from "react";
import { formatDistance, formatVertical } from "~/services/format";
import { Link } from "@remix-run/react";
import { Container, MapContainer, SidebarContainer } from "~/components/layout";
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

const routeItemCss = css(
  { padding: "20px 0", containerType: "inline-size" },
  css`
    border-bottom: 1px solid ${COLORS.offWhite};
  `
);
const routeTitleCss = css({ marginBottom: "6px", textDecoration: "underline" });
const routeSubtitleCss = css`
  color: ${COLORS.midGrey};

  display: grid;
  grid-auto-flow: column;
  max-width: 320px;

  font-size: 0.875rem; /* 14px */
  line-height: 1.25rem; /* 20px */

  @container (max-width: 300px) {
    grid-auto-flow: row;
  }
`;
const routeSubtitleArrowCss = css`
  width: 30px;
  display: inline-block;
  text-align: center;
`;

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
            <p css={routeSubtitleCss}>
              <span>
                <span css={routeSubtitleArrowCss}>&rarr;</span>
                {formatDistance(route.distance)}
              </span>
              <span>
                <span css={routeSubtitleArrowCss}>&uarr;</span>
                {formatVertical(route.elevationAscentM)}
              </span>
              <span>
                <span css={routeSubtitleArrowCss}>&darr;</span>
                {formatVertical(route.elevationDescentM)}
              </span>
            </p>
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
