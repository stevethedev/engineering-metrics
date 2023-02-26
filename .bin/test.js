const { NPM_EXECUTABLE, CARGO_EXECUTABLE, run } = require("./lib/exec");
const { SERVER_DIR, CLIENT_DIR, TEST_DIR } = require("./lib/dir");
const startDocker = require("./start");
const stopDocker = require("./stop");

const testClient = async () => run(NPM_EXECUTABLE, ["run", "test"], CLIENT_DIR);

const testServer = async () => run(CARGO_EXECUTABLE, ["test"], SERVER_DIR);

const testIntegration = async () => {
  await startDocker(["--detached"]);
  const result = await run(NPM_EXECUTABLE, ["run", "test"], TEST_DIR);
  await stopDocker();
  return result;
};

const test = async ({
  isClient = true,
  isServer = true,
  isIntegration = true,
} = {}) => {
  const isClientPassed = !isClient || (await testClient());
  const isServerPassed = !isServer || (await testServer());
  const isIntegrationPassed = !isIntegration || (await testIntegration());
  return isClientPassed && isServerPassed && isIntegrationPassed;
};

const main = async () => {
  const isHelp = process.argv.includes("--help");

  if (isHelp) {
    process.stdout.write("Usage:\n");
    process.stdout.write("  test [--all]\n");
    process.stdout.write("  test [--client] [--server] [--integration]\n");
    return;
  }

  const isAll = process.argv.includes("--all");
  const isClient = isAll || process.argv.includes("--client");
  const isServer = isAll || process.argv.includes("--server");
  const isIntegration = isAll || process.argv.includes("--integration");

  const passed = await test({ isClient, isServer, isIntegration });
  if (!passed) {
    process.stderr.write("Tests failed\n");
    process.exit(1);
  }

  process.stdout.write("Tests passed\n");
};

module.exports = main;

if (require.main === module) {
  main().catch((err) => {
    console.error(err);
    process.exit(1);
  });
}
