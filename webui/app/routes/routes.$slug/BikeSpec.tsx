import { uniq } from "es-toolkit";
import type { BikeSpec } from "~/__generated__/schema-types";
import { DataTable } from "~/components/DataTable";
import { isNotNil } from "~/services/isNotNil";

function formatTyreWidth(mm: number): string {
  if (mm <= 50) {
    return [mm, "mm"].join("");
  }
  return [Math.round((mm / 25.4) * 100) / 100, '"'].join("");
}

function formatTyreWidths(widths?: number[]): string {
  return uniq(widths ?? [])
    .map((width) => formatTyreWidth(width))
    .join(" ~ ");
}

function formatTravel(mm: number): string {
  if (mm === 0) {
    return "rigid";
  }
  return [mm, "mm"].join("");
}

function formatTravels(travels?: number[]): string {
  return uniq(travels ?? [])
    .map((travel) => formatTravel(travel))
    .join(" ~ ");
}

function isRigid(travels?: number[]): boolean {
  return (travels ?? []).every((travel) => travel === 0);
}

interface Props {
  title: string;
  bikeSpec: BikeSpec;
}

export function BikeSpecContent({ title, bikeSpec }: Props): React.ReactNode {
  const tableItems = [
    { name: "Tyre Width", value: formatTyreWidths(bikeSpec.tyreWidth) },
    isRigid(bikeSpec.frontSuspension)
      ? undefined
      : {
          name: "Front Suspension",
          value: formatTravels(bikeSpec.frontSuspension),
        },
    isRigid(bikeSpec.rearSuspension)
      ? undefined
      : {
          name: "Rear Suspension",
          value: formatTravels(bikeSpec.rearSuspension),
        },
  ].filter((item) => isNotNil(item));

  return <DataTable title={title} items={tableItems} />;
}
