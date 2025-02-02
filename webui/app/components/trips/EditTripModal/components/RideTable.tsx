import { css } from "@emotion/react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { useState, useMemo } from "react";
import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { sortBy } from "lodash";

export const TripRidesFragment = gql(`
  fragment tripRides on Trip {
    id
    user {
        username
    }
    rides {
      id
      name
      startedAt
      finishedAt
      distance
    }
  }
`);

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
  trip: FragmentType<typeof TripRidesFragment>;
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
    max-height: calc(67vh - 41px); /* 41px accounts for header height */
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

export function RideTable({ trip: tripFragment }: Props): React.ReactElement {
  const trip = useFragment(TripRidesFragment, tripFragment);

  const [includedRideIds, setIncludedRideIds] = useState<Set<string>>(
    new Set(trip.rides.map((ride) => ride.id)),
  );

  const { data: allRidesData, loading } = useQuery(AllRidesQuery, {
    variables: {
      username: trip.user.username,
    },
  });

  const handleToggleRide = (rideId: string): void => {
    setIncludedRideIds((prev) => {
      const next = new Set(prev);
      if (next.has(rideId)) {
        next.delete(rideId);
      } else {
        next.add(rideId);
      }
      return next;
    });
  };

  const rides = useMemo(() => {
    const unsortedRides = allRidesData?.userWithUsername?.rides ?? [];
    return sortBy(unsortedRides, (ride) => ride.startedAt).reverse();
  }, [allRidesData]);

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
                  checked={includedRideIds.has(ride.id)}
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
