import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { sortBy } from "lodash";
import { gql } from "~/__generated__/gql";
import { Link } from "@remix-run/react";
import { LoadingSpinnerSidebarContent } from "../ui/LoadingSpinner";
import { tableContainerCss, tableCss } from "./Table";

const AllTripsQuery = gql(`
    query AllTrips($username: String!) {
      userWithUsername(username: $username) {
        trips {
          id
          name
          year
          isPublished
          slug
        }
      }
    }
  `);

interface TripListProps {
  username: string;
}

export function TripList({ username }: TripListProps): React.ReactElement {
  const { data, loading } = useQuery(AllTripsQuery, {
    variables: { username },
  });

  const trips = sortBy(data?.userWithUsername?.trips ?? [], [
    (trip): number => -trip.year,
    (trip): string => trip.name,
  ]);

  if (loading) {
    return <LoadingSpinnerSidebarContent />;
  }

  return (
    <div css={tableContainerCss}>
      <table css={tableCss}>
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
              <td>
                <Link
                  to={`/riders/${username}/trips/${trip.year}/${trip.slug}`}
                >
                  {trip.name}
                </Link>
              </td>
              <td>{trip.isPublished ? "Published" : "Draft"}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
