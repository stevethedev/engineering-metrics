import { act, renderHook } from "@testing-library/react";

import { Provider } from "../provider";
import { useAuth } from "./use-auth";

describe("useAuth", () => {
  it("defaults to an empty token", () => {
    const { result } = renderHook(() => useAuth(), { wrapper: Provider });
    expect(result.current[0]).toEqual({ token: null });
  });

  it("can set a default auth state", () => {
    const data = { token: "test" };
    const { result } = renderHook(() => useAuth(), {
      wrapper: ({ children }) => <Provider data={data}>{children}</Provider>,
    });
    expect(result.current[0]).toEqual(data);
  });

  it("can set a new auth state", () => {
    const data = { token: "test" };
    const { result } = renderHook(() => useAuth(), { wrapper: Provider });

    expect(result.current[0]).toEqual({ token: null });

    act(() => result.current[1](data));

    expect(result.current[0]).toEqual(data);
  });
});
