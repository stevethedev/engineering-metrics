import { Requester, RequesterOptions } from "../../common";
import { apiAuthLogoutUrl } from "../../../environment";

/**
 * Logout API
 */
export interface LogoutController {
  logout: (token?: string) => Promise<void>;
}

/**
 * Options for the logout API.
 */
export interface LogoutApiOptions {
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
export class LogoutApi implements LogoutController {
  /** Base URL for sending requests to the API. */
  readonly #url: string;

  /** Options for the underlying request client. */
  readonly #request: Requester;

  /**
   * Creates a new instance of the logout API.
   */
  constructor(options?: Readonly<LogoutApiOptions>) {
    this.#url = options?.url ?? apiAuthLogoutUrl;
    this.#request = new Requester(options?.requesterOptions);
  }

  /**
   * Attempts to logout the current user.
   *
   * @param token The token to use for authentication.
   */
  async logout(token?: string): Promise<void> {
    const headers = token ? { Authorization: token } : undefined;
    await this.#request.get(this.#url, { headers });
  }
}
