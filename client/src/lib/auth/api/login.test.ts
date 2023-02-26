import { LoginApi } from "./login";
import { LoginResponseSuccess } from "../../../generated/auth";

describe("LoginApi", () => {
  it("sends a POST request to the login API", async () => {
    const response: LoginResponseSuccess = {
      authToken: "token",
      authTokenExpires: ((Date.now() + 1e6) / 1000) | 0,
      refreshToken: "refresh-token-2",
      refreshTokenExpires: ((Date.now() + 1e7) / 1000) | 0,
    };
    const fetch = jest.fn().mockResolvedValue({
      ok: true,
      json: jest.fn().mockResolvedValue(response),
    });

    const loginApi = new LoginApi({
      url: "/auth/login",
      requesterOptions: { fetch },
    });

    const result = loginApi.login({
      username: "username",
      password: "password",
    });

    await expect(result).resolves.toEqual({
      token: response.authToken,
      tokenExpires: response.authTokenExpires * 1000,
      refresh: response.refreshToken,
      refreshExpires: response.refreshTokenExpires * 1000,
    });
    expect(fetch).toHaveBeenCalledWith(
      "/auth/login",
      expect.objectContaining({
        method: "POST",
      })
    );
  });

  it("returns null if the response is not ok", async () => {
    const fetch = jest.fn().mockResolvedValue({
      ok: false,
    });

    const loginApi = new LoginApi({
      url: "/auth/login",
      requesterOptions: { fetch },
    });

    const result = loginApi.login({
      username: "username",
      password: "password",
    });

    await expect(result).resolves.toEqual(null);
  });
});
