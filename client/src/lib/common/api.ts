export type WindowFetch = typeof window.fetch;

export class Requester {
  readonly #fetch: WindowFetch;

  constructor(fetch: WindowFetch = window.fetch) {
    this.#fetch = (...args) => fetch(...args);
  }

  async post<Body>(url: string, body: Body, options?: RequestInit): Promise<Response> {
    return this.#fetch(url, {
      ...options,
      method: "POST",
      headers: {
        "Content-Type": "application/json; charset=utf-8",
        ...options?.headers,
      },
      body: JSON.stringify(body),
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
