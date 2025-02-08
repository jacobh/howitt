import { DEFAULT_INITIAL_VIEW } from "../components/map";
import { useQuery } from "@apollo/client/react/hooks/useQuery";
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
import { tokens } from "~/styles/tokens";
import { TripItem } from "~/components/trips/TripItem";
import { PrimaryMap } from "~/components/map/PrimaryMap";
import { buildRideTrack } from "~/components/map/types";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";

const UserProfileQuery = gql(`
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
        trips {
          id
          name
          ...tripItem
        }
    }
    viewer {
      ...viewerInfo
    }
  }
`);

const rideItemContainerCss = css`
  padding: 20px 1.5%;
  border-bottom: 1px solid ${tokens.colors.offWhite};

  &:hover {
    background-color: ${tokens.colors.offWhite};
  }
`;

const sectionHeaderCss = css`
  padding: 20px 1.5%;
  font-size: 1.2em;
  font-weight: bold;
`;

export default function UserProfile(): React.ReactElement {
  const params = useParams();

  const { data, loading } = useQuery(UserProfileQuery, {
    variables: { username: params.username ?? "", pointsPerKm: 1 },
  });

  const { data: data2 } = useQuery(UserProfileQuery, {
    variables: { username: params.username ?? "", pointsPerKm: 8 },
    ssr: false,
  });

  const sidebarRides = sortBy(
    data?.userWithUsername?.recentRides ?? [],
    (ride) => ride.date,
  )
    .reverse()
    .slice(0, 30);

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer
        titleSegments={[
          { name: "Riders", linkTo: "/riders" },
          ...(data?.userWithUsername
            ? [
                {
                  name: data.userWithUsername.username,
                  linkTo: `/riders/${data.userWithUsername.username}`,
                },
              ]
            : []),
        ]}
      >
        {data?.userWithUsername?.username ? (
          <div>
            <div css={sectionHeaderCss}>Trips</div>
            {data.userWithUsername.trips.map((trip) => (
              <div key={trip.id} css={rideItemContainerCss}>
                <TripItem trip={trip} />
              </div>
            ))}

            <div css={sectionHeaderCss}>Recent Rides</div>
            {sidebarRides.map((ride) => (
              <div key={ride.id} css={rideItemContainerCss}>
                <RideItem ride={ride} />
              </div>
            ))}
          </div>
        ) : loading ? (
          <LoadingSpinnerSidebarContent />
        ) : (
          <h3>User not found</h3>
        )}
      </SidebarContainer>
      <MapContainer>
        <PrimaryMap
          initialView={DEFAULT_INITIAL_VIEW}
          tracks={(
            data2?.userWithUsername?.recentRides ??
            data?.userWithUsername?.recentRides
          )?.map((ride) => buildRideTrack(ride))}
        />
      </MapContainer>
    </Container>
  );
}
