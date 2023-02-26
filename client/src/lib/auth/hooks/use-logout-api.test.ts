import { renderHook } from "@testing-library/react";
import { useLogoutApi } from "./use-logout-api";
import { act } from "react-dom/test-utils";
import { TokenProvider } from "../../common/token";
import { Provider } from "../provider";

describe("useLogoutApi", () => {
  it("sends logout requests", async () => {
    const url = "/api/auth/logout";
    const fetch = jest.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({}),
    });
    const tokenProvider: TokenProvider = {
      token: "test-token",
      refresh: "test-refresh",
      refreshToken: jest.fn(),
      setRefresh: jest.fn(),
      setToken: jest.fn(),
      clearToken: jest.fn(),
    };

    const { result } = renderHook(
      () => useLogoutApi({ url, requesterOptions: { fetch, tokenProvider } }),
      { wrapper: Provider }
    );
    expect(result.current).toBeDefined();

    await act(() => result.current());

    expect(fetch).toHaveBeenCalledWith(url, {
      method: "GET",
      headers: {
        Authorization: "Bearer test-token",
      },
    });
  });
});
