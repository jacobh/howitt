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
import { css } from "@emotion/react";
import { tokens } from "~/styles/tokens";
import { TripItem } from "~/components/trips/TripItem";
import { PrimaryMap } from "~/components/map/PrimaryMap";
import { buildRideTrack } from "~/components/map/types";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";
import { PointsDetail } from "~/__generated__/graphql";
import { useMemo } from "react";

const UserProfileQuery = gql(`
  query UserProfileQuery($username: String!, $detailLevel: PointsDetail!) {
    userWithUsername(username: $username) {
        id
        username
        recentRides {
          id
          date
          pointsJson(detailLevel: $detailLevel)
          ...rideItem
        }
        trips {
          id
          name
          legs {
            rides {
              id
              pointsJson(detailLevel: $detailLevel)
            }
          }
          ...tripItem
        }
    }
    viewer {
      id
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
  padding: 20px 1.5% 0;
  font-size: 1.2em;
  font-weight: bold;
`;

export default function UserProfile(): React.ReactElement {
  const params = useParams();

  const { data, loading } = useQuery(UserProfileQuery, {
    variables: {
      username: params.username ?? "",
      detailLevel: PointsDetail.Low,
    },
  });

  const { data: data2 } = useQuery(UserProfileQuery, {
    variables: {
      username: params.username ?? "",
      detailLevel: PointsDetail.High,
    },
    ssr: false,
  });

  const tracks = useMemo(() => {
    const trips =
      data2?.userWithUsername?.trips ?? data?.userWithUsername?.trips ?? [];

    return trips
      .flatMap((trip) => trip.legs)
      .flatMap((leg) => leg.rides)
      .map((ride) => buildRideTrack(ride, "default"));
  }, [data2?.userWithUsername?.trips, data?.userWithUsername?.trips]);

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
          </div>
        ) : loading ? (
          <LoadingSpinnerSidebarContent />
        ) : (
          <h3>User not found</h3>
        )}
      </SidebarContainer>
      <MapContainer>
        <PrimaryMap initialView={DEFAULT_INITIAL_VIEW} tracks={tracks} />
      </MapContainer>
    </Container>
  );
}
