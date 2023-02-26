export const webHost = process.env.WEB_HOST || "http://localhost";
export const apiHost = process.env.API_HOST || "http://localhost/api";
export const clientHost = process.env.CLIENT_HOST || "http://localhost/client";

export const authCredentials: Readonly<{ username: string; password: string }> =
  JSON.parse(
    process.env.AUTH_CREDENTIALS || '{"username": "admin", "password": "admin"}'
  );
