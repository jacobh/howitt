export function formatDistance(meters: number): string {
  const km = Math.round(meters / 100) / 10;

  return `${km}km`;
}

export function formatVertical(meters: number): string {
  return `${Math.round(meters)}m`;
}
