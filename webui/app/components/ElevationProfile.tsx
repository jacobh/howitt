import { css } from "@emotion/react";
import { useMemo } from "react";
import { Area, AreaChart, ResponsiveContainer, XAxis, YAxis } from "recharts";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { zipStrict } from "~/services/zipStrict";

export const ElevationPathFragment = gql(`
  fragment elevationPath on ElevationPath {
    elevationPoints
    distancePoints
  }
`);

interface Props {
  data: FragmentType<typeof ElevationPathFragment>;
}

interface DataPoint {
  elevation: number;
  distance: number;
}

function computePoints(
  elevationPoints: number[],
  distancePoints: number[]
): DataPoint[] {
  return zipStrict(elevationPoints, distancePoints).map(
    ([elevation, distance]) => ({ elevation, distance })
  );
}

const elevationProfileWrapperCss = css`
  height: 150px;
`;

export function ElevationProfile({
  data: dataFragment,
}: Props): React.ReactElement {
  const data = useFragment(ElevationPathFragment, dataFragment);

  const points = useMemo(
    () => computePoints(data.elevationPoints, data.distancePoints),
    [data]
  );

  return (
    <div css={elevationProfileWrapperCss}>
      <ResponsiveContainer>
        <AreaChart data={points}>
          <XAxis
            dataKey="distance"
            minTickGap={75}
            tickFormatter={(tick): string =>
              `${Math.round((tick / 1000) * 10) / 10}km`
            }
          />
          <YAxis
            domain={["dataMin", "dataMax"]}
            minTickGap={30}
            scale="linear"
            tickFormatter={(tick): string => `${Math.round(tick / 10) * 10}m`}
          />
          <Area
            dataKey="elevation"
            stroke="#8884d8"
            fill="#8884d8"
            isAnimationActive={false}
          />
        </AreaChart>
      </ResponsiveContainer>
    </div>
  );
}
