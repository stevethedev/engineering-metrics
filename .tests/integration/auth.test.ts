import { apiHost, authCredentials } from "../environment";
import axios from "axios";

describe("Auth", () => {
  const sendLoginRequest = async (credentials: Readonly<object>) =>
    axios.post(`${apiHost}/auth/login`, credentials).then(
      (response) => response,
      (error) => error.response
    );

  const sendWhoAmIRequest = async (token: string) =>
    axios
      .get(`${apiHost}/auth/whoami`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      })
      .then(
        (response) => response,
        (error) => error.response
      );

  const sendLogoutRequest = async (token: string) =>
    axios
      .get(`${apiHost}/auth/logout`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      })
      .then(
        (response) => response,
        (error) => error.response
      );

  const sendRegisterRequest = async (credentials: Readonly<object>) =>
    axios.post(`${apiHost}/auth/register`, credentials).then(
      (response) => response,
      (error) => error.response
    );

  describe("HTTP POST /auth/register", () => {
    it("should succeed with valid credentials", async () => {
      const credentials = {
        username: `test${Date.now()}`,
        password: "test",
      };

      const result = await sendRegisterRequest(credentials);
      expect(result.status).toBe(201);

      const loginResult = await sendLoginRequest(credentials);
      expect(loginResult.status).toBe(200);
      expect(loginResult.data).toHaveProperty("token");
    });
  });

  describe("HTTP GET /auth/logout", () => {
    it("should succeed with valid token", async () => {
      const response = await sendLoginRequest(authCredentials);
      const token = response.data.token;

      const result = await sendLogoutRequest(token);

      expect(result.status).toBe(200);
    });

    it("should fail with invalid token", async () => {
      const result = await sendLogoutRequest("invalid-token");
      expect(result.status).toBe(401);
    });
  });

  describe("HTTP POST /auth/login", () => {
    it("should succeed with valid credentials", async () => {
      const credentials = authCredentials;

      console.log(`${apiHost}/auth/login`);

      const result = await sendLoginRequest(credentials);

      expect(result.status).toBe(200);
      expect(result.data).toHaveProperty("token");
    });

    it("should fail with invalid credentials", async () => {
      const credentials = {
        ...authCredentials,
        password: authCredentials.password + "invalid",
      };

      const result = await sendLoginRequest(credentials);
      expect(result.status).toBe(401);
    });

    it("should reject with missing credentials", async () => {
      const { password, ...credentials } = authCredentials;

      const result = await sendLoginRequest(credentials);
      expect(result.status).toBe(400);
    });

    it("should fail with empty credentials", async () => {
      const credentials = {};

      const result = await sendLoginRequest(credentials);
      expect(result.status).toBe(400);
    });
  });

  describe("HTTP GET /auth/whoami", () => {
    it("should succeed with valid token", async () => {
      const response = await sendLoginRequest(authCredentials);
      const token = response.data.token;

      const result = await sendWhoAmIRequest(token);

      expect(result.status).toBe(200);
      expect(result.data).toHaveProperty("username");
      expect(result.data.username).toBe(authCredentials.username);
    });

    it("should fail with invalid token", async () => {
      const result = await sendWhoAmIRequest("invalid-token");
      expect(result.status).toBe(401);
    });
  });
});
