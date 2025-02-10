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

const TripsQuery = gql(`
  query TripsQuery {
    trips {
      id
      ...tripItem
    }
    viewer {
      ...viewerInfo
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
  const { data, loading } = useQuery(TripsQuery);

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
        <PrimaryMap initialView={DEFAULT_INITIAL_VIEW} />
      </MapContainer>
    </Container>
  );
}
