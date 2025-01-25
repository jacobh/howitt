import { NearbyRoute, Route, Terminus } from "~/__generated__/graphql";
import { formatDistance } from "~/services/format";
import { CardinalSubset, cardinalFromDegree } from "cardinal-direction";
import { sortBy } from "lodash";
import { RouteItem } from "~/components/routes/RouteItem";
import { css } from "@emotion/react";
import { FragmentType, gql, useFragment } from "~/__generated__";

export const NearbyRoutesFragment = gql(`
  fragment nearbyRoutesInfo on Terminus {
    bearing
    nearbyRoutes {
      delta {
        distance
        bearing
      }
      closestTerminus {
        bearing
        route {
          id
          ...routeItem
        }
      }
    }
  }
`);

interface Props {
  terminus: FragmentType<typeof NearbyRoutesFragment>;
}

const routeItemContainerCss = css`
  margin: 24px 0;

  &:first-child {
    margin-top: 18px;
  }
`;

export function NearbyRoutes({
  terminus: terminusFragment,
}: Props): React.ReactNode {
  const terminus = useFragment(NearbyRoutesFragment, terminusFragment);
  const { nearbyRoutes } = terminus;

  if (terminus.nearbyRoutes.length === 0) {
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
          ),
        )}
      </div>
    </div>
  );
}
