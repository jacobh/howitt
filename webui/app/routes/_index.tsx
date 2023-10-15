import { Map } from "../components/map";
import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";
import { useState } from "react";
import { Container, MapContainer, SidebarContainer } from "~/components/layout";
import { RouteItem } from "~/components/RouteItem";

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

export default function Index(): React.ReactElement {
  const [mode] = useState("routes");

  const { data } = useQuery(HOME_QUERY);

  return (
    <Container>
      <SidebarContainer title="Routes">
        {data?.starredRoutes.map((route) => (
          <RouteItem key={route.id} route={route} />
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
