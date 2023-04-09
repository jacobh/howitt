import { Map } from "../components/map";
import styled from "styled-components";
import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";
import Grid2 from "@mui/material/Unstable_Grid2";
import { useState } from "react";
import type { LinksFunction } from "@remix-run/node";

const StyledMain = styled.main`
  width: 100%;
  height: 100%;
  margin: 0;
  font-family: "Hanken Grotesk", sans-serif;
`;

const HOME_QUERY = gql(`
  query homeQuery {
    starredRoutes {
      id
      name
      distance
      points
    }
    checkpoints {
      id
      name
      point
      checkpointType
    }
  }
`);

const SidebarContainer = styled.div`
  overflow-y: scroll;
  padding: 20px 50px;
`;

export default function Index() {
  const [mode] = useState("routes");

  const { data } = useQuery(HOME_QUERY);

  return (
    <div>
      <StyledMain>
        <Grid2 container spacing={2}>
          <Grid2 xs={4} style={{ zIndex: 10, overflowY: "scroll" }}>
            <SidebarContainer>
              <h2>Routes</h2>
              <hr />
              {data?.starredRoutes.map((route) => (
                <div key={route.id}>
                  <h3>{route.name}</h3>
                  <p>{Math.round(route.distance / 100) / 10}km</p>
                </div>
              ))}
            </SidebarContainer>
          </Grid2>
          <Grid2 xs={8}>
            <Map
              routes={mode === "routes" ? data?.starredRoutes : undefined}
              checkpoints={data?.checkpoints}
            />
          </Grid2>
        </Grid2>
      </StyledMain>
    </div>
  );
}

export const links: LinksFunction = () => {
  return [
    { rel: "preconnect", href: "https://fonts.googleapis.com" },
    {
      rel: "preconnect",
      href: "https://fonts.gstatic.com",
      crossOrigin: "anonymous",
    },
    {
      rel: "stylesheet",
      href: "https://fonts.googleapis.com/css2?family=Hanken+Grotesk&display=swap",
    },
  ];
};
