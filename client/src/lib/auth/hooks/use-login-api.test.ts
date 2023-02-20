import { useLoginApi } from "./use-login-api";
import { act, renderHook } from "@testing-library/react";
import { Provider } from "../provider";

describe("useLoginApi", () => {
  it("sends a request to the login endpoint", async () => {
    const fetch = jest.fn().mockResolvedValue({
      ok: true,
      json: jest.fn().mockResolvedValue({ token: "token" }),
    });

    const { result } = renderHook(
      () => useLoginApi({ url: "/auth/login", fetch }),
      { wrapper: Provider }
    );

    const login = result.current;

    const data = act(async () =>
      login({ username: "username", password: "password" })
    );

    await expect(data).resolves.toEqual({ token: "token" });
  });
});
