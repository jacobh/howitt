declare global {
  interface Window {
    __ENV__: { API_BASE_URL: string };
  }
}

export function getApiBaseUrl(): string {
  return (globalThis as typeof globalThis & Window).__ENV__.API_BASE_URL;
}
