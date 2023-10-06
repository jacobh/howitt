import { last, range } from "lodash";
import { useMemo } from "react";
import { Area, AreaChart, XAxis, YAxis } from "recharts";

interface Props {
  elevationPoints: number[];
  distancePoints: number[];
}

interface DataPoint {
  elevation: number;
  distanceFromStart: number;
}

function zipStrict<T, U>(items1: T[], items2: U[]): [T, U][] {
  if (items1.length !== items2.length) {
    throw new Error("items must have same length");
  }

  return items1.map((x, i) => [x, items2[i]]);
}

function computeData({ elevationPoints, distancePoints }: Props): DataPoint[] {
  const zipped = zipStrict(elevationPoints, distancePoints);

  let distanceFromStart = 0;

  return zipped.map(([elevation, distance]) => {
    distanceFromStart += (distance / 1000);

    return { elevation, distanceFromStart };
  });
}

export function ElevationProfile(props: Props): React.ReactElement {
  const data = useMemo(() => computeData(props), [props]);

  return (
    <AreaChart
      width={730}
      height={250}
      data={data}
      margin={{
        top: 20,
        right: 20,
        bottom: 20,
        left: 20,
      }}
    >
      <XAxis dataKey="distanceFromStart" minTickGap={50} tickFormatter={(tick): string => `${Math.round(tick * 10) / 10}km`} />
      <YAxis domain={['dataMin', 'dataMax']} minTickGap={30} scale="linear" tickFormatter={(tick): string => `${Math.round(tick / 10) * 10}m`} />
      <Area dataKey="elevation" stroke="#8884d8" fill="#8884d8" isAnimationActive={false} />
    </AreaChart>
  );
}
