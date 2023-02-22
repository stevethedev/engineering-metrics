const path = require("path");

const ROOT_DIR = path.resolve(__dirname, "..");
const CLIENT_DIR = path.resolve(ROOT_DIR, "client");
const SERVER_DIR = path.resolve(ROOT_DIR, "server");

module.exports = {
  ROOT_DIR,
  CLIENT_DIR,
  SERVER_DIR,
};
