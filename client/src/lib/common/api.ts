export type WindowFetch = typeof window.fetch;

export class Requester {
  readonly #fetch: WindowFetch;

  constructor(fetch: WindowFetch = window.fetch) {
    this.#fetch = (...args) => fetch(...args);
  }

  async post(url: string, options: RequestInit): Promise<Response> {
    return this.#fetch(url, {
      ...options,
      method: "POST",
    });
  }
}

export const joinPaths = (baseUrl: string, path: string): string => {
  const isBaseSlashed = baseUrl.endsWith("/");
  const isPathSlashed = path.startsWith("/");

  if (isBaseSlashed && isPathSlashed) {
    return `${baseUrl.slice(0, -1)}${path}`;
  }

  if (!isBaseSlashed && !isPathSlashed) {
    return `${baseUrl}/${path}`;
  }

  return `${baseUrl}${path}`;
};
