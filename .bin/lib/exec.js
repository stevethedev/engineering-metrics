const { spawnSync } = require("child_process");

const NODE_EXECUTABLE = process.argv0;
const NPM_EXECUTABLE = process.platform === "win32" ? "npm.cmd" : "npm";
const CARGO_EXECUTABLE = "cargo";
const DOCKER_EXECUTABLE = "docker";
const SEA_ORM_CLI = "sea-orm-cli";

const run = async (command, args, cwd = process.cwd()) => {
  try {
    const proc = spawnSync(command, args, {
      cwd,
      stdio: "inherit",
    });
    return proc.status === 0;
  } catch (err) {
    console.error(err);
    return false;
  }
};

module.exports = {
  NODE_EXECUTABLE,
  NPM_EXECUTABLE,
  CARGO_EXECUTABLE,
  DOCKER_EXECUTABLE,
  SEA_ORM_CLI,
  run,
};
