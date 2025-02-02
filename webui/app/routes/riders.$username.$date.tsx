import { useParams } from "@remix-run/react";
import {
  Container,
  MapContainer,
  Nav,
  SidebarContainer,
} from "~/components/layout";
import { RideSummary } from "~/components/rides/RideSummary";
import { Temporal } from "@js-temporal/polyfill";
import { gql } from "~/__generated__";
import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { ElevationProfile } from "~/components/ElevationProfile";
import { PrimaryMap } from "~/components/map/PrimaryMap";

const RidesWithDateQuery = gql(`
  query ridesWithDate($username: String!, $date: IsoDate!, $pointsPerKm: Int!) {
    viewer {
      ...viewerInfo
    }
    userWithUsername(username: $username) {
      username
      ridesWithDate(date: $date) {
        id
        date
        pointsJson(pointsPerKm: $pointsPerKm)
        ...rideSummary
        ...elevationPath
      }
    }
  }
`);

function UserProfileDate(): React.ReactElement {
  const params = useParams();

  const { data } = useQuery(RidesWithDateQuery, {
    variables: {
      username: params.username ?? "",
      date: params.date ?? "",
      pointsPerKm: 1,
    },
  });

  const { data: data2 } = useQuery(RidesWithDateQuery, {
    variables: {
      username: params.username ?? "",
      date: params.date ?? "",
      pointsPerKm: 50,
    },
    ssr: false,
  });

  // Format the date for display using Temporal
  const displayDate = params.date
    ? Temporal.PlainDate.from(params.date).toLocaleString("en-US", {
        weekday: "long",
        year: "numeric",
        month: "long",
        day: "numeric",
      })
    : undefined;

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer
        titleSegments={[
          { name: "Riders", linkTo: "/riders" },
          ...(params.username
            ? [
                {
                  name: params.username,
                  linkTo: `/riders/${params.username}`,
                },
              ]
            : []),
          ...(params.username && params.date && displayDate
            ? [
                {
                  name: displayDate,
                  linkTo: `/riders/${params.username}/${params.date}`,
                },
              ]
            : []),
        ]}
      >
        {data?.userWithUsername?.username ? (
          <>
            {data?.userWithUsername?.ridesWithDate?.length ? (
              data.userWithUsername.ridesWithDate.map((ride) => (
                <div key={ride.id}>
                  <div css={{ marginTop: "12px" }}>
                    <ElevationProfile data={ride} />
                  </div>
                  <RideSummary ride={ride} />
                </div>
              ))
            ) : (
              <p>No rides found for this date</p>
            )}
          </>
        ) : (
          <h3>User not found</h3>
        )}
      </SidebarContainer>
      <MapContainer>
        <PrimaryMap
          initialView={
            data?.userWithUsername?.ridesWithDate
              ? {
                  type: "rides",
                  rideIds: data.userWithUsername.ridesWithDate.map(
                    ({ id }) => id,
                  ),
                }
              : undefined
          }
          rides={
            data2?.userWithUsername?.ridesWithDate ??
            data?.userWithUsername?.ridesWithDate
          }
        />
      </MapContainer>
    </Container>
  );
}

export default UserProfileDate;
