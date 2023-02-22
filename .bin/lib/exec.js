const { spawnSync } = require("child_process");

const NPM_EXECUTABLE = process.platform === "win32" ? "npm.cmd" : "npm";
const CARGO_EXECUTABLE = "cargo";
const DOCKER_EXECUTABLE = "docker";

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
  NPM_EXECUTABLE,
  CARGO_EXECUTABLE,
  DOCKER_EXECUTABLE,
  run,
};
