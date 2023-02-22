import { LogoutApi } from "./logout";
import { TokenProvider } from "../../common/token";

describe("LogoutApi", () => {
  it("sends a GET request to the logout API", async () => {
    const url = "/auth/logout";

    const fetch = jest.fn().mockResolvedValue({
      ok: true,
      json: jest.fn(),
    });

    const tokenProvider: TokenProvider = {
      token: "test-token",
      setToken: jest.fn(),
      clearToken: jest.fn(),
    };

    const logoutApi = new LogoutApi({
      url,
      requesterOptions: { fetch, tokenProvider },
    });

    await logoutApi.logout();

    expect(fetch).toHaveBeenCalledWith(
      url,
      expect.objectContaining({
        method: "GET",
        headers: {
          Authorization: "Bearer test-token",
        },
      })
    );
  });
});
