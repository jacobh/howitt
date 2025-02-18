import { Link } from "@remix-run/react";
import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { css } from "@emotion/react";
import { sortBy } from "lodash";
import { gql } from "~/__generated__/gql";
import { LoadingSpinnerSidebarContent } from "../ui/LoadingSpinner";
import { tokens } from "~/styles/tokens";

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

const routeTableContainerCss = css`
  max-height: 67vh;
  overflow: hidden;
  border: 1px solid ${tokens.colors.grey200};
`;

const routeTableCss = css`
  width: 100%;
  border-collapse: separate;
  border-spacing: 0;

  th,
  td {
    padding: 10px 10px;
    text-align: left;
    border-bottom: 1px solid ${tokens.colors.grey200};
  }

  th {
    background-color: ${tokens.colors.grey50};
    font-weight: 500;
    position: sticky;
    top: 0;
    z-index: 1;
  }

  tbody {
    display: block;
    overflow-y: auto;
    max-height: calc(67vh - 41px);
  }

  thead,
  tbody tr {
    display: table;
    width: 100%;
    table-layout: fixed;
  }

  tbody tr {
    transition: background-color 0.2s;
  }

  tbody tr:hover {
    background-color: ${tokens.colors.grey50};
  }
`;

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
    <div css={routeTableContainerCss}>
      <table css={routeTableCss}>
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
