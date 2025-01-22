import { css } from "@emotion/react";
import { Link } from "@remix-run/react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { formatDistance, formatDuration } from "~/services/format";
import { Temporal } from "@js-temporal/polyfill";

export const RideItemFragment = gql(`
    fragment rideItem on Ride {
        id
        date
        distance
        startedAt
        finishedAt
        user {
            username
        }
    }
`);

interface Props {
  ride: FragmentType<typeof RideItemFragment>;
}

const rideItemCss = css`
  container-type: inline-size;
`;

const rideTitleCss = css({
  marginBottom: "6px",
});

const subtitleContainerCss = css`
  display: flex;
`;

const rideVitalsCss = css`
  flex: 1 1 auto;
`;

export function RideItem({ ride: rideFragment }: Props): React.ReactNode {
  const ride = useFragment(RideItemFragment, rideFragment);

  const startTime = Temporal.Instant.from(ride.startedAt);
  const endTime = Temporal.Instant.from(ride.finishedAt);
  const duration = startTime.until(endTime);

  const date = Temporal.PlainDate.from(ride.date);
  const formattedDate = date.toLocaleString("en-US", {
    day: "numeric",
    month: "short",
    year: "numeric",
  });

  return (
    <div className="ride-item" css={rideItemCss}>
      <p className="ride-title" css={rideTitleCss}>
        <Link to={`/riders/${ride.user.username}/${ride.date}/`}>
          {formattedDate}
        </Link>
      </p>
      <div css={subtitleContainerCss}>
        <div css={rideVitalsCss}>
          {formatDistance(ride.distance)} • {formatDuration(duration)}
        </div>
      </div>
    </div>
  );
}
