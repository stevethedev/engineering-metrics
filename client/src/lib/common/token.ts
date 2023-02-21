import { apiTokenKey } from "../../environment";

/**
 * TokenProvider is a simple interface for managing the API token.
 */
export interface TokenProvider {
  /**
   * The current token.
   */
  readonly token: string | null;

  /**
   * Sets the token.
   * @param token
   */
  setToken(token: string | null): void;

  /**
   * Clears the token.
   */
  clearToken(): void;
}

/**
 * TokenManager is a simple implementation of the TokenProvider interface.
 */
export class TokenManager implements TokenProvider {
  #token: string | null = null;

  /**
   * Creates a new token manager.
   * @param token The initial token to use. If not provided, the token will be
   * loaded from local storage.
   */
  constructor(token: string | null = null) {
    this.#token = token;
  }

  /**
   * The current token.
   */
  get token(): string | null {
    return this.#token ?? localStorage.getItem(apiTokenKey) ?? null;
  }

  /**
   * Sets the token.
   * @param token The token to set.
   */
  setToken(token: string | null) {
    this.#token = null;
    localStorage.setItem(apiTokenKey, token ?? "");
  }

  /**
   * Clears the token.
   */
  clearToken() {
    this.#token = null;
    localStorage.removeItem(apiTokenKey);
  }
}
