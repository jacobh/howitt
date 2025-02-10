import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { gql } from "../__generated__/gql";
import {
  Container,
  MapContainer,
  SidebarContainer,
  Nav,
} from "~/components/layout";
import { css } from "@emotion/react";
import { tokens } from "~/styles/tokens";
import { TripItem } from "~/components/trips/TripItem";
import { PrimaryMap } from "~/components/map/PrimaryMap";
import { DEFAULT_INITIAL_VIEW } from "~/components/map";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";
import { useMemo } from "react";
import { buildRideTrack } from "~/components/map/types";

const TripsQueryNoPoints = gql(`
  query TripsQuery {
    trips {
      id
      name
      legs {
        rides {
          id
          pointsJson(pointsPerKm: 1)
        }
      }
      ...tripItem
    }
    viewer {
      ...viewerInfo
    }
  }
`);

const TripsQueryWithPoints = gql(`
  query TripsQueryPoints {
    trips {
      id
      legs {
        rides {
          id
          pointsJson(pointsPerKm: 8)
        }
      }
    }
  }
`);

const tripItemContainerCss = css`
  padding: 20px 1.5%;
  border-bottom: 1px solid ${tokens.colors.offWhite};

  &:hover {
    background-color: ${tokens.colors.offWhite};
  }
`;

export default function Trips(): React.ReactElement {
  const { data, loading } = useQuery(TripsQueryNoPoints);
  const { data: data2 } = useQuery(TripsQueryWithPoints, {
    ssr: false,
  });

  const tracks = useMemo(() => {
    const trips = data2?.trips ?? data?.trips ?? [];
    return trips
      .flatMap((trip) => trip.legs)
      .flatMap((leg) => leg.rides)
      .map((ride) => buildRideTrack(ride, "default"));
  }, [data?.trips, data2?.trips]);

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer titleSegments={[{ name: "Trips", linkTo: "/trips" }]}>
        {loading ? (
          <LoadingSpinnerSidebarContent />
        ) : (
          <div>
            {data?.trips.map((trip) => (
              <div key={trip.id} css={tripItemContainerCss}>
                <TripItem trip={trip} />
              </div>
            ))}
          </div>
        )}
      </SidebarContainer>
      <MapContainer>
        <PrimaryMap tracks={tracks} initialView={DEFAULT_INITIAL_VIEW} />
      </MapContainer>
    </Container>
  );
}
