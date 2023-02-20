import { act, renderHook } from "@testing-library/react";
import { useToken } from "./use-token";
import { Provider } from "../provider";

describe("useToken", () => {
  it("can update the token", () => {
    const token = "test";
    const { result } = renderHook(() => useToken(), { wrapper: Provider });

    expect(result.current[0]).toEqual(null);

    act(() => result.current[1](token));

    expect(result.current[0]).toEqual(token);
  });
});
