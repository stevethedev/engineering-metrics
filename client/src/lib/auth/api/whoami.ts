import { Requester, RequesterOptions } from "../../common";
import { apiAuthWhoAmIUrl } from "../../../environment";

/**
 * The result of a successful whoami request.
 */
export interface WhoAmISuccess {
  id: string;
  username: string;
}

export type WhoAmIFailure = null;

export type WhoAmIResult = WhoAmISuccess | WhoAmIFailure;

/**
 * Controller for the whoami API.
 */
export interface WhoAmIController {
  /**
   * Attempts to check the current user.
   * @param token
   * @returns
   *  The current user if the request was successful, or null if the request
   *  failed.
   */
  whoAmI: (token: string) => Promise<WhoAmIResult>;
}

/**
 * Options for the whoami API.
 */
export interface WhoAmIApiOptions {
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
 * Default implementation of the whoami API.
 */
export class WhoAmIApi implements WhoAmIController {
  /** Base URL for sending requests to the API. */
  readonly #url: string;

  /** Options for the underlying request client. */
  readonly #request: Requester;

  /**
   * Creates a new instance of the whoami API.
   * @param options Options for the whoami API.
   */
  constructor(options?: Readonly<WhoAmIApiOptions>) {
    this.#url = options?.url ?? apiAuthWhoAmIUrl;
    this.#request = new Requester(options?.requesterOptions);
  }

  /**
   * Attempts to check the current user.
   */
  async whoAmI(token: string | null = null): Promise<WhoAmIResult> {
    const headers = token ? { Authorization: `Bearer ${token}` } : undefined;
    const response = await this.#request.get(this.#url, { headers });

    if (response.ok) {
      return (await response.json()) as WhoAmISuccess;
    }

    return null;
  }
}
