export function getApiBaseUrl(): string {
  return (window as any).__ENV__.API_BASE_URL ?? "https://api.howittplains.net";
}
