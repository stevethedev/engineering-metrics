const { DOCKER_EXECUTABLE, run } = require("./lib/exec");
const { ROOT_DIR } = require("./lib/dir");

const stopDocker = async () =>
  run(DOCKER_EXECUTABLE, ["compose", "down"], ROOT_DIR);

const main = async (argv = process.argv) => {
  const isHelp = argv.includes("--help");

  if (isHelp) {
    process.stdout.write("Usage:\n");
    process.stdout.write("  stop\n");
    return;
  }

  const passed = await stopDocker();
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
