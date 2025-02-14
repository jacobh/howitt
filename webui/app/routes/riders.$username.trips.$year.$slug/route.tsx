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
import { useCallback, useMemo, useState } from "react";
import { EditTripModal } from "~/components/trips/EditTripModal";
import { css } from "@emotion/react";
import { PrimaryMap } from "~/components/map/PrimaryMap";
import { buildRideTrack, Marker } from "~/components/map/types";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";
import { ContentBlock, ContentBlockEvent } from "./components/ContentBlock";
import { useVisibleContent } from "./hooks/useVisibleContent";
import { create } from "mutative";
import { useUpdatePrimaryMapView } from "~/components/map/hooks/useUpdatePrimaryMapView";
import { match } from "ts-pattern";
import LineString from "ol/geom/LineString";
import { PointsDetail } from "~/__generated__/graphql";
import { tokens } from "~/styles/tokens";

const TripQuery = gql(`
  query TripQuery($username: String!, $slug: String!, $detailLevel: PointsDetail!) {
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
            pointsJson(detailLevel: $detailLevel)
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

const infoBoxStyles = css({
  backgroundColor: "#f5f5f5",
  padding: "12px 16px",
  borderRadius: "8px",
  fontSize: "14px",
  color: "#666",
  marginTop: "12px",
  marginBottom: "20px",
});

const editTripButtonCss = css`
  background-color: white;
  border: 1px solid ${tokens.colors.lightGrey};
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
  margin-left: 1.5%;
  font-size: 0.9em;

  &:hover {
    background-color: ${tokens.colors.offWhite};
  }

  &:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }
`;

export default function TripDetail(): React.ReactElement {
  const params = useParams();
  const [isOverlayActive, setOverlayActive] = useState(false);
  const [isEditModalOpen, setEditModalOpen] = useState(false);
  const {
    visibleRouteIds,
    visibleMediaIds,
    hoveredMediaIds,
    onContentBlockEvent: handleVisibilityEvent,
  } = useVisibleContent();

  const { updateView } = useUpdatePrimaryMapView();

  const { data, loading, refetch } = useQuery(TripQuery, {
    variables: {
      username: params.username ?? "",
      slug: params.slug ?? "",
      detailLevel: PointsDetail.Low,
    },
  });

  const { data: data2 } = useQuery(TripQuery, {
    variables: {
      username: params.username ?? "",
      slug: params.slug ?? "",
      detailLevel: PointsDetail.High,
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
          style: hoveredMediaIds.has(media.id) ? "highlighted" : "default",
        }),
      );
  }, [trip?.media, visibleMediaIds, hoveredMediaIds]);

  const isOwnTrip =
    data?.viewer?.id === data?.userWithUsername?.tripWithSlug?.user?.id;

  const onContentBlockEvent = useCallback(
    (event: ContentBlockEvent) => {
      handleVisibilityEvent(event);

      match(event)
        .with({ contentType: "Media", eventType: "click" }, (event) => {
          const media = trip?.media.find(
            (media) => media.id === event.contentBlockId,
          );

          if (!media || !media.point) return;

          const point = [media.point[0], media.point[1]];

          setOverlayActive(true);
          updateView((view) => {
            view.setCenter(point);
            view.setZoom(12);
          });
        })
        .with({ contentType: "Ride", eventType: "click" }, (event) => {
          const ride = trip?.legs[0].rides.find(
            (ride) => ride.id === event.contentBlockId,
          );

          if (!ride) return;

          const lineString = new LineString(JSON.parse(ride.pointsJson));

          const bounds = lineString.getExtent();

          setOverlayActive(true);
          updateView((view) => {
            view.fit(bounds, {
              padding: [100, 100, 100, 100],
              duration: 0,
            });
          });
        })
        .otherwise(() => {
          // no-op
        });
    },
    [handleVisibilityEvent, updateView, trip?.media, trip?.legs],
  );

  const onDismissOverlay = useCallback(() => {
    setOverlayActive(false);
  }, []);

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
              <button
                onClick={(): void => setEditModalOpen(true)}
                css={editTripButtonCss}
              >
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

            <div css={infoBoxStyles}>
              Tap on minimaps or photos to see them on the interactive map
            </div>

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

      <MapContainer
        isOverlayActive={isOverlayActive}
        onDismissOverlay={onDismissOverlay}
      >
        <PrimaryMap
          initialView={initialView}
          tracks={tracks}
          markers={markers}
        />
      </MapContainer>
    </Container>
  );
}
