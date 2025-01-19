import { NearbyRoute, Route, Terminus } from "~/__generated__/graphql";
import { formatDistance } from "~/services/format";
import { CardinalSubset, cardinalFromDegree } from "cardinal-direction";
import { sortBy } from "lodash";
import { RouteItem } from "~/components/routes/RouteItem";
import { css } from "@emotion/react";

interface Props {
  terminus?: Pick<Terminus, "bearing">;
  nearbyRoutes: (Pick<NearbyRoute, "delta"> & {
    closestTerminus: Pick<Terminus, "bearing"> & {
      route: Pick<
        Route,
        "id" | "name" | "distance" | "elevationAscentM" | "elevationDescentM"
      >;
    };
  })[];
}

const routeItemContainerCss = css`
  margin: 24px 0;

  &:first-child {
    margin-top: 18px;
  }
`;

export function NearbyRoutes({
  terminus,
  nearbyRoutes,
}: Props): React.ReactNode {
  if (nearbyRoutes.length === 0) {
    return null;
  }

  return (
    <div>
      <p>
        Nearby Routes
        {terminus &&
          ` (${cardinalFromDegree(terminus.bearing, CardinalSubset.Ordinal)})`}
      </p>
      <div>
        {sortBy(nearbyRoutes, ({ delta }) => delta.distance).map(
          ({ delta, closestTerminus: { route } }) => (
            <div key={route.id} css={routeItemContainerCss}>
              <RouteItem
                titlePostfix={`(${[
                  formatDistance(delta.distance),
                  cardinalFromDegree(delta.bearing, CardinalSubset.Ordinal),
                ].join(" ")})`}
                route={route}
              />
            </div>
          )
        )}
      </div>
    </div>
  );
}
