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

const TripQuery = gql(`
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
        ...editTrip
        user {
          id
        }
        legs {
          ...elevationPath
          rides {
            id
            ...elevationPath
            pointsJson(pointsPerKm: $pointsPerKm)
          }
        }
        temporalContentBlocks {
          __typename
          contentAt
          ... on Ride {
            rideId: id
            date
            ...rideItem
          }
          ... on Media {
            mediaId: id
            imageSizes {
              fit1600 {
                webpUrl
              }
            }
          }
          ... on Note {
            text
          }
        }
      }
    }
  }
`);

export default function TripDetail(): React.ReactElement {
  const params = useParams();
  const [isEditModalOpen, setEditModalOpen] = useState(false);

  const { data, refetch } = useQuery(TripQuery, {
    variables: {
      username: params.username ?? "",
      slug: params.slug ?? "",
      pointsPerKm: 1,
    },
  });

  const { data: data2 } = useQuery(TripQuery, {
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
              </div>
            ))}

            {/* New content blocks rendering */}
            <div css={{ margin: "20px 0" }}>
              {trip.temporalContentBlocks.map((block) => (
                <div
                  key={`${block.__typename}-${(block as any).rideId ?? (block as any).mediaId ?? block.contentAt}`}
                >
                  {block.__typename === "Ride" && (
                    <div css={{ margin: "20px 0" }}>
                      <RideItem ride={block} />
                    </div>
                  )}

                  {block.__typename === "Note" && (
                    <section css={{ margin: "24px 0" }}>
                      <p>{block.text}</p>
                    </section>
                  )}

                  {block.__typename === "Media" && (
                    <img
                      src={block.imageSizes.fit1600.webpUrl}
                      css={{
                        width: "100%",
                        height: "auto",
                        borderRadius: "4px",
                        margin: "16px 0",
                      }}
                      alt=""
                    />
                  )}
                </div>
              ))}
            </div>

            <EditTripModal
              trip={trip}
              isOpen={isEditModalOpen}
              onClose={(): void => setEditModalOpen(false)}
              refetch={refetch}
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
