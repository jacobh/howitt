import { every, uniq } from "lodash";
import type { BikeSpec } from "~/__generated__/graphql";
import { DataTable } from "~/components/DataTable";
import { isNotNil } from "~/services/isNotNil";

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

function isRigid(travels?: number[]): boolean {
  return every(travels ?? [], (t) => t === 0);
}

interface Props {
  title: string;
  bikeSpec: BikeSpec;
}

export function BikeSpecContent({ title, bikeSpec }: Props): React.ReactNode {
  const tableItems = [
    { name: "Tyre Width", value: formatTyreWidths(bikeSpec.tyreWidth) },
    !isRigid(bikeSpec.frontSuspension)
      ? {
          name: "Front Suspension",
          value: formatTravels(bikeSpec.frontSuspension),
        }
      : undefined,
    !isRigid(bikeSpec.rearSuspension)
      ? {
          name: "Rear Suspension",
          value: formatTravels(bikeSpec.rearSuspension),
        }
      : undefined,
  ].filter(isNotNil);

  return <DataTable title={title} items={tableItems} />;
}
