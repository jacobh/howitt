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
  query UserProfileQuery($username: String!, $pointsPerKm: Int!) {
    userWithUsername(username: $username) {
        id
        username
        recentRides {
          id
          finishedAt
          points(pointsPerKm: $pointsPerKm)
        }
    }
    viewer {
      ...viewerInfo
    }
  }
`);

export default function UserProfile(): React.ReactElement {
  const params = useParams();

  const { data, loading } = useQuery(USER_PROFILE_QUERY, {
    variables: { username: params.username ?? "", pointsPerKm: 1 },
  });

  const { data: data2 } = useQuery(USER_PROFILE_QUERY, {
    variables: { username: params.username ?? "", pointsPerKm: 8 },
    ssr: false,
  });

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer
        title="Riders"
        titleLinkTo="/riders"
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
          rides={
            data2?.userWithUsername?.recentRides ??
            data?.userWithUsername?.recentRides
          }
        />
      </MapContainer>
    </Container>
  );
}
