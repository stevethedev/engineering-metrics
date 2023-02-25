import { act, renderHook } from "@testing-library/react";
import { useToken } from "./use-token";
import { Provider } from "../provider";

describe("useToken", () => {
  it("can update the token", () => {
    const token = "some-other-token";
    const tokenExpires = Date.now() + 1e6;
    const tokenProvider = {
      token: "test-token",
      refresh: null,
      refreshToken: jest.fn(),
      setRefresh: jest.fn(),
      setToken: jest.fn(),
      clearToken: jest.fn(),
    };

    const { result } = renderHook(() => useToken(tokenProvider), {
      wrapper: Provider,
    });

    expect(result.current[0]).toBe(tokenProvider.token);

    act(() => result.current[1](token, tokenExpires));

    expect(tokenProvider.setToken).toHaveBeenCalledWith(token, tokenExpires);
  });
});
