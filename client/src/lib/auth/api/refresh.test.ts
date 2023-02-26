import { RefreshApi } from "./refresh";
import { RefreshResponseSuccess } from "../../../generated/auth";

describe("RefreshApi", () => {
  it("sends a POST request to the refresh API", async () => {
    const response: RefreshResponseSuccess = {
      authToken: "token",
      authTokenExpires: ((Date.now() + 1e6) / 1000) | 0,
      refreshToken: "refresh-token-2",
      refreshTokenExpires: ((Date.now() + 1e7) / 1000) | 0,
    };
    const fetch = jest.fn().mockResolvedValue({
      ok: true,
      json: jest.fn().mockResolvedValue(response),
    });

    const refreshApi = new RefreshApi({
      url: "/auth/refresh",
      requesterOptions: { fetch },
    });

    const result = refreshApi.refresh("refresh-token");

    await expect(result).resolves.toEqual({
      token: response.authToken,
      tokenExpires: response.authTokenExpires * 1000,
      refresh: response.refreshToken,
      refreshExpires: response.refreshTokenExpires * 1000,
    });
    expect(fetch).toHaveBeenCalledWith(
      "/auth/refresh",
      expect.objectContaining({
        method: "POST",
      })
    );
  });

  it("returns null if the response is not ok", async () => {
    const fetch = jest.fn().mockResolvedValue({
      ok: false,
    });

    const refreshApi = new RefreshApi({
      url: "/auth/refresh",
      requesterOptions: { fetch },
    });

    const result = refreshApi.refresh("refresh-token");

    await expect(result).resolves.toEqual(null);
  });
});
