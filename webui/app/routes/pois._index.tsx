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
import { PrimaryMap } from "~/components/map/PrimaryMap";
import { DEFAULT_INITIAL_VIEW } from "~/components/map";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";
import { useMemo, useState } from "react";
import { buildMarker } from "~/components/map/types";
import { FragmentType, useFragment } from "~/__generated__";
import { Link } from "@remix-run/react";

const POIsQuery = gql(`
  query POIsQuery {
    pointsOfInterest {
      id
      name
      point
      pointOfInterestType
      ...poiItem
    }
    viewer {
      ...viewerInfo
    }
  }
`);

const poiItemContainerCss = css`
  padding: 20px 1.5%;
  border-bottom: 1px solid ${tokens.colors.grey100};

  &:hover {
    background-color: ${tokens.colors.grey100};
  }
`;

const poiNameCss = css`
  font-size: 1.25rem;
  line-height: 1.75rem;
  font-weight: 500;
`;

const poiTypeCss = css`
  color: ${tokens.colors.grey500};
  font-size: 0.875rem;
  line-height: 1.25rem;
`;

const POIItemFragment = gql(`
    fragment poiItem on PointOfInterest {
      id
      name
      point
      slug
      pointOfInterestType
    }
  `);

function POIItem({
  poi: poiFragment,
}: {
  poi: FragmentType<typeof POIItemFragment>;
}): React.ReactElement {
  const poi = useFragment(POIItemFragment, poiFragment);

  return (
    <div>
      <div css={poiNameCss}>
        <Link to={`/pois/${poi.slug}`}>{poi.name}</Link>
      </div>
      <div css={poiTypeCss}>{poi.pointOfInterestType.toLowerCase()}</div>
    </div>
  );
}

export default function POIs(): React.ReactElement {
  const { data, loading } = useQuery(POIsQuery);
  const [hoveredPoiId, setHoveredPoiId] = useState<string | undefined>(
    undefined,
  );

  const markers = useMemo(() => {
    const pois = data?.pointsOfInterest ?? [];
    return pois.map((poi) =>
      buildMarker(poi, hoveredPoiId === poi.id ? "highlighted" : "default"),
    );
  }, [data?.pointsOfInterest, hoveredPoiId]);

  const pois = useMemo(
    () => data?.pointsOfInterest ?? [],
    [data?.pointsOfInterest],
  );

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer
        titleSegments={[{ name: "Points of Interest", linkTo: "/pois" }]}
      >
        {loading ? (
          <LoadingSpinnerSidebarContent />
        ) : (
          <>
            {pois.map((poi) => (
              <div
                key={poi.id}
                css={poiItemContainerCss}
                onMouseEnter={(): void => setHoveredPoiId(poi.id)}
                onMouseLeave={(): void => setHoveredPoiId(undefined)}
              >
                <POIItem poi={poi} />
              </div>
            ))}
          </>
        )}
      </SidebarContainer>
      <MapContainer>
        <PrimaryMap markers={markers} initialView={DEFAULT_INITIAL_VIEW} />
      </MapContainer>
    </Container>
  );
}
