import { useParams } from "@remix-run/react";
import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { gql } from "~/__generated__";
import {
  Container,
  MapContainer,
  Nav,
  SidebarContainer,
} from "~/components/layout";
import { PrimaryMap } from "~/components/map/PrimaryMap";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";
import { useMemo } from "react";
import { buildMarker } from "~/components/map/types";
import { css } from "@emotion/react";
import { tokens } from "~/styles/tokens";
import { DEFAULT_INITIAL_VIEW } from "~/components/map";

const POIQuery = gql(`
  query POIQuery($slug: String!) {
    pointOfInterestWithSlug(slug: $slug) {
      id
      name
      point
      pointOfInterestType
      media {
        id
        point
      }
    }
    viewer {
      ...viewerInfo
    }
  }
`);

const poiNameCss = css`
  font-size: 1.25rem;
  line-height: 1.75rem;
  font-weight: 500;
  margin-bottom: 8px;
`;

const poiTypeCss = css`
  color: ${tokens.colors.grey500};
  font-size: 0.875rem;
  line-height: 1.25rem;
`;

const contentContainerCss = css`
  padding: 20px;
`;

export default function POIDetail(): React.ReactElement {
  const params = useParams();

  const { data, loading } = useQuery(POIQuery, {
    variables: { slug: params.slug ?? "" },
  });

  const poi = data?.pointOfInterestWithSlug;

  const markers = useMemo(() => {
    if (!poi) return [];

    return [buildMarker(poi, "highlighted")];
  }, [poi]);

  // const initialView = useMemo(
  //   () =>
  //     poi
  //       ? {
  //           type: "point" as const,
  //           point: poi.point,
  //           zoom: 14,
  //         }
  //       : undefined,
  //   [poi],
  // );

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer
        titleSegments={[
          { name: "Points of Interest", linkTo: "/pois" },
          ...(poi
            ? [
                {
                  name: poi.name,
                  linkTo: `/pois/${params.slug}`,
                },
              ]
            : []),
        ]}
      >
        {loading ? (
          <LoadingSpinnerSidebarContent />
        ) : poi ? (
          <div css={contentContainerCss}>
            <div css={poiNameCss}>{poi.name}</div>
            <div css={poiTypeCss}>{poi.pointOfInterestType.toLowerCase()}</div>
          </div>
        ) : (
          <div css={contentContainerCss}>Point of interest not found</div>
        )}
      </SidebarContainer>
      <MapContainer>
        <PrimaryMap markers={markers} initialView={DEFAULT_INITIAL_VIEW} />
      </MapContainer>
    </Container>
  );
}
