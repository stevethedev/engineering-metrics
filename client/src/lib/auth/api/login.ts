import { Requester, RequesterOptions } from "../../common";
import { apiAuthLoginUrl } from "../../../environment";
import { LoginRequest } from "../../../generated/auth";
import { DeepReadonly } from "ts-essentials";

/**
 * Structured representation of a login request.
 */
export interface LoginCredentials {
  username: string;

  password: string;
}

/**
 * Successful login response.
 */
export interface LoginSuccess {
  token: string;
}

/**
 * Failed login response.
 */
export type LoginFailure = null;

export type LoginResult = LoginSuccess | LoginFailure;

/**
 * Defines the interface for a login controller.
 */
export interface LoginController {
  login: (options: DeepReadonly<LoginCredentials>) => Promise<LoginResult>;
}

/**
 * Configuration options for the login API.
 */
export interface LoginApiOptions {
  requesterOptions?: RequesterOptions;

  url?: string;
}

/**
 * Provides a structured interface for the login API.
 */
export class LoginApi implements LoginController {
  readonly #url: string;

  readonly #request: Requester;

  constructor(options?: Readonly<LoginApiOptions>) {
    this.#url = options?.url ?? apiAuthLoginUrl;
    this.#request = new Requester(options?.requesterOptions);
  }

  /**
   * Attempts to authenticate with the given credentials.
   * @param credentials
   * @returns The authentication token if successful, null otherwise.
   */
  async login(
    credentials: DeepReadonly<LoginCredentials>
  ): Promise<LoginResult> {
    const response = await this.#request.post<LoginRequest>(
      this.#url,
      credentials
    );

    if (response.ok) {
      return (await response.json()) as LoginResult;
    }

    return null;
  }
}
