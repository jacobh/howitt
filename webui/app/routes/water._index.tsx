import { DEFAULT_INITIAL_VIEW } from "../components/map";
import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { gql } from "../__generated__/gql";
import {
  Container,
  MapContainer,
  SidebarContainer,
  Nav,
} from "~/components/layout";
import { PrimaryMap } from "~/components/map/PrimaryMap";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";

const ViewerQuery = gql(`
  query viewerQuery {
    viewer {
      ...viewerInfo
    }
  }
`);

export default function Water(): React.ReactElement {
  const { data, loading } = useQuery(ViewerQuery);

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer titleSegments={[{ name: "Water", linkTo: "/water" }]}>
        {loading ? <LoadingSpinnerSidebarContent /> : <></>}
      </SidebarContainer>
      <MapContainer>
        <PrimaryMap initialView={DEFAULT_INITIAL_VIEW} />
      </MapContainer>
    </Container>
  );
}
