import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { sortBy } from "lodash";
import { gql } from "~/__generated__/gql";
import { Link } from "@remix-run/react";
import { LoadingSpinnerSidebarContent } from "../ui/LoadingSpinner";
import { tableContainerCss, tableCss } from "../ui/Table";

const AllPOIsQuery = gql(`
  query AllPOIs($username: String!) {
    userWithUsername(username: $username) {
      pointsOfInterest {
        id
        name
        slug
        pointOfInterestType
      }
    }
  }
`);

interface POIListProps {
  username: string;
}

export function POIList({ username }: POIListProps): React.ReactElement {
  const { data, loading } = useQuery(AllPOIsQuery, {
    variables: { username },
  });

  const pois = sortBy(
    data?.userWithUsername?.pointsOfInterest ?? [],
    (poi) => poi.name,
  );

  if (loading) {
    return <LoadingSpinnerSidebarContent />;
  }

  return (
    <div css={tableContainerCss}>
      <table css={tableCss}>
        <thead>
          <tr>
            <th>Name</th>
            <th>Type</th>
          </tr>
        </thead>
        <tbody>
          {pois.map((poi) => (
            <tr key={poi.id}>
              <td>
                <Link to={`/pois/${poi.slug}`}>{poi.name}</Link>
              </td>
              <td>{poi.pointOfInterestType}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
