import { css } from "@emotion/react";
import { gql } from "~/__generated__";
import { useMemo } from "react";
import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { sortBy } from "lodash";
import { tableContainerCss, tableCss } from "~/components/ui/Table";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";

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

const checkboxCss = css`
  width: 20px;
  height: 20px;
  cursor: pointer;
  pointer-events: none;
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
    return <LoadingSpinnerSidebarContent />;
  }

  return (
    <div css={tableContainerCss}>
      <table css={tableCss}>
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
