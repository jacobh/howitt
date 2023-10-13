import { Map } from "../components/map";
import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";
import { useState } from "react";
import { formatDistance } from "~/services/formatDistance";
import { Link } from "@remix-run/react";
import { Container, MapContainer, SidebarContainer } from "~/components/layout";

const HOME_QUERY = gql(`
  query homeQuery {
    starredRoutes {
      id
      name
      distance
      points
    }
  }
`);

export default function Index(): React.ReactElement {
  const [mode] = useState("routes");

  const { data } = useQuery(HOME_QUERY);

  return (
    <Container>
      <SidebarContainer>
        <h2>Routes</h2>
        <hr />
        {data?.starredRoutes.map((route) => (
          <div key={route.id}>
            <h3>
              <Link to={`/routes/${route.id.split("#")[1]}`}>{route.name}</Link>
            </h3>
            <p>{formatDistance(route.distance)}</p>
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
