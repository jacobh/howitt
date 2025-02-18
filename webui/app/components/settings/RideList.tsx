import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { css } from "@emotion/react";
import { Link } from "@remix-run/react";
import { sortBy } from "lodash";
import { gql } from "~/__generated__";
import { LoadingSpinnerSidebarContent } from "../ui/LoadingSpinner";
import { tokens } from "~/styles/tokens";

const AllRidesQuery = gql(`
  query SettingsRideList($username: String!) {
    userWithUsername(username: $username) {
      rides {
        id
        name
        startedAt
        finishedAt
        distance
        date
      }
    }
  }
`);

const rideTableContainerCss = css`
  max-height: 67vh;
  overflow: hidden;
  border: 1px solid ${tokens.colors.grey200};
`;

const rideTableCss = css`
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

interface RideListProps {
  username: string;
}

export function RideList({ username }: RideListProps): React.ReactElement {
  const { data, loading } = useQuery(AllRidesQuery, {
    variables: { username },
  });

  const rides = sortBy(
    data?.userWithUsername?.rides ?? [],
    (ride) => ride.startedAt,
  ).reverse();

  if (loading) {
    return <LoadingSpinnerSidebarContent />;
  }

  return (
    <div css={rideTableContainerCss}>
      <table css={rideTableCss}>
        <thead>
          <tr>
            <th>Started At</th>
            <th>Name</th>
            <th>Distance</th>
          </tr>
        </thead>
        <tbody>
          {rides.map((ride) => (
            <tr key={ride.id}>
              <td>
                <Link to={`/riders/${username}/${ride.date}/`}>
                  {new Date(ride.startedAt).toLocaleString()}
                </Link>
              </td>
              <td>
                <Link to={`/riders/${username}/${ride.date}/`}>
                  {ride.name}
                </Link>
              </td>
              <td>{(ride.distance / 1000).toFixed(1)}km</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
