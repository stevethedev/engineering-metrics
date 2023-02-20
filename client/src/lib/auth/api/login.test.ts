import { LoginApi } from "./login";

describe("LoginApi", () => {
  it("sends a POST request to the login API", async () => {
    const fetch = jest.fn().mockResolvedValue({
      ok: true,
      json: jest.fn().mockResolvedValue({ token: "token" }),
    });

    const loginApi = new LoginApi({ url: "/auth/login", fetch });

    const result = loginApi.login({
      username: "username",
      password: "password",
    });

    await expect(result).resolves.toEqual({ token: "token" });
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

    const loginApi = new LoginApi({ url: "/auth/login", fetch });

    const result = loginApi.login({
      username: "username",
      password: "password",
    });

    await expect(result).resolves.toEqual(null);
  });
});
