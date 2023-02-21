import { renderHook } from "@testing-library/react";

import { Provider } from "../provider";
import { useAuth } from "./use-auth";
import { TokenManager } from "../../common/token";

describe("useAuth", () => {
  it("creates a new token manager", () => {
    const { result } = renderHook(() => useAuth(), { wrapper: Provider });
    expect(result.current[0].tokenProvider).toBeInstanceOf(TokenManager);
  });
});
