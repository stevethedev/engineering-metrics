import { WhoAmIApi } from "./whoami";
import { TokenManager } from "../../common/token";

describe("WhoAmIApi", () => {
  it("sends a GET request to the whoami API", async () => {
    const fetch = jest.fn().mockResolvedValue({
      ok: true,
      json: jest.fn().mockResolvedValue({ username: "username" }),
    });
    const tokenProvider = new TokenManager("token");

    const whoAmIApi = new WhoAmIApi({
      url: "/auth/whoami",
      requesterOptions: {
        fetch,
        tokenProvider,
      },
    });

    const result = whoAmIApi.whoAmI();

    await expect(result).resolves.toEqual({ username: "username" });
    expect(fetch).toHaveBeenCalledWith(
      "/auth/whoami",
      expect.objectContaining({
        method: "GET",
        headers: expect.objectContaining({
          Authorization: "Bearer token",
        }) as Record<string, string>,
      })
    );
  });

  it("returns null if the response is not ok", async () => {
    const fetch = jest.fn().mockResolvedValue({
      ok: false,
    });

    const whoAmIApi = new WhoAmIApi({
      url: "/auth/whoami",
      requesterOptions: { fetch },
    });

    const result = whoAmIApi.whoAmI();

    await expect(result).resolves.toEqual(null);
  });
});
