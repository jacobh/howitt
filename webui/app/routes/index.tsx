import { Map } from "../components/map";
import styled from "styled-components";
import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";
import Grid2 from "@mui/material/Unstable_Grid2";
import { useState } from "react";
import { formatDistance } from "~/services/formatDistance";
import { Link } from "@remix-run/react";

const HOME_QUERY = gql(`
  query homeQuery {
    starredRoutes {
      id
      name
      distance
      points
    }
    pointsOfInterest {
      id
      name
      point
      pointOfInterestType
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
    <Grid2 container spacing={2}>
      <Grid2 xs={4} style={{ overflowY: "scroll" }}>
        <SidebarContainer>
          <h2>Routes</h2>
          <hr />
          {data?.starredRoutes.map((route) => (
            <div key={route.id}>
              <h3>
                <Link to={`/routes/${route.id.split("#")[1]}`}>
                  {route.name}
                </Link>
              </h3>
              <p>{formatDistance(route.distance)}</p>
            </div>
          ))}
        </SidebarContainer>
      </Grid2>
      <Grid2 xs={8}>
        <Map
          routes={mode === "routes" ? data?.starredRoutes : undefined}
          checkpoints={data?.pointsOfInterest}
        />
      </Grid2>
    </Grid2>
  );
}
