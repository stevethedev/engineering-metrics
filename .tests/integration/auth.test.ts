import { apiHost, authCredentials } from "../environment";
import axios from "axios";

describe("Auth", () => {
  describe("HTTP POST /auth/login", () => {
    const sendRequest = async (credentials: Readonly<object>) =>
      axios.post(`${apiHost}/auth/login`, credentials)
        .then((response) => response, (error) => error.response);

    it("should succeed with valid credentials", async () => {
      const credentials = authCredentials;

      console.log(`${apiHost}/auth/login`);

      const result = await sendRequest(credentials);

      expect(result.status).toBe(200);
      expect(result.data).toHaveProperty("token");
    });

    it("should fail with invalid credentials", async () => {
      const credentials = {
        ...authCredentials,
        password: authCredentials.password + "invalid",
      };

      const result = await sendRequest(credentials);
      expect(result.status).toBe(401);
    });

    it("should reject with missing credentials", async () => {
      const { password, ...credentials } = authCredentials;

      const result = await sendRequest(credentials);
      expect(result.status).toBe(400);
    });

    it("should fail with empty credentials", async () => {
      const credentials = {};

      const result = await sendRequest(credentials);
      expect(result.status).toBe(400);
    });
  });
});
