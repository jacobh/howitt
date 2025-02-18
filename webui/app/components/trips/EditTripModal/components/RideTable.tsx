import { css } from "@emotion/react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { useState, useMemo } from "react";
import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { useMutation } from "@apollo/client/react/hooks/useMutation";
import { sortBy } from "lodash";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";
import { tableContainerCss, tableCss } from "~/components/ui/Table";

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

const UpdateTripRidesMutation = gql(`
  mutation UpdateTripRides($input: UpdateTripRidesInput!) {
    updateTripRides(input: $input) {
      trip {
        id
        rides {
          id
        }
      }
    }
  }
`);

interface Props {
  trip: FragmentType<typeof TripRidesFragment>;
  refetch: () => void;
}

const checkboxCss = css`
  width: 20px;
  height: 20px;
  cursor: pointer;
  pointer-events: none;
`;

export function RideTable({
  trip: tripFragment,
  refetch,
}: Props): React.ReactElement {
  const trip = useFragment(TripRidesFragment, tripFragment);

  const [includedRideIds, setIncludedRideIds] = useState<Set<string>>(
    new Set(trip.rides.map((ride) => ride.id)),
  );

  const { data: allRidesData, loading } = useQuery(AllRidesQuery, {
    variables: {
      username: trip.user.username,
    },
  });

  const [updateRides, { loading: updatingRides }] = useMutation(
    UpdateTripRidesMutation,
    {
      onCompleted: () => {
        refetch();
      },
    },
  );

  const handleToggleRide = (rideId: string): void => {
    setIncludedRideIds((prev) => {
      const next = new Set(prev);
      if (next.has(rideId)) {
        next.delete(rideId);
      } else {
        next.add(rideId);
      }

      // Call mutation with updated IDs
      updateRides({
        variables: {
          input: {
            tripId: trip.id,
            rideIds: Array.from(next),
          },
        },
      });

      return next;
    });
  };

  const rides = useMemo(() => {
    const unsortedRides = allRidesData?.userWithUsername?.rides ?? [];
    return sortBy(unsortedRides, (ride) => ride.startedAt).reverse();
  }, [allRidesData]);

  if (loading || updatingRides) {
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
