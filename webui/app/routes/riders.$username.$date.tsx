import { useParams } from "react-router";
import {
  Container,
  MapContainer,
  Nav,
  SidebarContainer,
} from "~/components/layout";
import { RideSummary } from "~/components/rides/RideSummary";
import { Temporal } from "@js-temporal/polyfill";
import { gql } from "~/__generated__";
import { useQuery } from "@apollo/client/react";
import { ElevationProfile } from "~/components/ElevationProfile";
import { PrimaryMap } from "~/components/map/PrimaryMap";
import { buildRideTrack, Marker } from "~/components/map/types";
import { useMemo, useState } from "react";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";
import { PointsDetail } from "~/__generated__/graphql";
import { formatLongDate } from "~/services/format";

const RidesWithDateQuery = gql(`
  query ridesWithDate($username: String!, $date: IsoDate!, $detailLevel: PointsDetail!) {
    viewer {
      ...viewerInfo
    }
    userWithUsername(username: $username) {
      username
      ridesWithDate(date: $date) {
        id
        date
        tz
        pointsJson(detailLevel: $detailLevel)
        ...rideSummary
        ...elevationPath
      }
    }
  }
`);

function UserProfileDate(): React.ReactElement {
  const params = useParams();
  const [hoveredPoint, setHoveredPoint] = useState<
    { rideId: string; pointIndex: number } | undefined
  >();

  const { data, loading } = useQuery(RidesWithDateQuery, {
    variables: {
      username: params.username ?? "",
      date: params.date ?? "",
      detailLevel: PointsDetail.Low,
    },
  });

  const { data: data2 } = useQuery(RidesWithDateQuery, {
    variables: {
      username: params.username ?? "",
      date: params.date ?? "",
      detailLevel: PointsDetail.High,
    },
    ssr: false,
  });

  const initialView = useMemo(
    () =>
      data?.userWithUsername?.ridesWithDate
        ? {
            type: "tracks" as const,
            trackIds: data.userWithUsername.ridesWithDate.map(({ id }) => id),
          }
        : undefined,
    [data],
  );

  const rides =
    data2?.userWithUsername?.ridesWithDate ??
    data?.userWithUsername?.ridesWithDate;

  const tracks = useMemo(
    () => rides?.map((ride) => buildRideTrack(ride)) ?? [],
    [rides],
  );

  const markers = useMemo((): Marker[] => {
    if (!hoveredPoint) {
      return [];
    }

    const point = tracks.find(({ id }) => id === hoveredPoint.rideId)?.points[
      hoveredPoint.pointIndex
    ];
    return point
      ? [{ id: "elevation-profile", point, style: "elevation" }]
      : [];
  }, [hoveredPoint, tracks]);

  // Format the date for display using Temporal
  const displayDate = useMemo(() => {
    const firstRide = data?.userWithUsername?.ridesWithDate?.at(0);

    if (!params.date || !firstRide) {
      return;
    }

    return formatLongDate(Temporal.PlainDate.from(params.date));
  }, [params.date, data]);

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
                    <ElevationProfile
                      data={ride}
                      onPointHover={(pointIndex): void =>
                        setHoveredPoint(
                          pointIndex === undefined
                            ? undefined
                            : { rideId: ride.id, pointIndex },
                        )
                      }
                    />
                  </div>
                  <RideSummary ride={ride} />
                </div>
              ))
            ) : (
              <p>No rides found for this date</p>
            )}
          </>
        ) : loading ? (
          <LoadingSpinnerSidebarContent />
        ) : (
          <h3>User not found</h3>
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

export default UserProfileDate;
