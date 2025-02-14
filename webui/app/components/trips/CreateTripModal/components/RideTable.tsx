import { css } from "@emotion/react";
import { gql } from "~/__generated__";
import { useMemo } from "react";
import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { sortBy } from "lodash";

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

interface Props {
  username: string;
  selectedRideIds: Set<string>;
  onSelectionChange: (rideIds: Set<string>) => void;
}

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
    cursor: pointer;
    transition: background-color 0.2s;

    &:hover {
      background-color: #f8f8f8;
    }
  }
`;

const checkboxCss = css`
  width: 20px;
  height: 20px;
  cursor: pointer;
  pointer-events: none;
`;

const loadingStyles = css`
  padding: 16px;
  color: #666;
  font-style: italic;
`;

export function RideTable({
  username,
  selectedRideIds,
  onSelectionChange,
}: Props): React.ReactElement {
  const { data, loading } = useQuery(AllRidesQuery, {
    variables: {
      username,
    },
  });

  const handleToggleRide = (rideId: string): void => {
    const newSelection = new Set(selectedRideIds);
    if (newSelection.has(rideId)) {
      newSelection.delete(rideId);
    } else {
      newSelection.add(rideId);
    }
    onSelectionChange(newSelection);
  };

  const rides = useMemo(() => {
    const unsortedRides = data?.userWithUsername?.rides ?? [];
    return sortBy(unsortedRides, (ride) => ride.startedAt).reverse();
  }, [data]);

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
            <th>Include?</th>
          </tr>
        </thead>
        <tbody>
          {rides.map((ride) => (
            <tr key={ride.id} onClick={(): void => handleToggleRide(ride.id)}>
              <td>{new Date(ride.startedAt).toLocaleString()}</td>
              <td>{ride.name}</td>
              <td>{(ride.distance / 1000).toFixed(1)}km</td>
              <td>
                <input
                  type="checkbox"
                  css={checkboxCss}
                  checked={selectedRideIds.has(ride.id)}
                  readOnly
                />
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
