import { useQuery } from "@apollo/client";
import Grid2 from "@mui/material/Unstable_Grid2";
import { Link, useParams } from "@remix-run/react";
import styled from "styled-components";
import { formatDistance } from "~/services/formatDistance";
import { gql } from "~/__generated__";
import { Map } from "../../components/map";

const ROUTE_QUERY = gql(`
query RouteQuery($routeId: RouteId!) {
 route(id: $routeId) {
	id
name
distance
points
}
}
`);

const SidebarContainer = styled.div`
  overflow-y: scroll;
  padding: 20px 50px;
`;

export default function Route() {
  const params = useParams();

  const { data } = useQuery(ROUTE_QUERY, {
    variables: { routeId: ["ROUTE", params.routeId].join("#") },
  });

  return (
    <Grid2 container spacing={2}>
      <Grid2 xs={4} style={{ overflowY: "scroll" }}>
        <SidebarContainer>
          <Link to="/">Back</Link>
          {data?.route ? (
            <>
              <h2>{data.route.name}</h2>
              <hr />
              {formatDistance(data.route.distance)}
            </>
          ) : (
            <></>
          )}
        </SidebarContainer>
      </Grid2>
      <Grid2 xs={8}>
        <Map routes={data?.route ? [data.route] : []} />
      </Grid2>
    </Grid2>
  );
}
