import { Requester, RequesterOptions } from "../../common";
import {
  RefreshRequest,
  RefreshResponseSuccess,
} from "../../../generated/auth";
import { LoginResult } from "./login";
import { apiAuthRefreshUrl } from "../../../environment";

/**
 * Logout API
 */
export interface RefreshController {
  refresh: (refreshToken: string) => Promise<LoginResult | null>;
}

/**
 * Options for the logout API.
 */
export interface RefreshApiOptions {
  /**
   * Base URL for sending requests to the API.
   */
  url?: string;

  /**
   * Options for the underlying request client.
   */
  requesterOptions?: RequesterOptions;
}

/**
 * Default implementation of the logout API.
 */
export class RefreshApi implements RefreshController {
  /** Base URL for sending requests to the API. */
  readonly #url: string;

  /** Options for the underlying request client. */
  readonly #request: Requester;

  /**
   * Creates a new instance of the logout API.
   */
  constructor(options?: Readonly<RefreshApiOptions>) {
    this.#url = options?.url ?? apiAuthRefreshUrl;
    this.#request = new Requester(options?.requesterOptions);
  }

  /**
   * Attempts to refresh the current user's authentication.
   *
   * @param refreshToken The refresh token to use for authentication.
   */
  async refresh(refreshToken: string): Promise<LoginResult | null> {
    const response = await this.#request.post<RefreshRequest>(this.#url, {
      refreshToken,
    });

    if (response.ok) {
      const { refreshToken, refreshTokenExpires, authTokenExpires, authToken } =
        (await response.json()) as RefreshResponseSuccess;

      return {
        token: authToken,
        tokenExpires: authTokenExpires * 1000,
        refresh: refreshToken,
        refreshExpires: refreshTokenExpires * 1000,
      };
    }

    return null;
  }
}
