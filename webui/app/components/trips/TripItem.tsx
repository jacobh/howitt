import { css } from "@emotion/react";
import { Link } from "@remix-run/react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { useMemo } from "react";
import { tokens } from "~/styles/tokens";
import { Temporal } from "@js-temporal/polyfill";

export const TripItemFragment = gql(`
        fragment tripItem on Trip {
        id
        name
        year
        slug
        legs {
          rides {
            startedAt
          }
        }
        user {
          username
        }
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

const subtitleContainerCss = css`
  display: flex;
`;

const tripMetaCss = css`
  color: ${tokens.colors.darkGrey};
  font-size: 0.9em;
`;

export function TripItem({ trip: tripFragment }: Props): React.ReactNode {
  const trip = useFragment(TripItemFragment, tripFragment);

  const formattedDate = useMemo(() => {
    const firstRide = trip.legs.at(0)?.rides.at(0);
    if (!firstRide) return null;

    const startTime = Temporal.Instant.from(firstRide.startedAt);
    const date = startTime.toZonedDateTimeISO("Australia/Melbourne");

    return date.toLocaleString("en-US", {
      month: "short",
      year: "numeric",
    });
  }, [trip]);

  return (
    <div className="trip-item" css={tripItemCss}>
      <p className="trip-title" css={tripTitleCss}>
        <Link
          to={`/riders/${trip.user.username}/trips/${trip.year}/${trip.slug}`}
        >
          {trip.name}
        </Link>
      </p>
      <div css={subtitleContainerCss}>
        <div css={tripMetaCss}>
          {formattedDate} • {trip.user.username}
        </div>
      </div>
    </div>
  );
}
