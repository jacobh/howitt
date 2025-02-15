import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { css } from "@emotion/react";
import { sortBy } from "lodash";
import { gql } from "~/__generated__";

const AllRidesQuery = gql(`
    query AllRides($username: String!) {
      userWithUsername(username: $username) {
        rides {
          id
          name
          startedAt
          finishedAt
          distance
        }
      }
    }
  `);

const rideTableContainerCss = css`
  max-height: 67vh;
  overflow: hidden;
  border: 1px solid #ddd;
`;

const rideTableCss = css`
  width: 100%;
  border-collapse: separate;
  border-spacing: 0;

  th,
  td {
    padding: 8px;
    text-align: left;
    border-bottom: 1px solid #ddd;
  }

  th {
    background-color: #f5f5f5;
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
    background-color: #f8f8f8;
  }
`;

const loadingStyles = css`
  padding: 16px;
  color: #666;
  font-style: italic;
`;

interface Ride {
  id: string;
  name: string;
  startedAt: string;
  finishedAt: string;
  distance: number;
}

interface RideListProps {
  username: string;
}

export function RideList({ username }: RideListProps): React.ReactElement {
  const { data, loading } = useQuery(AllRidesQuery, {
    variables: { username },
  });

  const rides: Ride[] = sortBy(
    data?.userWithUsername?.rides ?? [],
    (ride) => ride.startedAt,
  ).reverse();

  if (loading) {
    return <div css={loadingStyles}>Loading rides...</div>;
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
              <td>{new Date(ride.startedAt).toLocaleString()}</td>
              <td>{ride.name}</td>
              <td>{(ride.distance / 1000).toFixed(1)}km</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
