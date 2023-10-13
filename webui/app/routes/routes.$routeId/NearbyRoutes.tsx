import { Link } from "@remix-run/react";
import { NearbyRoute, Route, Terminus } from "~/__generated__/graphql";
import { formatDistance } from "~/services/format";
import { CardinalSubset, cardinalFromDegree } from "cardinal-direction";

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
      <h3>
        Nearby Routes (
        {cardinalFromDegree(terminus.bearing, CardinalSubset.Ordinal)})
      </h3>
      {nearbyRoutes.map(({ delta, closestTerminus: { bearing, route } }) => (
        <div key={route.id}>
          <h3>
            <Link to={`/routes/${route.id.split("#")[1]}`}>
              {route.name} (
              {cardinalFromDegree(bearing, CardinalSubset.Ordinal)})
            </Link>
          </h3>
          <p>
            {formatDistance(delta.distance)}{" "}
            {cardinalFromDegree(delta.bearing, CardinalSubset.Ordinal)}
          </p>
        </div>
      ))}
    </div>
  );
}
