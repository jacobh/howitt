import { Temporal } from "@js-temporal/polyfill";
import { css } from "@emotion/react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { DataTable } from "../DataTable";
import { formatDistance, formatDuration } from "~/services/format";
import { useMemo } from "react";

export const RideSummaryFragment = gql(`
  fragment rideSummary on Ride {
    id
    name
    distance
    startedAt
    finishedAt
    tz
  }
`);

interface Props {
  ride: FragmentType<typeof RideSummaryFragment>;
}

const rideSummaryCss = css`
  margin-bottom: 16px;
`;

export function RideSummary({ ride: rideFragment }: Props): React.ReactNode {
  const ride = useFragment(RideSummaryFragment, rideFragment);

  const items = useMemo(() => {
    const timeZone = Temporal.TimeZone.from(ride.tz ?? "Australia/Melbourne");

    const startTime = Temporal.Instant.from(ride.startedAt).toZonedDateTime({
      timeZone,
      calendar: "iso8601",
    });

    const endTime = Temporal.Instant.from(ride.finishedAt).toZonedDateTime({
      timeZone,
      calendar: "iso8601",
    });

    const duration = startTime.until(endTime);
    const distanceKm = ride.distance / 1000;
    const averageSpeed = distanceKm / duration.total("hours");

    return [
      {
        name: "Start Time",
        value: startTime.toLocaleString("en-US", {
          hour: "numeric",
          minute: "numeric",
        }),
      },
      {
        name: "End Time",
        value: endTime.toLocaleString("en-US", {
          hour: "numeric",
          minute: "numeric",
        }),
      },
      {
        name: "Duration",
        value: formatDuration(duration),
      },
      {
        name: "Distance",
        value: formatDistance(ride.distance),
      },
      {
        name: "Average Speed",
        value: `${averageSpeed.toFixed(1)} km/h`,
      },
    ];
  }, [ride.tz, ride.startedAt, ride.finishedAt, ride.distance]);

  return (
    <div css={rideSummaryCss}>
      <DataTable title={ride.name} items={items} />
    </div>
  );
}
