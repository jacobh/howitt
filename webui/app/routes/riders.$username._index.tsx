import { DEFAULT_VIEW, Map } from "../components/map";
import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";
import {
  Container,
  MapContainer,
  SidebarContainer,
  Nav,
} from "~/components/layout";
import { useParams } from "@remix-run/react";
import { sortBy } from "lodash";
import { RideItem } from "~/components/rides/RideItem";
import { css } from "@emotion/react";
import { COLORS } from "~/styles/theme";

const USER_PROFILE_QUERY = gql(`
  query UserProfileQuery($username: String!, $pointsPerKm: Int!) {
    userWithUsername(username: $username) {
        id
        username
        recentRides {
          id
          date
          pointsJson(pointsPerKm: $pointsPerKm)
          ...rideItem
        }
    }
    viewer {
      ...viewerInfo
    }
  }
`);

const rideItemContainerCss = css`
  padding: 20px 1.5%;
  border-bottom: 1px solid ${COLORS.offWhite};

  &:hover {
    background-color: ${COLORS.offWhite};
  }
`;

export default function UserProfile(): React.ReactElement {
  const params = useParams();

  const { data } = useQuery(USER_PROFILE_QUERY, {
    variables: { username: params.username ?? "", pointsPerKm: 1 },
  });

  const { data: data2 } = useQuery(USER_PROFILE_QUERY, {
    variables: { username: params.username ?? "", pointsPerKm: 8 },
    ssr: false,
  });

  const sidebarRides = sortBy(
    data?.userWithUsername?.recentRides ?? [],
    (ride) => ride.date
  )
    .reverse()
    .slice(0, 30);

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer
        title="Riders"
        titleLinkTo="/riders"
        titlePostfix={["/", data?.userWithUsername?.username ?? ""].join(" ")}
      >
        {data?.userWithUsername?.username ? (
          <div>
            {sidebarRides.map((ride) => (
              <div key={ride.id} css={rideItemContainerCss}>
                <RideItem ride={ride} />
              </div>
            ))}
          </div>
        ) : (
          <h3>User not found</h3>
        )}
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
