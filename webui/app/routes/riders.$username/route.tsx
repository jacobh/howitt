import { DEFAULT_VIEW, Map } from "../../components/map";
import { useQuery } from "@apollo/client";
import { gql } from "../../__generated__/gql";
import {
  Container,
  MapContainer,
  SidebarContainer,
  Nav,
} from "~/components/layout";
import { useParams } from "@remix-run/react";

const USER_PROFILE_QUERY = gql(`
  query UserProfileQuery($username: String!) {
    userWithUsername(username: $username) {
        id
        username
    }
    viewer {
      ...viewerInfo
    }
  }
`);

export default function UserProfile(): React.ReactElement {
  const params = useParams();

  const { data, loading } = useQuery(USER_PROFILE_QUERY, {
    variables: { username: params.username ?? "" },
  });

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer
        title="Riders"
        titlePostfix={["/", data?.userWithUsername?.username ?? ""].join(" ")}
      >
        {!loading && !data?.userWithUsername?.username ? (
          <h3>User not found</h3>
        ) : null}
      </SidebarContainer>
      <MapContainer>
        <Map
          initialView={{
            type: "view",
            view: DEFAULT_VIEW,
          }}
        />
      </MapContainer>
    </Container>
  );
}
