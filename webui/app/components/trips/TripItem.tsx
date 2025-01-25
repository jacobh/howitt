import { css } from "@emotion/react";
import { Link } from "@remix-run/react";
import { FragmentType, gql, useFragment } from "~/__generated__";

export const TripItemFragment = gql(`
    fragment tripItem on Trip {
        id
        name
    }
`);

interface Props {
  trip: FragmentType<typeof TripItemFragment>;
}

const tripItemCss = css`
  container-type: inline-size;
`;

const tripTitleCss = css({
  marginBottom: "6px",
});

export function TripItem({ trip: tripFragment }: Props): React.ReactNode {
  const trip = useFragment(TripItemFragment, tripFragment);

  return (
    <div className="trip-item" css={tripItemCss}>
      <p className="trip-title" css={tripTitleCss}>
        <Link to={`/trips/${trip.id}`}>{trip.name}</Link>
      </p>
    </div>
  );
}
