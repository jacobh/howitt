import { Link } from "@remix-run/react";
import { NearbyRoute, Route, Terminus } from "~/__generated__/graphql";
import { formatDistance } from "~/services/format";
import { CardinalSubset, cardinalFromDegree } from "cardinal-direction";
import { COLORS } from "~/styles/theme";
import { sortBy } from "lodash";

interface Props {
  terminus: Pick<Terminus, "bearing">;
  nearbyRoutes: (Pick<NearbyRoute, "delta"> & {
    closestTerminus: Pick<Terminus, "bearing"> & {
      route: Pick<Route, "id" | "name">;
    };
  })[];
}

export function NearbyRoutes({
  terminus,
  nearbyRoutes,
}: Props): React.ReactElement {
  return (
    <div>
      <p css={{ margin: "10px 0" }}>
        Nearby Routes (
        {cardinalFromDegree(terminus.bearing, CardinalSubset.Ordinal)})
      </p>
      {sortBy(nearbyRoutes, ({ delta }) => delta.distance).map(
        ({ delta, closestTerminus: { route } }) => (
          <div key={route.id} css={{ margin: "10px 0" }}>
            <p>
              <Link to={`/routes/${route.id.split("#")[1]}`}>{route.name}</Link>
            </p>
            <p css={{ color: COLORS.darkGrey }}>
              {formatDistance(delta.distance)}{" "}
              {cardinalFromDegree(delta.bearing, CardinalSubset.Ordinal)}
            </p>
          </div>
        )
      )}
    </div>
  );
}
