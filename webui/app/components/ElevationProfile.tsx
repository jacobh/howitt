import { css } from "@emotion/react";
import { maxBy, minBy, sortBy } from "lodash";
import { useMemo } from "react";
import { Area, AreaChart, ResponsiveContainer, XAxis, YAxis } from "recharts";
import { isNotNil } from "~/services/isNotNil";

interface Props {
  elevationPoints: number[];
  distancePoints: number[];
}

interface DataPoint {
  elevation: number;
  distance: number;
}

function zipStrict<T, U>(items1: T[], items2: U[]): [T, U][] {
  if (items1.length !== items2.length) {
    throw new Error("items must have same length");
  }

  return items1.map((x, i) => [x, items2[i]]);
}

function computeData({ elevationPoints, distancePoints }: Props): DataPoint[] {
  return zipStrict(elevationPoints, distancePoints).map(
    ([elevation, distance]) => ({ elevation, distance })
  );
}

const elevationProfileWrapperCss = css`
  height: 150px;
`;

export function ElevationProfile(props: Props): React.ReactElement {
  const data = useMemo(() => computeData(props), [props]);

  const minElevationAt = minBy(data, ({ elevation }) => elevation)?.distance;

  const maxElevationAt = maxBy(data, ({ elevation }) => elevation)?.distance;

  return (
    <div css={elevationProfileWrapperCss}>
      <ResponsiveContainer>
        <AreaChart data={data}>
          <XAxis
            dataKey="distance"
            // minTickGap={50}
            ticks={sortBy(
              [0, minElevationAt, maxElevationAt, data.at(-1)?.distance].filter(
                isNotNil
              ),
              (x) => x
            )}
            tickFormatter={(tick): string => {
              const formattedDistance = `${
                Math.round((tick / 1000) * 10) / 10
              }km`;

              const point = data.find((p) => p.distance == tick);

              if (!point) {
                return formattedDistance;
              }

              const isFirst = point.distance === 0;
              const isLast = point.distance === data.at(-1)?.distance;
              const isMaxElevation = point.distance === maxElevationAt;
              const isMinElevation = point.distance === minElevationAt;

              const arrow = [
                isFirst ? `←` : undefined,
                isLast ? `→` : undefined,
                isMinElevation ? `↓` : undefined,
                isMaxElevation ? `↑` : undefined,
              ]
                .filter(isNotNil)
                .at(0);

              const formattedElevation = `${arrow} ${Math.round(
                point.elevation
              )}m`;

              return [formattedDistance, formattedElevation]
                .filter(isNotNil)
                .join(" ");
            }}
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
