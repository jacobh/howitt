import { useParams } from "@remix-run/react";
import {
  Container,
  MapContainer,
  Nav,
  SidebarContainer,
} from "~/components/layout";
import { DEFAULT_VIEW, Map } from "~/components/map";
import { RideSummary } from "~/components/rides/RideSummary";
import { Temporal } from "@js-temporal/polyfill";
import { gql } from "~/__generated__";
import { useQuery } from "@apollo/client";

const RIDES_WITH_DATE_QUERY = gql(`
  query ridesWithDate($username: String!, $date: IsoDate!, $pointsPerKm: Int!) {
    viewer {
      ...viewerInfo
    }
    userWithUsername(username: $username) {
      username
      ridesWithDate(date: $date) {
        id
        ...rideSummary
        pointsJson(pointsPerKm: $pointsPerKm)
      }
    }
  }
`);

function UserProfileDate(): React.ReactElement {
  const params = useParams();

  const { data } = useQuery(RIDES_WITH_DATE_QUERY, {
    variables: {
      username: params.username ?? "",
      date: params.date ?? "",
      pointsPerKm: 1,
    },
  });

  const { data: data2 } = useQuery(RIDES_WITH_DATE_QUERY, {
    variables: {
      username: params.username ?? "",
      date: params.date ?? "",
      pointsPerKm: 8,
    },
    ssr: false,
  });

  // Format the date for display using Temporal
  const displayDate = Temporal.PlainDate.from(params.date ?? "").toLocaleString(
    undefined,
    {
      weekday: "long",
      year: "numeric",
      month: "long",
      day: "numeric",
    },
  );

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer
        title="Riders"
        titleLinkTo="/riders"
        titlePostfix={[
          "/",
          data?.userWithUsername?.username ?? "",
          "/",
          displayDate,
        ].join(" ")}
      >
        {data?.userWithUsername?.username ? (
          <>
            {data?.userWithUsername?.ridesWithDate?.length ? (
              data.userWithUsername.ridesWithDate.map((ride) => (
                <RideSummary key={ride.id} ride={ride} />
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
        <Map
          initialView={{
            type: "view",
            view: DEFAULT_VIEW,
          }}
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
