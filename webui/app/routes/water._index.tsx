import { DEFAULT_INITIAL_VIEW } from "../components/map";
import { useQuery as useGqlQuery } from "@apollo/client/react/hooks/useQuery";
import { gql } from "../__generated__/gql";
import {
  Container,
  MapContainer,
  SidebarContainer,
  Nav,
} from "~/components/layout";
import { PrimaryMap } from "~/components/map/PrimaryMap";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";
import { useQuery } from "@tanstack/react-query";

const ViewerQuery = gql(`
  query viewerQuery {
    viewer {
      ...viewerInfo
    }
  }
`);

export default function Water(): React.ReactElement {
  const { data: viewerData, loading: viewerLoading } = useGqlQuery(ViewerQuery);

  const { isPending, isError, data, error } = useQuery({
    queryKey: ["waterFeatureIndex"],
    queryFn: async () => {
      const resp = await fetch(
        // "https://ts-api.howittplains.net/api/water-features",
        "http://localhost:3001/api/water-features",
      );
      const data = await resp.json();

      return data;
    },
  });

  console.log({ isPending, isError, data, error });

  return (
    <Container>
      <Nav viewer={viewerData?.viewer} />
      <SidebarContainer titleSegments={[{ name: "Water", linkTo: "/water" }]}>
        {viewerLoading ? <LoadingSpinnerSidebarContent /> : <></>}
      </SidebarContainer>
      <MapContainer>
        <PrimaryMap initialView={DEFAULT_INITIAL_VIEW} />
      </MapContainer>
    </Container>
  );
}
