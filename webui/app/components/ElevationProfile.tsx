import { css } from "@emotion/react";
import { useCallback, useMemo } from "react";
import { Area, AreaChart, ResponsiveContainer, XAxis, YAxis } from "recharts";
import type { MouseHandlerDataParam } from "recharts";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { zipStrict } from "~/services/zipStrict";
import { tokens } from "~/styles/tokens";

export const ElevationPathFragment = gql(`
  fragment elevationPath on ElevationPath {
    elevationPointsJson
    distancePointsJson
  }
`);

interface Props {
  data: FragmentType<typeof ElevationPathFragment>;
  onPointHover?: (pointIndex: number | undefined) => void;
}

interface DataPoint {
  elevation: number;
  distance: number;
}

function computePoints(
  elevationPoints: number[],
  distancePoints: number[],
): DataPoint[] {
  return zipStrict(elevationPoints, distancePoints).map(
    ([elevation, distance]) => ({ elevation, distance }),
  );
}

const elevationProfileWrapperCss = css`
  height: 150px;
`;

export function ElevationProfile({
  data: dataFragment,
  onPointHover,
}: Props): React.ReactElement {
  const data = useFragment(ElevationPathFragment, dataFragment);

  const points = useMemo(
    () =>
      computePoints(
        JSON.parse(data.elevationPointsJson),
        JSON.parse(data.distancePointsJson),
      ),
    [data],
  );

  const formatDistanceTick = useCallback((tick: number): string => {
    return `${Math.round((tick / 1000) * 10) / 10}km`;
  }, []);

  const formatElevationTick = useCallback((tick: number): string => {
    return `${Math.round(tick / 10) * 10}m`;
  }, []);

  const handleMouseMove = useCallback(
    ({ activeTooltipIndex }: MouseHandlerDataParam) => {
      if (activeTooltipIndex === null || activeTooltipIndex === undefined) {
        onPointHover?.(undefined);
        return;
      }

      const pointIndex = Number(activeTooltipIndex);
      onPointHover?.(Number.isInteger(pointIndex) ? pointIndex : undefined);
    },
    [onPointHover],
  );

  const handleMouseLeave = useCallback(() => {
    onPointHover?.(undefined);
  }, [onPointHover]);

  return (
    <div css={elevationProfileWrapperCss}>
      <ResponsiveContainer>
        <AreaChart
          data={points}
          onMouseMove={handleMouseMove}
          onMouseLeave={handleMouseLeave}
        >
          <XAxis
            dataKey="distance"
            minTickGap={75}
            tickFormatter={formatDistanceTick}
          />
          <YAxis
            domain={["dataMin", "dataMax"]}
            minTickGap={30}
            scale="linear"
            tickFormatter={formatElevationTick}
          />
          <Area
            dataKey="elevation"
            stroke={tokens.colors.purple600}
            fill={tokens.colors.purple500}
            isAnimationActive={false}
          />
        </AreaChart>
      </ResponsiveContainer>
    </div>
  );
}
