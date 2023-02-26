const path = require("path");

const ROOT_DIR = path.resolve(__dirname, "..", "..");
const CLIENT_DIR = path.resolve(ROOT_DIR, "client");
const SERVER_DIR = path.resolve(ROOT_DIR, "server");
const JSON_SCHEMA_DIR = path.resolve(ROOT_DIR, "json-schema");
const TEST_DIR = path.resolve(ROOT_DIR, ".tests");

module.exports = {
  ROOT_DIR,
  CLIENT_DIR,
  SERVER_DIR,
  JSON_SCHEMA_DIR,
  TEST_DIR,
};
