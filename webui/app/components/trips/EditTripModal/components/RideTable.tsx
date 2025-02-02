import { css } from "@emotion/react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { useState } from "react";

export const TripRidesFragment = gql(`
  fragment tripRides on Trip {
    id
    rides {
      id
      name
      startedAt
      finishedAt
      distance
    }
  }
`);

interface Props {
  trip: FragmentType<typeof TripRidesFragment>;
}
const rideTableCss = css`
  width: 100%;
  border-collapse: collapse;

  th,
  td {
    padding: 8px;
    text-align: left;
    border-bottom: 1px solid #ddd;
  }

  th {
    background-color: #f5f5f5;
    font-weight: 500;
  }
`;

const checkboxCss = css`
  width: 20px;
  height: 20px;
  cursor: pointer;
`;

export function RideTable({ trip: tripFragment }: Props): React.ReactElement {
  const trip = useFragment(TripRidesFragment, tripFragment);

  const [includedRideIds, setIncludedRideIds] = useState<Set<string>>(
    new Set(trip.rides.map((ride) => ride.id)),
  );

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

  return (
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
        {trip.rides.map((ride) => (
          <tr key={ride.id}>
            <td>{new Date(ride.startedAt).toLocaleString()}</td>
            <td>{ride.name}</td>
            <td>{(ride.distance / 1000).toFixed(1)}km</td>
            <td>
              <input
                type="checkbox"
                css={checkboxCss}
                checked={includedRideIds.has(ride.id)}
                onChange={(): void => handleToggleRide(ride.id)}
              />
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
