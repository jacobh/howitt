import { css } from "@emotion/react";
import { uniq } from "lodash";
import type { BikeSpec } from "~/__generated__/graphql";
import { DataTable } from "~/components/DataTable";

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

const bikeSpecContentCss = css`
  margin: 20px 0;
`;

export function BikeSpecContent({ title, bikeSpec }: Props): JSX.Element {
  const tableItems = [
    { name: "Tyre Width", value: formatTyreWidths(bikeSpec.tyreWidth) },
    {
      name: "Front Suspension",
      value: formatTravels(bikeSpec.frontSuspension),
    },
    { name: "Rear Suspension", value: formatTravels(bikeSpec.rearSuspension) },
  ];

  return (
    <div css={bikeSpecContentCss}>
      <DataTable title={title} items={tableItems} />
    </div>
  );
}
