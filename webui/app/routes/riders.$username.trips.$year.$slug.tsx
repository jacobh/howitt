import { useParams } from "@remix-run/react";
import {
  Container,
  MapContainer,
  Nav,
  SidebarContainer,
} from "~/components/layout";
import { Map } from "~/components/map";
import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { gql } from "~/__generated__";
import { ElevationProfile } from "~/components/ElevationProfile";
import { RideItem } from "~/components/rides/RideItem";
import { useState } from "react";
import { EditTripModal } from "~/components/trips/EditTripModal";
import { match } from "ts-pattern";
import { css } from "@emotion/react";

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

const rideItemStyles = css({
  margin: "20px 0",
});

const noteStyles = css({
  margin: "24px 0",
});

const mediaStyles = css({
  width: "100%",
  height: "auto",
  borderRadius: "4px",
  margin: "16px 0",
});

const elevationContainerStyles = css({
  marginTop: "12px",
});

const temporalBlocksContainerStyles = css({
  margin: "20px 0",
});

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
                <div css={elevationContainerStyles}>
                  <ElevationProfile data={leg} />
                </div>
              </div>
            ))}

            <div css={temporalBlocksContainerStyles}>
              {trip.temporalContentBlocks.map((block) =>
                match(block)
                  .with({ __typename: "Ride" }, (ride) => (
                    <div key={`ride-${ride.rideId}`} css={rideItemStyles}>
                      <RideItem ride={ride} />
                    </div>
                  ))
                  .with({ __typename: "Note" }, (note) => (
                    <section key={`note-${note.contentAt}`} css={noteStyles}>
                      <p>{note.text}</p>
                    </section>
                  ))
                  .with({ __typename: "Media" }, (media) => (
                    <img
                      key={`media-${media.mediaId}`}
                      src={media.imageSizes.fit1600.webpUrl}
                      css={mediaStyles}
                      alt=""
                    />
                  ))
                  .exhaustive(),
              )}
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
