import { Temporal } from "@js-temporal/polyfill";
import { css } from "@emotion/react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { DataTable } from "../DataTable";
import { formatDistance, formatDuration } from "~/services/format";

export const RideSummaryFragment = gql(`
  fragment rideSummary on Ride {
    id
    name
    distance
    startedAt
    finishedAt
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

  const startTime = Temporal.Instant.from(ride.startedAt);
  const endTime = Temporal.Instant.from(ride.finishedAt);
  const duration = startTime.until(endTime);
  const distanceKm = ride.distance / 1000;
  const averageSpeed = distanceKm / duration.total("hours");

  const items = [
    {
      name: "Start Time",
      value: startTime.toLocaleString(undefined, {
        hour: "numeric",
        minute: "numeric",
      }),
    },
    {
      name: "End Time",
      value: endTime.toLocaleString(undefined, {
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

  return (
    <div css={rideSummaryCss}>
      <DataTable title={ride.name} items={items} />
    </div>
  );
}
