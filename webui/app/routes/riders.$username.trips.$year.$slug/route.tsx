import { useParams } from "@remix-run/react";
import {
  Container,
  MapContainer,
  Nav,
  SidebarContainer,
} from "~/components/layout";
import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { gql } from "~/__generated__";
import { ElevationProfile } from "~/components/ElevationProfile";
import { useMemo, useState } from "react";
import { EditTripModal } from "~/components/trips/EditTripModal";
import { css } from "@emotion/react";
import { PrimaryMap } from "~/components/map/PrimaryMap";
import { buildRideTrack, Marker } from "~/components/map/types";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";
import { ContentBlock } from "./components/ContentBlock";
import { useVisibleContent } from "./hooks/useVisibleContent";
import { create } from "mutative";

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
        media {
          id
          point
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
          ...contentBlock
        }
      }
    }
  }
`);

const elevationContainerStyles = css({
  marginTop: "12px",
});

const temporalBlocksContainerStyles = css({
  margin: "20px 0",
});

export default function TripDetail(): React.ReactElement {
  const params = useParams();
  const [isEditModalOpen, setEditModalOpen] = useState(false);
  const { visibleRouteIds, visibleMediaIds, onContentBlockEvent } =
    useVisibleContent();

  const { data, loading, refetch } = useQuery(TripQuery, {
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

  const trip =
    data2?.userWithUsername?.tripWithSlug ??
    data?.userWithUsername?.tripWithSlug;

  const allRides = useMemo(
    () => trip?.legs.flatMap((leg) => leg.rides) ?? [],
    [trip?.legs],
  );

  const rideIdRideMap = useMemo(() => {
    return new Map(
      allRides.map((ride) => [ride.id, buildRideTrack(ride)] as const),
    );
  }, [allRides]);

  const initialView = useMemo(
    () => ({
      type: "tracks" as const,
      trackIds: allRides.map(({ id }) => id),
    }),
    [allRides],
  );

  const baseRideTracks = useMemo(
    () =>
      (
        data2?.userWithUsername?.tripWithSlug ??
        data?.userWithUsername?.tripWithSlug
      )?.legs
        .flatMap((leg) => leg.rides)
        .map((ride) => buildRideTrack(ride, "default")) ?? [],
    [
      data2?.userWithUsername?.tripWithSlug,
      data?.userWithUsername?.tripWithSlug,
    ],
  );

  const tracks = useMemo(
    () =>
      baseRideTracks.map((track) =>
        create(track, (draft) => {
          draft.style = visibleRouteIds.has(track.id)
            ? "highlighted"
            : "default";
        }),
      ),
    [baseRideTracks, visibleRouteIds],
  );

  const markers = useMemo(() => {
    if (!trip?.media) return [];

    return trip.media
      .filter(
        (media): media is typeof media & { point: number[] } =>
          media.point != null,
      )
      .filter((media) => visibleMediaIds.has(media.id))
      .map(
        (media): Marker => ({
          id: media.id,
          point: [media.point[0], media.point[1]],
          style: "default",
        }),
      );
  }, [trip?.media, visibleMediaIds]);

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
              {trip.temporalContentBlocks.map((block, i) => (
                <ContentBlock
                  key={i}
                  block={block}
                  rideIdRideMap={rideIdRideMap}
                  onEvent={onContentBlockEvent}
                />
              ))}
            </div>

            {isEditModalOpen ? (
              <EditTripModal
                trip={trip}
                isOpen={true}
                onClose={(): void => setEditModalOpen(false)}
                refetch={refetch}
              />
            ) : (
              <></>
            )}
          </>
        ) : loading ? (
          <LoadingSpinnerSidebarContent />
        ) : (
          <h3>Trip not found</h3>
        )}
      </SidebarContainer>

      <MapContainer>
        <PrimaryMap
          initialView={initialView}
          tracks={tracks}
          markers={markers}
        />
      </MapContainer>
    </Container>
  );
}
