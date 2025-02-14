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
import { useMemo, useState } from "react";
import { buildRideTrack } from "~/components/map/types";
import { CreateTripModal } from "~/components/trips/CreateTripModal";

const TripsQueryNoPoints = gql(`
  query TripsQuery {
    publishedTrips {
      id
      name
      legs {
        rides {
          id
          pointsJson(detailLevel: LOW)
        }
      }
      ...tripItem
    }
    viewer {
      profile {
        id
        username
      }
      ...viewerInfo
    }
  }
`);

const TripsQueryWithPoints = gql(`
  query TripsQueryPoints {
    publishedTrips {
      id
      legs {
        rides {
          id
          pointsJson(detailLevel: MEDIUM) 
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

const createTripButtonCss = css`
  background-color: white;
  border: 1px solid ${tokens.colors.lightGrey};
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9em;

  &:hover {
    background-color: ${tokens.colors.offWhite};
  }

  &:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }
`;

export default function Trips(): React.ReactElement {
  const [isCreateModalOpen, setCreateModalOpen] = useState(false);

  const { data, loading } = useQuery(TripsQueryNoPoints);
  const { data: data2 } = useQuery(TripsQueryWithPoints, {
    ssr: false,
  });

  const tracks = useMemo(() => {
    const trips = data2?.publishedTrips ?? data?.publishedTrips ?? [];

    return trips
      .flatMap((trip) => trip.legs)
      .flatMap((leg) => leg.rides)
      .map((ride) => buildRideTrack(ride, "default"));
  }, [data?.publishedTrips, data2?.publishedTrips]);

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer titleSegments={[{ name: "Trips", linkTo: "/trips" }]}>
        {data?.viewer && (
          <button
            onClick={(): void => setCreateModalOpen(true)}
            css={createTripButtonCss}
          >
            Create Trip
          </button>
        )}
        {loading ? (
          <LoadingSpinnerSidebarContent />
        ) : (
          <div>
            {data?.publishedTrips.map((trip) => (
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
      {data?.viewer && (
        <CreateTripModal
          isOpen={isCreateModalOpen}
          onClose={(): void => setCreateModalOpen(false)}
          username={data.viewer.profile.username}
        />
      )}
    </Container>
  );
}
