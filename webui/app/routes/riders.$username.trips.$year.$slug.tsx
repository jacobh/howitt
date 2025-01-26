import { useParams } from "@remix-run/react";
import {
  Container,
  MapContainer,
  Nav,
  SidebarContainer,
} from "~/components/layout";
import { Map } from "~/components/map";
import { RideSummary } from "~/components/rides/RideSummary";
import { useQuery } from "@apollo/client";
import { gql } from "~/__generated__";
import { ElevationProfile } from "~/components/ElevationProfile";

const TRIP_QUERY = gql(`
  query TripQuery($username: String!, $slug: String!, $pointsPerKm: Int!) {
    viewer {
      ...viewerInfo
    }

    userWithUsername(username: $username) {
      username
      tripWithSlug(slug: $slug) {
        id
        name
        description
        legs {
          rides {
            id
            date
            pointsJson(pointsPerKm: $pointsPerKm)
            ...rideSummary
            ...elevationPath
          }
        }
      }
    }
  }
`);

export default function TripDetail(): React.ReactElement {
  const params = useParams();

  const { data } = useQuery(TRIP_QUERY, {
    variables: {
      username: params.username ?? "",
      slug: params.slug ?? "",
      pointsPerKm: 1,
    },
  });

  const { data: data2 } = useQuery(TRIP_QUERY, {
    variables: {
      username: params.username ?? "",
      slug: params.slug ?? "",
      pointsPerKm: 20,
    },

    ssr: false,
  });

  const trip = data?.userWithUsername?.tripWithSlug;
  const allRides = trip?.legs.flatMap((leg) => leg.rides) ?? [];

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

          ...(trip
            ? [
                {
                  name: trip.name,
                  linkTo: `/riders/${params.username}/trips/${params.year}/${params.slug}`,
                },
              ]
            : []),
        ]}
      >
        {trip ? (
          <>
            {trip.description && (
              <section css={{ margin: "24px 0" }}>
                <p>{trip.description}</p>
              </section>
            )}

            {allRides.map((ride) => (
              <div key={ride.id}>
                <div css={{ marginTop: "12px" }}>
                  <ElevationProfile data={ride} />
                </div>
                <RideSummary ride={ride} />
              </div>
            ))}
          </>
        ) : (
          <h3>Trip not found</h3>
        )}
      </SidebarContainer>

      <MapContainer>
        <Map
          // TODO this should show all rides
          initialView={{
            type: "ride",
            rideId: allRides[0]?.id,
          }}
          rides={
            (
              data2?.userWithUsername?.tripWithSlug ??
              data?.userWithUsername?.tripWithSlug
            )?.legs.flatMap((leg) => leg.rides) ?? []
          }
        />
      </MapContainer>
    </Container>
  );
}
