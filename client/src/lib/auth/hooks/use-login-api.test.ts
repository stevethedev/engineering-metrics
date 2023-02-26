import { useLoginApi } from "./use-login-api";
import { act, renderHook } from "@testing-library/react";
import { Provider } from "../provider";
import { LoginResponseSuccess } from "../../../generated/auth";

describe("useLoginApi", () => {
  it("sends a request to the login endpoint", async () => {
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

    const { result } = renderHook(
      () => useLoginApi({ url: "/auth/login", requesterOptions: { fetch } }),
      { wrapper: Provider }
    );

    const login = result.current;

    const data = act(async () =>
      login({ username: "username", password: "password" })
    );

    await expect(data).resolves.toEqual({
      token: "token",
      refresh: "refresh-token-2",
      tokenExpires: response.authTokenExpires * 1000,
      refreshExpires: response.refreshTokenExpires * 1000,
    });
  });
});
