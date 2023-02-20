import { Requester, WindowFetch } from "../../common";
import { apiAuthLoginUrl } from "../../../environment";

/**
 * Structured representation of a login request.
 */
export interface LoginOptions {
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
  login: (options: LoginOptions) => Promise<LoginResult>;
}

/**
 * Configuration options for the login API.
 */
export interface LoginApiOptions {
  url?: string;

  fetch?: WindowFetch;
}

/**
 * Provides a structured interface for the login API.
 */
export class LoginApi implements LoginController {
  readonly #url: string;

  readonly #request: Requester;

  constructor(options?: LoginApiOptions) {
    this.#url = options?.url ?? apiAuthLoginUrl;
    this.#request = new Requester(options?.fetch);
  }

  async login(options: LoginOptions): Promise<LoginResult> {
    const response = await this.#request.post(this.#url, {
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(options),
    });

    if (response.ok) {
      return response.json();
    }

    return null;
  }
}
