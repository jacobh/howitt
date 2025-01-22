import { Temporal } from "@js-temporal/polyfill";

export function formatDistance(meters: number): string {
  const km = Math.round(meters / 100) / 10;

  return `${km}km`;
}

export function formatVertical(meters: number): string {
  return `${Math.round(meters)}m`;
}

export function formatDuration(duration: Temporal.Duration): string {
  // Convert to total hours and minutes
  const totalHours = Math.floor(duration.total("hours"));
  const remainingMinutes = Math.floor(duration.total("minutes") % 60);

  return `${totalHours}h${remainingMinutes}m`;
}
