import { uniq } from "lodash";
import type { BikeSpec } from "~/__generated__/graphql";

function formatTyreWidth(mm: number): string {
  if (mm <= 50) {
    return [mm, "mm"].join("");
  }
  return [Math.round((mm / 25.4) * 100) / 100, '"'].join("");
}

function formatTyreWidths(widths?: number[]): string {
  return uniq(widths).map(formatTyreWidth).join(" ~ ");
}

function formatTravel(mm: number): string {
  if (mm === 0) {
    return "rigid";
  }
  return [mm, "mm"].join("");
}

function formatTravels(travels?: number[]): string {
  return uniq(travels).map(formatTravel).join(" ~ ");
}

interface Props {
  title: string;
  bikeSpec: BikeSpec;
}

export function BikeSpecContent({ title, bikeSpec }: Props) {
  return (
    <div>
      <h3>{title}</h3>
      <dl>
        <dt>Tyre Width</dt>
        <dd>{formatTyreWidths(bikeSpec.tyreWidth)}</dd>
        <dt>Front Suspension</dt>
        <dd>{formatTravels(bikeSpec.frontSuspension)}</dd>
        <dt>Rear Suspension</dt>
        <dd>{formatTravels(bikeSpec.rearSuspension)}</dd>
      </dl>
    </div>
  );
}
