import { renderHook } from "@testing-library/react";
import { useWhoAmI } from "./use-whoami";
import { Provider } from "../provider";
import { TokenProvider } from "../../common/token";
import { apiAuthWhoAmIUrl } from "../../../environment";

describe("useWhoAmI", () => {
  beforeEach(() => {
    jest.resetAllMocks();
    jest.spyOn(console, "error").mockImplementation(() => {
      // do nothing
    });
  });

  it("dispatches a request to the whoami endpoint", () => {
    const fetch = jest.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ id: "uuid", username: "test" }),
    });
    const tokenProvider: TokenProvider = {
      token: "test",
      clearToken: jest.fn(),
      setToken: jest.fn(),
    };

    renderHook(
      () =>
        useWhoAmI({
          requesterOptions: {
            fetch,
            tokenProvider,
          },
        }),
      {
        wrapper: Provider,
      }
    );

    expect(fetch).toHaveBeenCalledWith(apiAuthWhoAmIUrl, {
      method: "GET",
      headers: {
        Authorization: "Bearer test",
      },
    });
  });
});
