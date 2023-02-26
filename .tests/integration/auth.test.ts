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

  const sendRefreshRequest = async (refreshToken: string) =>
    axios.post(`${apiHost}/auth/refresh`, { refreshToken }).then(
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
      expect(loginResult.data).toMatchObject({
        authToken: expect.any(String),
        authTokenExpires: expect.any(Number),
        refreshToken: expect.any(String),
        refreshTokenExpires: expect.any(Number),
      });
    });
  });

  describe("HTTP GET /auth/logout", () => {
    it("should succeed with valid token", async () => {
      const response = await sendLoginRequest(authCredentials);
      const token = response.data.authToken;

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
      expect(result.data).toMatchObject({
        authToken: expect.any(String),
        authTokenExpires: expect.any(Number),
        refreshToken: expect.any(String),
        refreshTokenExpires: expect.any(Number),
      });
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
      const token = response.data.authToken;

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

  describe("HTTP POST /auth/refresh", () => {
    it("should succeed with valid refresh token", async () => {
      const response = await sendLoginRequest(authCredentials);
      const refreshToken = response.data.refreshToken;

      const result = await sendRefreshRequest(refreshToken);

      expect(result.status).toBe(200);
      expect(result.data).toMatchObject({
        authToken: expect.any(String),
        authTokenExpires: expect.any(Number),
        refreshToken: expect.any(String),
        refreshTokenExpires: expect.any(Number),
      });
    });

    it("should fail with invalid refresh token", async () => {
      const result = await sendRefreshRequest("invalid-token");
      expect(result.status).toBe(401);
    });

    it("should invalidate the old token after refresh", async () => {
      const response = await sendLoginRequest(authCredentials);
      const refreshToken = response.data.refreshToken;

      const result = await sendRefreshRequest(refreshToken);

      expect(result.status).toBe(200);
      expect(result.data).toMatchObject({
        authToken: expect.any(String),
        authTokenExpires: expect.any(Number),
        refreshToken: expect.any(String),
        refreshTokenExpires: expect.any(Number),
      });

      const oldToken = response.data.authToken;
      const whoAmIResult = await sendWhoAmIRequest(oldToken);
      expect(whoAmIResult.status).toBe(401);
    });

    it("should invalidate the old refresh token after refresh", async () => {
      const response = await sendLoginRequest(authCredentials);
      const refreshToken = response.data.refreshToken;

      const result = await sendRefreshRequest(refreshToken);

      expect(result.status).toBe(200);
      expect(result.data).toMatchObject({
        authToken: expect.any(String),
        authTokenExpires: expect.any(Number),
        refreshToken: expect.any(String),
        refreshTokenExpires: expect.any(Number),
      });

      const oldRefreshToken = response.data.refreshToken;
      const refreshResult = await sendRefreshRequest(oldRefreshToken);
      expect(refreshResult.status).toBe(401);
    });
  });
});
