import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { Link } from "@remix-run/react";
import { sortBy } from "lodash";
import { gql } from "~/__generated__";
import { LoadingSpinnerSidebarContent } from "../ui/LoadingSpinner";
import { tableContainerCss, tableCss } from "../ui/Table";

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
    <div css={tableContainerCss}>
      <table css={tableCss}>
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
