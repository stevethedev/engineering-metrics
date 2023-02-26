import {
  apiRefreshExpiresKey,
  apiRefreshKey,
  apiTokenExpiresKey,
  apiTokenKey,
} from "../../environment";

/**
 * TokenProvider is a simple interface for managing the API token.
 */
export interface TokenProvider {
  /**
   * The current token.
   */
  readonly token: string | null;

  readonly refresh: string | null;

  /**
   * Sets the token.
   * @param token - The token to set.
   * @param expires - The expiration time of the token, in milliseconds.
   */
  setToken(token: string | null, expires: number | null): void;

  /**
   * Sets the refresh token.
   * @param refresh - The refresh token to set.
   * @param expires - The expiration time of the refresh token, in milliseconds.
   */
  setRefresh(refresh: string | null, expires: number | null): void;

  /**
   * Clears the tokens.
   */
  clearToken(): void;

  /**
   * Refreshes the token.
   */
  refreshToken(): Promise<void>;
}

export interface TokenManagerOptions {
  authToken?: string | null;
  refreshToken?: string | null;
  authTokenExpires?: number | null;
  refreshTokenExpires?: number | null;
  refreshNow?: boolean;
}

/**
 * TokenManager is a simple implementation of the TokenProvider interface.
 */
export class TokenManager implements TokenProvider {
  #token: string | null = null;

  #refresh: string | null = null;

  #tokenExpires: number | null = null;

  #refreshExpires: number | null = null;

  #tokenTimer: NodeJS.Timeout | null = null;

  /**
   * Creates a new token manager.
   * @param options Initial configuration options.
   */
  constructor(options: TokenManagerOptions = {}) {
    const {
      authToken = null,
      refreshToken = null,
      authTokenExpires = null,
      refreshTokenExpires = null,
      refreshNow = false,
    } = options;

    this.setToken(authToken, authTokenExpires);
    this.setRefresh(refreshToken, refreshTokenExpires);

    if (refreshNow) {
      void this.refreshToken();
    }
  }

  get refresh(): string | null {
    if (this.#refresh !== null) {
      return this.#refresh;
    }

    const token = localStorage.getItem(apiRefreshKey);
    if (token !== null && token !== "") {
      return token;
    }

    return token;
  }

  /**
   * The current token.
   */
  get token(): string | null {
    if (this.#token !== null) {
      return this.#token;
    }

    const token = localStorage.getItem(apiTokenKey);
    if (token !== null && token !== "") {
      return token;
    }

    return token;
  }

  /**
   * Sets the token.
   * @param token The token to set.
   * @param expires The expiration time of the token, in milliseconds.
   */
  setToken(token: string | null, expires: number | null) {
    this.#token = null;
    this.#tokenExpires = null;

    localStorage.setItem(apiTokenKey, token ?? "");
    localStorage.setItem(apiTokenExpiresKey, expires?.toString() ?? "");

    if (this.#tokenTimer) {
      clearTimeout(this.#tokenTimer);
      this.#tokenTimer = null;
    }

    if (expires) {
      this.#tokenTimer = setTimeout(() => {
        this.clearToken();
      }, expires - Date.now());
    }
  }

  /**
   * The current refresh token.
   * @param refresh
   * @param expires
   */
  setRefresh(refresh: string | null, expires: number | null) {
    this.#refresh = null;
    this.#refreshExpires = null;

    localStorage.setItem(apiRefreshKey, refresh ?? "");
    localStorage.setItem(apiRefreshExpiresKey, expires?.toString() ?? "");
  }

  refreshToken(): Promise<void> {
    // TODO: Implement this
    return Promise.reject(new Error("Not implemented"));
  }

  /**
   * Clears the token.
   */
  clearToken() {
    this.#token = null;
    this.#tokenExpires = null;
    this.#refresh = null;
    this.#refreshExpires = null;

    if (this.#tokenTimer) {
      clearTimeout(this.#tokenTimer);
      this.#tokenTimer = null;
    }

    localStorage.removeItem(apiTokenKey);
    localStorage.removeItem(apiTokenExpiresKey);
    localStorage.removeItem(apiRefreshKey);
    localStorage.removeItem(apiRefreshExpiresKey);
  }
}
