declare global {
  interface Window {
    __ENV__: { API_BASE_URL: string };
  }
}

export function getApiBaseUrl(): string {
  return window.__ENV__.API_BASE_URL;
}
