import { TokenManager, TokenProvider } from "./token";

/**
 * A type definition for the window.fetch function.
 */
export type WindowFetch = typeof window.fetch;

/**
 * Options for the request client.
 */
export interface RequesterOptions {
  /**
   * The window.fetch function to use. If not provided, the window.fetch
   * function will be used.
   */
  fetch?: WindowFetch;

  /**
   * The token provider to use. If not provided, a new token manager will be
   * used.
   */
  tokenProvider?: TokenProvider;
}

/**
 * A request client.
 */
export class Requester {
  readonly #fetch: WindowFetch;

  readonly #tokenProvider: TokenProvider;

  constructor(options: Readonly<RequesterOptions> = {}) {
    const fetch = options.fetch ?? window.fetch;
    const tokenProvider = options.tokenProvider ?? new TokenManager();

    this.#fetch = (...args) => fetch(...args);
    this.#tokenProvider = tokenProvider;
  }

  get #token(): string | null {
    return this.#tokenProvider.token;
  }

  /**
   * The authorization header to use for requests.
   */
  get authHeader(): Record<string, string> {
    return this.#token ? { Authorization: `Bearer ${this.#token}` } : {};
  }

  /**
   * Sends a POST request to the specified URL.
   * @param url The URL to send the request to.
   * @param body The body of the request.
   * @param options Options for the request.
   * @returns The response from the server.
   */
  async post<Body>(
    url: string,
    body: Body,
    options?: RequestInit
  ): Promise<Response> {
    return this.#fetch(url, {
      ...options,
      method: "POST",
      headers: {
        "Content-Type": "application/json; charset=utf-8",
        ...this.authHeader,
        ...options?.headers,
      },
      body: JSON.stringify(body),
    });
  }

  /**
   * Sends a GET request to the specified URL.
   * @param url The URL to send the request to.
   * @param options Options for the request.
   */
  async get(url: string, options?: RequestInit): Promise<Response> {
    return this.#fetch(url, {
      ...options,
      method: "GET",
      headers: {
        ...this.authHeader,
        ...options?.headers,
      },
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
