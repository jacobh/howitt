import { useParams } from "@remix-run/react";
import {
  Container,
  MapContainer,
  Nav,
  SidebarContainer,
} from "~/components/layout";
import { Map } from "~/components/map";
import { useQuery } from "@apollo/client";
import { gql } from "~/__generated__";
import { ElevationProfile } from "~/components/ElevationProfile";
import { RideItem } from "~/components/rides/RideItem";
import { useState } from "react";
import { EditTripModal } from "~/components/trips/EditTripModal";

const TRIP_QUERY = gql(`
  query TripQuery($username: String!, $slug: String!, $pointsPerKm: Int!) {
    viewer {
      id
      ...viewerInfo
    }

    userWithUsername(username: $username) {
      username
      tripWithSlug(slug: $slug) {
        id
        name
        description
        ...editTrip
        user {
          id
        }
        legs {
          ...elevationPath
          rides {
            id
            date
            pointsJson(pointsPerKm: $pointsPerKm)
            ...rideItem
            ...elevationPath
          }
        }
      }
    }
  }
`);

export default function TripDetail(): React.ReactElement {
  const params = useParams();
  const [isEditModalOpen, setEditModalOpen] = useState(false);

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

  const isOwnTrip =
    data?.viewer?.id === data?.userWithUsername?.tripWithSlug?.user?.id;

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
            {isOwnTrip && (
              <button onClick={(): void => setEditModalOpen(true)}>
                Edit Trip
              </button>
            )}
            {trip.legs.map((leg, i) => (
              <div key={i}>
                <div css={{ marginTop: "12px" }}>
                  <ElevationProfile data={leg} />
                </div>
                {leg.rides.map((ride) => (
                  <div key={ride.id} css={{ margin: "20px 0" }}>
                    <RideItem ride={ride} />
                  </div>
                ))}
              </div>
            ))}

            {trip.description && (
              <section css={{ margin: "24px 0" }}>
                <p>{trip.description}</p>
              </section>
            )}

            <EditTripModal
              trip={trip}
              isOpen={isEditModalOpen}
              onClose={(): void => setEditModalOpen(false)}
            />
          </>
        ) : (
          <h3>Trip not found</h3>
        )}
      </SidebarContainer>

      <MapContainer>
        <Map
          initialView={{
            type: "rides",
            rideIds: allRides.map(({ id }) => id),
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
