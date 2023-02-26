const { DOCKER_EXECUTABLE, run } = require("./lib/exec");
const { ROOT_DIR } = require("./lib/dir");

const startDocker = async ({ isBuild = true, isDetached = false } = {}) => {
  const args = ["compose", "up"];
  if (isBuild) {
    args.push("--build");
  }
  if (isDetached) {
    args.push("-d");
  }
  return run(DOCKER_EXECUTABLE, args, ROOT_DIR);
};

const main = async (argv = process.argv) => {
  const isHelp = argv.includes("--help");

  if (isHelp) {
    process.stdout.write("Usage:\n");
    process.stdout.write("  start [--no-build] [--detached]\n");
    return;
  }

  const isBuild = !argv.includes("--no-build");
  const isDetached = argv.includes("--detached");

  const passed = await startDocker({ isBuild, isDetached });
  if (!passed) {
    process.stderr.write("Docker failed\n");
    process.exit(1);
  }

  process.stdout.write("Docker started\n");
};

module.exports = main;

if (require.main === module) {
  main().catch((err) => {
    console.error(err);
    process.exit(1);
  });
}
