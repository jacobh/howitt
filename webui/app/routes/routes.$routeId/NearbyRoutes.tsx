import { Link } from "@remix-run/react";
import { NearbyRoute, Route, Terminus } from "~/__generated__/graphql";
import { formatDistance } from "~/services/formatDistance";
import { CardinalSubset, cardinalFromDegree } from "cardinal-direction";

interface Props {
  terminus: Pick<Terminus, "direction">;
  nearbyRoutes: (Pick<NearbyRoute, "delta"> & {
    terminus: Pick<Terminus, "direction"> & { route: Pick<Route, "id" | "name"> };
  })[];
}

export function NearbyRoutes({
  terminus,
  nearbyRoutes,
}: Props): React.ReactElement {
  return (
    <div>
      <h3>Nearby Routes ({terminus.direction[0]})</h3>
      {nearbyRoutes.map(({ delta, terminus: {direction, route} }) => <div key={route.id}>
              <h3>
                <Link to={`/routes/${route.id.split("#")[1]}`}>
                  {route.name} ({direction[0]})
                </Link>
              </h3>
              <p>{formatDistance(delta.distance)} {cardinalFromDegree(delta.bearing, CardinalSubset.Ordinal)}</p>
            </div>)}
    </div>
  );
}
