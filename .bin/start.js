const { DOCKER_EXECUTABLE, run } = require("./lib/exec");
const { ROOT_DIR } = require("./lib/dir");

const startDocker = async ({ isBuild = true } = {}) => {
  const args = ["compose", "up"];
  if (isBuild) {
    args.push("--build");
  }
  return run(DOCKER_EXECUTABLE, args, ROOT_DIR);
};

const main = async () => {
  const isHelp = process.argv.includes("--help");

  if (isHelp) {
    process.stdout.write("Usage:\n");
    process.stdout.write("  start [--no-build]\n");
    return;
  }

  const isBuild = !process.argv.includes("--no-build");

  const passed = await startDocker({ isBuild });
  if (!passed) {
    process.stderr.write("Docker failed\n");
    process.exit(1);
  }
};

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
