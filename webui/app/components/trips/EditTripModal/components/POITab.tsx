import { FragmentType, gql, useFragment } from "~/__generated__";

export const TripPoisFragment = gql(`
  fragment tripPois on Trip {
    id
    user {
        username
    }
  }
`);

type Props = {
  trip: FragmentType<typeof TripPoisFragment>;
};

export function POITab({ trip: tripFragment }: Props): React.ReactElement {
  const trip = useFragment(TripPoisFragment, tripFragment);

  return <p>Coming Soon</p>;
}
