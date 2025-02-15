import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { css } from "@emotion/react";
import { sortBy } from "lodash";
import { gql } from "~/__generated__/gql";

const AllTripsQuery = gql(`
    query AllTrips($username: String!) {
      userWithUsername(username: $username) {
        trips {
          id
          name
          year
          isPublished
        }
      }
    }
  `);

const tripTableContainerCss = css`
  max-height: 67vh;
  overflow: hidden;
  border: 1px solid #ddd;
`;

const tripTableCss = css`
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

interface Trip {
  id: string;
  name: string;
  year: number;
  isPublished: boolean;
}

interface TripListProps {
  username: string;
}

export function TripList({ username }: TripListProps): React.ReactElement {
  const { data, loading } = useQuery(AllTripsQuery, {
    variables: { username },
  });

  const trips: Trip[] = sortBy(data?.userWithUsername?.trips ?? [], [
    (trip): number => -trip.year,
    (trip): string => trip.name,
  ]);

  if (loading) {
    return <div css={loadingStyles}>Loading trips...</div>;
  }

  return (
    <div css={tripTableContainerCss}>
      <table css={tripTableCss}>
        <thead>
          <tr>
            <th>Year</th>
            <th>Name</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          {trips.map((trip) => (
            <tr key={trip.id}>
              <td>{trip.year}</td>
              <td>{trip.name}</td>
              <td>{trip.isPublished ? "Published" : "Draft"}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
