import { Link } from "@remix-run/react";
import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { sortBy } from "lodash";
import { gql } from "~/__generated__/gql";
import { LoadingSpinnerSidebarContent } from "../ui/LoadingSpinner";
import { tableContainerCss, tableCss } from "../ui/Table";

const AllRoutesQuery = gql(`
    query AllRoutes($username: String!) {
      userWithUsername(username: $username) {
        routes {
          id
          name
          slug
          distance
          elevationAscentM
          elevationDescentM
        }
      }
    }
  `);

interface RouteListProps {
  username: string;
}

export function RouteList({ username }: RouteListProps): React.ReactElement {
  const { data, loading } = useQuery(AllRoutesQuery, {
    variables: { username },
  });

  const routes = sortBy(
    data?.userWithUsername?.routes ?? [],
    (route) => route.name,
  );

  if (loading) {
    return <LoadingSpinnerSidebarContent />;
  }

  return (
    <div css={tableContainerCss}>
      <table css={tableCss}>
        <thead>
          <tr>
            <th>Name</th>
            <th>Distance</th>
            <th>Elevation Gain</th>
            <th>Elevation Loss</th>
          </tr>
        </thead>
        <tbody>
          {routes.map((route) => (
            <tr key={route.id}>
              <td>
                <Link to={`/routes/${route.slug}`}>{route.name}</Link>
              </td>
              <td>{(route.distance / 1000).toFixed(1)}km</td>
              <td>{route.elevationAscentM.toFixed(0)}m</td>
              <td>{route.elevationDescentM.toFixed(0)}m</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
